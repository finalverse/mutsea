#!/bin/bash
set -e

echo "🔨 Building Mutsea..."

# Check Rust version
echo "📋 Checking Rust version..."
rustc --version
cargo --version

# Format code
echo "🎨 Formatting code..."
cargo fmt

# Check lints
echo "🔍 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
echo "🧪 Running tests..."
cargo test --all --all-features

# Build release
echo "🚀 Building release..."
cargo build --release --all

echo "✅ Build completed successfully!"