#!/usr/bin/env bash

set -eu -o pipefail

echo "Cleaning up previous database and configuration"

rm -rf nodes

echo "Generating testnet configuration..."

cargo run -- generate-testnet 1 --output-dir nodes

echo "Starting one node with public API on port 6000, private one on port 6020"

cargo run -- run -d nodes/db0 -c nodes/validators/0.toml --public-api-address 127.0.0.1:6000 --private-api-address 127.0.0.1:6020
