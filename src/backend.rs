use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{debug, info};

pub async fn create_backend(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;

    info!("Backend listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::spawn(async move {
            let handler = service_fn(move |req: Request<hyper::body::Incoming>| async move {
                let response_body = format!(
                    "Backend on port {}\nMethod: {}\nPath: {}\n",
                    port,
                    req.method(),
                    req.uri().path(),
                );
                Ok::<_, hyper::Error>(Response::new(Full::new(Bytes::from(response_body))))
            });

            if let Err(err) = http1::Builder::new()
                .serve_connection(io, handler)
                .await
            {
                debug!("Backend {} error: {:?}", port, err);
            }
        });
    }
}
