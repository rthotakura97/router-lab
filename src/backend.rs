use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn handle_request(
    req: Request<hyper::body::Incoming>,
    port: u16,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let response_body = format!(
        "Backend on port {}\nMethod: {}\nPath: {}\n",
        port,
        req.method(),
        req.uri().path(),
    );

    Ok(Response::new(Full::new(Bytes::from(response_body))))
}

pub async fn run_backend(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;

    println!("Backend listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(|req| handle_request(req, port)))
                .await
            {
                eprintln!("Backend {} error: {:?}", port, err);
            }
        });
    }
}
