#!/bin/bash
set -e

# Clean up any existing instances
pkill -9 router-lab 2>/dev/null || true
sleep 1

# Start server
echo "Starting server..."
cargo run --quiet -- --backends 3 &
SERVER_PID=$!
sleep 3

# Run load test
echo "Running load test with oha..."
oha -n 300 -c 10 --no-tui http://localhost:8080/test

# Stop server and show metrics
echo -e "\nStopping server to view metrics..."
kill -INT $SERVER_PID
wait $SERVER_PID 2>&1 | tail -10
