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
cargo run
```

The proxy will start on `http://localhost:8080` and forward requests to backend servers.

Test it:
```bash
curl http://localhost:8080/test
```

## Current Features

- HTTP/1.1 proxy server
- Concurrent request handling
- Single backend forwarding

## Dependencies

- [tokio](https://tokio.rs/) - Async runtime
- [hyper](https://hyper.rs/) - HTTP implementation
- [hyper-util](https://github.com/hyperium/hyper-util) - HTTP utilities

## License

MIT
