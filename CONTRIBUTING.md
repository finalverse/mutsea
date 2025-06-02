# Contributing to Mutsea

Thank you for your interest in contributing to Mutsea! This document provides guidelines and information for contributors.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/yourusername/mutsea.git
   cd mutsea
   ```
3. **Set up the development environment**:
   ```bash
   ./scripts/dev-setup.sh
   ```
4. **Create a branch** for your contribution:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Process

### Code Style

- Use `cargo fmt` to format your code
- Run `cargo clippy` to check for lints
- Follow Rust naming conventions
- Add documentation for public APIs
- Include tests for new functionality

### Testing

- Write unit tests for new functions
- Add integration tests for major features
- Ensure all tests pass: `cargo test`
- Test with actual Firestorm connections when possible

### Commits

- Use clear, descriptive commit messages
- Keep commits focused on a single change
- Reference issues in commit messages when applicable

### Pull Requests

1. Ensure your branch is up to date with main
2. Run the full test suite
3. Update documentation as needed
4. Create a clear pull request description
5. Link to relevant issues

## Project Areas

### Core Systems (Phase I)
- Protocol implementation
- Database layer
- Asset management
- User authentication
- Performance optimization

### AI Integration (Phase II)
- Content generation
- Social intelligence
- Natural language processing
- Performance optimization

### Advanced Features (Phase III+)
- MapleAI integration
- Cross-platform clients
- Advanced AI capabilities

## Code Organization

```
mutsea/
├── mutsea-core/           # Core types and utilities
├── mutsea-network/        # Network protocols
├── mutsea-protocol/       # LLUDP/HTTP implementation
├── mutsea-assets/         # Asset management
├── mutsea-users/          # User management
├── mutsea-regions/        # Region hosting
├── mutsea-database/       # Database abstraction
├── mutsea-server/         # Main server application
├── mutsea-cli/            # Command-line tools
├── tests/                 # Integration tests
└── docs/                  # Documentation
```

## Reporting Issues

When reporting bugs or requesting features:

1. Check if the issue already exists
2. Use the appropriate issue template
3. Provide clear reproduction steps
4. Include relevant system information
5. Add logs when applicable

## Security

- Report security vulnerabilities privately to security@finalverse.io
- Do not create public issues for security problems
- Allow time for fixes before public disclosure

## License

By contributing to Mutsea, you agree that your contributions will be licensed under the Apache License 2.0.

## Getting Help

- Join our Discord: [discord.gg/mutsea](https://discord.gg/mutsea)
- Read the documentation: [docs.mutsea.dev](https://docs.mutsea.dev)
- Create an issue for questions or problems

## Recognition

Contributors will be recognized in:
- The project README
- Release notes for major contributions
- Annual contributor appreciation posts

Thank you for helping make Mutsea better! 🚀