#!/bin/bash
set -e

echo "🛠️  Setting up Mutsea development environment..."

# Install Rust if not present
if ! command -v cargo &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
fi

# Install required components
echo "🔧 Installing Rust components..."
rustup component add rustfmt clippy

# Install development tools
echo "🛠️  Installing development tools..."
cargo install cargo-watch cargo-edit cargo-audit

# Create data directories
echo "📁 Creating data directories..."
mkdir -p data/assets
mkdir -p data/logs
mkdir -p data/cache

# Set up database (PostgreSQL)
echo "🐘 Setting up PostgreSQL..."
if command -v psql &> /dev/null; then
    createdb mutsea 2>/dev/null || echo "Database might already exist"
    psql -d mutsea -c "CREATE USER mutsea WITH PASSWORD 'mutsea';" 2>/dev/null || echo "User might already exist"
    psql -d mutsea -c "GRANT ALL PRIVILEGES ON DATABASE mutsea TO mutsea;" 2>/dev/null || echo "Privileges might already be granted"
else
    echo "⚠️  PostgreSQL not found. Please install PostgreSQL and run:"
    echo "   createdb mutsea"
    echo "   psql -d mutsea -c \"CREATE USER mutsea WITH PASSWORD 'mutsea';\""
    echo "   psql -d mutsea -c \"GRANT ALL PRIVILEGES ON DATABASE mutsea TO mutsea;\""
fi

# Set up Redis (optional)
if command -v redis-server &> /dev/null; then
    echo "📦 Redis found - caching will be available"
else
    echo "⚠️  Redis not found. Install Redis for caching support:"
    echo "   Ubuntu/Debian: sudo apt install redis-server"
    echo "   macOS: brew install redis"
fi

# Copy example configuration
echo "⚙️  Setting up configuration..."
cp config/mutsea.example.toml config/mutsea.toml

echo "✅ Development environment setup complete!"
echo ""
echo "📝 Next steps:"
echo "   1. Edit config/mutsea.toml for your environment"
echo "   2. Run: cargo build"
echo "   3. Run: cargo run --bin mutsea-server"
echo ""
echo "🌐 Connect with Firestorm to: http://localhost:8080/"