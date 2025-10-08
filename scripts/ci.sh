#!/bin/bash
set -e

echo "Checking..."
cargo check

echo "Testing..."
cargo test

echo "Formatting..."
if ! cargo fmt --all -- --check; then
  if [ "$CI" = "true" ]; then
    echo "Formatting issues found. Run 'cargo fmt' locally."
    exit 1
  fi
  read -p "Fix formatting? (y/N) " r
  if [[ "$r" =~ ^[Yy]$ ]]; then
    cargo fmt
  else
    exit 1
  fi
fi

echo "Linting..."
cargo clippy -- -D warnings

echo "All checks passed."
