mod backend;
mod load_balancer;
mod proxy;

use clap::Parser;
use load_balancer::RoundRobinBalancer;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "router-lab")]
#[command(about = "A learning-focused HTTP load balancer", long_about = None)]
struct Args {
    /// Number of backend servers to spawn
    #[arg(short = 'b', long, default_value_t = 3)]
    backends: u16,

    /// Load balancing algorithm to use
    #[arg(short = 'a', long, default_value = "round-robin")]
    algorithm: String,

    /// Port for the proxy to listen on
    #[arg(short = 'p', long, default_value_t = 8080)]
    port: u16,

    /// Starting port for backends
    #[arg(long, default_value_t = 3001)]
    backend_start_port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    info!("Starting router-lab");
    info!("Algorithm: {}", args.algorithm);
    info!("Backends: {}", args.backends);

    // Spawn backends
    let backend_ports: Vec<u16> = (args.backend_start_port..args.backend_start_port + args.backends)
        .collect();

    for port in &backend_ports {
        let port = *port;
        tokio::spawn(async move {
            if let Err(e) = backend::create_backend(port).await {
                eprintln!("Backend {} error: {}", port, e);
            }
        });
    }

    // Give backends a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // Create load balancer
    let lb: Box<dyn load_balancer::LoadBalancer> = match args.algorithm.as_str() {
        "round-robin" => Box::new(RoundRobinBalancer::new(backend_ports.clone())),
        _ => {
            eprintln!("Unknown algorithm: {}", args.algorithm);
            std::process::exit(1);
        }
    };

    // Run proxy and get metrics handle
    let metrics = proxy::run_proxy(args.port, lb).await?;

    // Wait for Ctrl+C
    info!("Press Ctrl+C to stop and view metrics");
    tokio::signal::ctrl_c().await?;

    // Print final metrics
    info!("Shutting down...");
    let m = metrics.lock().await;
    m.print_stats();

    Ok(())
}
