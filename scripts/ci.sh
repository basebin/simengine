#!/bin/bash

set -e  # Exit on any error

echo "Running cargo check..."
cargo check

echo "Running cargo test..."
cargo test

echo "Running cargo fmt --check..."
cargo fmt --all -- --check

echo "Running cargo clippy..."
cargo clippy -- -D warnings

echo "All checks passed!"