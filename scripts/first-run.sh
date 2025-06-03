#!/bin/bash
set -e

echo "🚀 Running Mutsea for the first time..."

# Check if config exists
if [ ! -f "config/mutsea.toml" ]; then
    echo "📝 Creating configuration from example..."
    mkdir -p config
    cp config/mutsea.example.toml config/mutsea.toml
    echo "✅ Configuration created at config/mutsea.toml"
fi

# Create data directories
echo "📁 Creating data directories..."
mkdir -p data/assets
mkdir -p data/logs
mkdir -p data/cache

# Build the project
echo "🔨 Building Mutsea..."
cargo build --release

# Create a test user
echo "👤 Creating test user..."
echo "📝 You can create users with: cargo run --bin mutsea-cli -- user create"
echo ""

# Start the server
echo "🌟 Starting Mutsea server..."
echo "📡 Server will be available at:"
echo "   HTTP: http://localhost:8080"
echo "   LLUDP: localhost:9000"
echo ""
echo "🔥 To connect with Firestorm:"
echo "   1. Add new grid with Login URI: http://localhost:8080/"
echo "   2. Create a user: cargo run --bin mutsea-cli -- user create \"Test\" \"User\" --password \"password\""
echo "   3. Login with First Name: Test, Last Name: User, Password: password"
echo ""

cargo run --bin mutsea-server