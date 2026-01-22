# router-lab

A learning-focused HTTP load balancer implementation in Rust for exploring load balancing algorithms, failure modes, and performance characteristics.

## Overview

This project implements a load balancer from scratch to understand:
- Load balancing algorithms (round-robin, least-connections, consistent hashing, etc.)
- HTTP/1.1 vs HTTP/2 behavior and tradeoffs
- Failure handling and health checking
- Capacity management and backpressure
- Performance optimization and profiling

## Quick Start

```bash
# Run with default settings (3 backends, round-robin)
cargo run

# Run with custom configuration
cargo run -- --backends 5 --algorithm round-robin --port 8080

# View available options
cargo run -- --help
```

The proxy will start on `http://localhost:8080` and forward requests to backend servers.

Test it:
```bash
# Send some requests
for i in {1..9}; do curl http://localhost:8080/test$i; done

# Press Ctrl+C to stop and view request distribution metrics
```

## Current Features

- **HTTP/1.1 proxy server** - Forwards requests to multiple backends
- **Round-robin load balancing** - Even distribution across backends
- **Configurable backends** - Specify number of backends via CLI
- **Request distribution metrics** - Track which backends received requests
- **Concurrent request handling** - Each connection handled in separate task
- **Structured logging** - Using tracing for clean output

## CLI Options

```
Options:
  -b, --backends <BACKENDS>
          Number of backend servers to spawn [default: 3]
  -a, --algorithm <ALGORITHM>
          Load balancing algorithm to use [default: round-robin]
  -p, --port <PORT>
          Port for the proxy to listen on [default: 8080]
      --backend-start-port <BACKEND_START_PORT>
          Starting port for backends [default: 3001]
  -h, --help
          Print help
```

## Dependencies

- [tokio](https://tokio.rs/) - Async runtime
- [hyper](https://hyper.rs/) - HTTP implementation
- [hyper-util](https://github.com/hyperium/hyper-util) - HTTP utilities
- [clap](https://docs.rs/clap) - CLI argument parsing
- [tracing](https://docs.rs/tracing) - Structured logging

## License

MIT
