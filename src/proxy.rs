use crate::load_balancer::LoadBalancer;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::{debug, info};

pub struct Metrics {
    request_counts: HashMap<u16, usize>,
    total_requests: usize,
}

impl Metrics {
    fn new() -> Self {
        Self {
            request_counts: HashMap::new(),
            total_requests: 0,
        }
    }

    fn record_request(&mut self, backend_port: u16) {
        *self.request_counts.entry(backend_port).or_insert(0) += 1;
        self.total_requests += 1;
    }

    pub fn print_stats(&self) {
        println!("\n=== Request Distribution ===");
        println!("Total requests: {}", self.total_requests);

        let mut backends: Vec<_> = self.request_counts.iter().collect();
        backends.sort_by_key(|(port, _)| *port);

        for (backend, count) in backends {
            let percentage = if self.total_requests > 0 {
                (*count as f64 / self.total_requests as f64) * 100.0
            } else {
                0.0
            };
            println!("  Backend {}: {} ({:.1}%)", backend, count, percentage);
        }
        println!("============================\n");
    }
}

async fn handle_proxy_request(
    req: Request<hyper::body::Incoming>,
    lb: Arc<Mutex<Box<dyn LoadBalancer>>>,
    metrics: Arc<Mutex<Metrics>>,
) -> Result<Response<hyper::body::Incoming>, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();

    // Select backend
    let backend_port = {
        let mut lb = lb.lock().await;
        lb.select_backend()
    };

    // Record the request
    {
        let mut m = metrics.lock().await;
        m.record_request(backend_port);
    }

    // Connect to backend
    let host = "127.0.0.1";
    let backend_addr = format!("{}:{}", host, backend_port);
    let stream = tokio::net::TcpStream::connect(&backend_addr).await?;
    let io = TokioIo::new(stream);

    // Build request to backend
    let path = req.uri().path();
    let backend_uri = format!("http://{}{}", backend_addr, path);

    let backend_req = Request::builder()
        .uri(backend_uri)
        .method(req.method())
        .body(Empty::<Bytes>::new())?;

    // Send request and get response
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    // Spawn connection driver
    tokio::spawn(async move {
        if let Err(err) = conn.await {
            debug!("Connection error: {:?}", err);
        }
    });

    // Send request
    let response = sender.send_request(backend_req).await?;

    let latency = start.elapsed();
    debug!(
        backend = backend_port,
        latency_us = latency.as_micros(),
        "Request completed"
    );

    Ok(response)
}

pub async fn run_proxy(
    proxy_port: u16,
    lb: Box<dyn LoadBalancer>,
) -> Result<Arc<Mutex<Metrics>>, Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], proxy_port));
    let listener = TcpListener::bind(addr).await?;

    info!("Proxy listening on http://{}", addr);

    let lb = Arc::new(Mutex::new(lb));
    let metrics = Arc::new(Mutex::new(Metrics::new()));
    let metrics_ret = metrics.clone();

    tokio::spawn(async move {
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let io = TokioIo::new(stream);
            let lb = lb.clone();
            let metrics = metrics.clone();

            tokio::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(
                        io,
                        service_fn(move |req| {
                            let lb = lb.clone();
                            let metrics = metrics.clone();
                            handle_proxy_request(req, lb, metrics)
                        }),
                    )
                    .await
                {
                    debug!("Proxy error: {:?}", err);
                }
            });
        }
    });

    Ok(metrics_ret)
}
