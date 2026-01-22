mod backend;
mod proxy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting router-lab...\n");

    // Spawn a backend on port 3001
    let backend_port = 3001;
    tokio::spawn(async move {
        if let Err(e) = backend::run_backend(backend_port).await {
            eprintln!("Backend error: {}", e);
        }
    });

    // Give backend a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Run proxy on port 8080, forwarding to backend on 3001
    let proxy_port = 8080;
    proxy::run_proxy(proxy_port, backend_port).await?;

    Ok(())
}
