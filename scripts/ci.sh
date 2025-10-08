#!/bin/bash

set -e  # Exit on any error

echo "Running cargo check..."
cargo check

echo "Running cargo test..."
cargo test

echo "Running cargo fmt --check..."
if ! cargo fmt --all -- --check; then
    echo "Formatting issues found. Run 'cargo fmt' to fix? (y/N)"
    read -r response
    if [[ "$response" =~ ^[Yy]$ ]]; then
        echo "Running cargo fmt..."
        cargo fmt
        echo "Formatting applied. Re-checking..."
        cargo fmt --all -- --check || echo "Formatting still has issues."
    else
        echo "Skipping formatting fix."
        exit 1
    fi
fi

echo "Running cargo clippy..."
cargo clippy -- -D warnings

echo "All checks passed!"