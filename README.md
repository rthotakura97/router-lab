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

## Load Testing

We use [oha](https://github.com/hatoo/oha) for load testing and performance measurement.

### Install oha

```bash
cargo install oha
```

### Quick Test

```bash
# Start the load balancer
cargo run -- --backends 3

# In another terminal, run load test
oha -n 1000 -c 50 http://localhost:8080/test
```

### Using the test script

```bash
./scripts/test-load.sh
```

This will:
1. Start the server with 3 backends
2. Run 300 requests using oha
3. Stop the server and display request distribution metrics

### Example Results

```
Summary:
  Success rate:  100.00%
  Requests/sec:  9,180
  Average:       0.99ms

Request Distribution:
  Backend 3001: 100 (33.3%)
  Backend 3002: 100 (33.3%)
  Backend 3003: 100 (33.3%)
```

Perfect round-robin distribution!

## Dependencies

- [tokio](https://tokio.rs/) - Async runtime
- [hyper](https://hyper.rs/) - HTTP implementation
- [hyper-util](https://github.com/hyperium/hyper-util) - HTTP utilities
- [clap](https://docs.rs/clap) - CLI argument parsing
- [tracing](https://docs.rs/tracing) - Structured logging

## License

MIT
