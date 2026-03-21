#!/bin/bash

set -e

# Build the Wasm project
cargo build --target wasm32-unknown-unknown --release

# Move built files to www directory
mv -f target/wasm32-unknown-unknown/release/*.wasm . 2>/dev/null || true

# Start a simple HTTP server
basic-http-server .