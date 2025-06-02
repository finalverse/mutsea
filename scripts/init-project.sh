#!/bin/bash
set -e

echo "🚀 Initializing Mutsea project..."

# Initialize git repository
git init

# Add all files
git add .

# Create initial commit
git commit -m "Initial Mutsea project structure

- Complete Rust workspace with 20+ crates
- OpenSim LLUDP protocol implementation
- Asset management system
- User authentication and sessions
- Database and caching layers
- AI-ready architecture for Phase II
- Docker and CI/CD configuration
- Comprehensive documentation"

# Add GitHub remote (user should update this)
echo "📝 Don't forget to:"
echo "   1. Create repository at https://github.com/finalverse/mutsea"
echo "   2. Add remote: git remote add origin https://github.com/finalverse/mutsea.git"
echo "   3. Push: git push -u origin main"
echo ""
echo "🎉 Mutsea project initialized successfully!"
echo ""
echo "Next steps:"
echo "   1. Run: ./scripts/dev-setup.sh"
echo "   2. Edit: config/mutsea.toml"
echo "   3. Build: cargo build"
echo "   4. Start: cargo run --bin mutsea-server"