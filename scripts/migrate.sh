#!/bin/bash
set -e

echo "🗃️  Running database migrations..."

# Check if config exists
if [ ! -f "config/mutsea.toml" ]; then
    echo "❌ Configuration file not found. Please copy config/mutsea.example.toml to config/mutsea.toml"
    exit 1
fi

# Run migrations
cargo run --bin mutsea-cli -- migrate

echo "✅ Database migrations completed!"