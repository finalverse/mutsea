# Mutsea - AI-Enhanced Virtual World Platform

Mutsea is a next-generation virtual world platform that combines the compatibility of OpenSimulator with cutting-edge AI capabilities. Built in Rust for performance and safety, Mutsea enables the creation of intelligent, adaptive virtual worlds.

## Features

### Phase I: OpenSim Compatibility
- **Full Firestorm Support**: 100% compatible with existing OpenSim viewers
- **LLUDP Protocol**: Complete implementation of the OpenSim network protocol
- **Asset Management**: High-performance asset storage with multiple backends (Local, S3, Azure, GCP)
- **User Management**: Secure authentication and session management
- **Region Management**: Scalable region hosting and management
- **Database Support**: PostgreSQL, MySQL, and SQLite support

### Phase II: AI Enhancement (Coming Soon)
- **AI Content Generation**: Text-to-3D object creation
- **Social Intelligence**: AI-powered social interactions and community management
- **Natural Language Interface**: Voice and text commands for world manipulation
- **Adaptive Environments**: Worlds that learn and adapt to user behavior

### Phase III: MapleAI Integration (Planned)
- **Multi-Agent Consensus**: AI agents collaborate to resolve conflicts
- **Collective Intelligence**: Multiple AI systems working together
- **Democratic Decision Making**: AI-mediated conflict resolution

## Quick Start

### Prerequisites
- Rust 1.75 or later
- PostgreSQL 13 or later (or MySQL/SQLite)
- Redis (optional, for caching)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/finalverse/mutsea.git
cd mutsea
```

2. Build the project:
```bash
cargo build --release
```

3. Configure the database:
```bash
# Copy example configuration
cp config/mutsea.example.toml config/mutsea.toml

# Edit configuration file
nano config/mutsea.toml

# Run database migrations
cargo run --bin mutsea-cli -- migrate
# This will also initialize AI-specific tables for advanced features
```

4. Start the server:
```bash
cargo run --bin mutsea-server
```

### Connecting with Firestorm

1. Download and install [Firestorm Viewer](https://www.firestormviewer.org/)
2. Add Mutsea grid:
   - Grid Manager → Add Grid
   - Grid Name: "Mutsea"
   - Login URI: `http://your-server:8080/`
3. Create an account:
```bash
cargo run --bin mutsea-cli -- create-user "First Name" "Last Name" "email@example.com"
```

## Architecture

### Core Components

- **mutsea-core**: Core types, traits, and utilities
- **mutsea-network**: Network protocols and communication
- **mutsea-protocol**: LLUDP and HTTP protocol implementation
- **mutsea-assets**: Asset management and storage
- **mutsea-users**: User authentication and management
- **mutsea-regions**: Region hosting and management
- **mutsea-database**: Database abstraction layer

### Server Applications

- **mutsea-server**: Main server application
- **mutsea-grid-server**: Grid services server
- **mutsea-asset-server**: Dedicated asset server
- **mutsea-user-server**: User management server

### Tools and Utilities

- **mutsea-cli**: Command-line interface for administration
- **mutsea-tools**: Development and debugging tools

## Configuration

Mutsea uses TOML configuration files. See `config/mutsea.example.toml` for a complete example.

### Key Configuration Sections

- `[server]`: Server binding and performance settings
- `[database]`: Database connection and pooling
- `[cache]`: Redis cache configuration
- `[network]`: LLUDP and HTTP protocol settings
- `[assets]`: Asset storage backend configuration
- `[opensim]`: OpenSim compatibility settings
- `[ai]`: AI feature configuration (Phase II)

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Check code formatting
cargo fmt --check

# Run clippy lints
cargo clippy -- -D warnings
```

### Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests
4. Ensure all tests pass: `cargo test`
5. Submit a pull request

### Code Structure

```
mutsea/
├── mutsea-core/           # Core types and utilities
├── mutsea-network/        # Network layer
├── mutsea-protocol/       # Protocol implementation
├── mutsea-assets/         # Asset management
├── mutsea-users/          # User management
├── mutsea-regions/        # Region management
├── mutsea-database/       # Database layer
├── mutsea-server/         # Main server
├── mutsea-cli/            # Command-line tools
├── config/                # Configuration files
├── docs/                  # Documentation
└── tests/                 # Integration tests
```

## Performance

Mutsea is designed for high performance:

- **Memory Safety**: Rust's ownership system prevents memory leaks and crashes
- **Zero-Copy Networking**: Efficient packet processing without unnecessary allocations
- **Async I/O**: Non-blocking operations for handling thousands of concurrent users
- **Intelligent Caching**: Multi-layer caching for assets and frequently accessed data

## OpenSim Compatibility

Mutsea maintains 100% compatibility with the OpenSimulator ecosystem:

- **Viewer Support**: Works with Firestorm, Singularity, and other OpenSim viewers
- **Asset Formats**: Supports all OpenSim asset types and formats
- **Protocol Compliance**: Full LLUDP and HTTP protocol implementation
- **Script Compatibility**: LSL and OSSL script support (via bridge)
- **Migration Tools**: Easy migration from existing OpenSim grids

## Roadmap

### Phase I: Foundation (Months 1-6) ✅
- [x] Project structure and core types
- [x] Network protocol implementation
- [x] Asset management system
- [x] User authentication
- [x] Firestorm compatibility
- [x] Database integration

### Phase II: AI Enhancement (Months 7-18)
- [ ] AI content generation
- [ ] Social intelligence system
- [ ] Natural language interface
- [ ] Adaptive environments
- [ ] Performance optimization

### Phase III: MapleAI Integration (Months 19-30)
- [ ] Multi-agent protocol
- [ ] Consensus engine
- [ ] Collective intelligence
- [ ] Cross-platform integration

### Phase IV: Ecosystem (Months 31-42)
- [ ] Developer APIs
- [ ] Plugin system
- [ ] Commercial marketplace
- [ ] Industry partnerships

## License

Mutsea is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.

## Support

- **Documentation**: [docs.mutsea.com](https://docs.mutsea.com)
- **Discord**: [Join our community](https://discord.gg/mutsea)
- **GitHub Issues**: [Report bugs and request features](https://github.com/finalverse/mutsea/issues)
- **Email**: support@finalverse.com

## Acknowledgments

- OpenSimulator project for the foundational virtual world architecture
- Firestorm Viewer team for the excellent viewer implementation
- Rust community for the amazing ecosystem and tools
- All contributors and testers who help make Mutsea better

---

Built with ❤️ by the Finalverse team.