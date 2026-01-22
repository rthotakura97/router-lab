# router-lab

HTTP load balancer implementation in Rust for exploring load balancing algorithms, scaling behaviors, failure modes, and perf.

## Overview

Implements a load balancer from scratch with:
- Different load balancing algorithms (round-robin, least-connections, consistent hashing, etc.)
- HTTP/1.1 vs HTTP/2 behavior and tradeoffs
- Failure handling and health checking
- Capacity management and backpressure
- Perf profiling

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

## Load Testing

Uses [oha](https://github.com/hatoo/oha) for load testing and performance measurement.

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

### Test Scripts

```bash
# Test round-robin
./scripts/test-load.sh

# Test round-robin with 100ms backend latency
./scripts/test-round-robin-latency.sh

# Test least-connections with 100ms backend latency
./scripts/test-least-conns.sh
```
