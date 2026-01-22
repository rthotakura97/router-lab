use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn handle_proxy_request(
    req: Request<hyper::body::Incoming>,
    backend_port: u16,
) -> Result<Response<hyper::body::Incoming>, Box<dyn std::error::Error + Send + Sync>> {
    // Create client to forward request to backend
    let host = "127.0.0.1";
    let backend_addr = format!("{}:{}", host, backend_port);

    // Connect to backend
    let stream = tokio::net::TcpStream::connect(&backend_addr).await?;
    let io = TokioIo::new(stream);

    // Build request to backend (for now, just GET to same path)
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
            eprintln!("Connection error: {:?}", err);
        }
    });

    // Send request
    let response = sender.send_request(backend_req).await?;

    Ok(response)
}

pub async fn run_proxy(
    proxy_port: u16,
    backend_port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], proxy_port));
    let listener = TcpListener::bind(addr).await?;

    println!("Proxy listening on http://{}", addr);
    println!("Forwarding to backend on port {}", backend_port);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(|req| handle_proxy_request(req, backend_port)),
                )
                .await
            {
                eprintln!("Proxy error: {:?}", err);
            }
        });
    }
}
