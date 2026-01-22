#!/bin/bash
set -e

pkill -9 router-lab 2>/dev/null || true
sleep 1

echo "Starting server with least-connections algorithm and 100ms backend latency..."
cargo run --quiet -- --backends 3 --algorithm least-connections --backend-latency 100 &
SERVER_PID=$!
sleep 3

echo "Running load test with oha..."
oha -n 300 -c 10 --no-tui http://localhost:8080/test

echo -e "\nStopping server to view metrics..."
kill -INT $SERVER_PID
wait $SERVER_PID 2>&1 | tail -10
