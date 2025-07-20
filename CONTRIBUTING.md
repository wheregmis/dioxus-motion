# Contributing to Dioxus Motion

Thank you for your interest in contributing to Dioxus Motion! This document provides guidelines and information for contributors.

## Development Setup

### Prerequisites

- Rust (latest stable version)
- Cargo
- Git

### Local Development

1. Clone the repository:
   ```bash
   git clone https://github.com/wheregmis/dioxus-motion.git
   cd dioxus-motion
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. Run clippy checks:
   ```bash
   cargo clippy --all-features -- -D warnings
   ```

## CI/CD Pipeline

We use GitHub Actions for continuous integration. The CI pipeline runs on every pull request to the main branch and includes:

### Comprehensive CI Checks
- **Compilation Check**: Ensures code compiles with all features
- **Clippy Check**: Enforces Rust coding standards and catches common issues
- **Test Suite**: Runs all unit and integration tests
- **Formatting Check**: Ensures code follows rustfmt standards
- **Security Audit**: Scans for known vulnerabilities
- **Multi-platform Testing**: Tests on Ubuntu, Windows, and macOS
- **Feature Matrix**: Tests all feature combinations (web, desktop, transitions)
- **Documentation Build**: Validates documentation generation

## Code Quality Standards

### Rust Standards
- All code must compile without warnings
- Clippy warnings are treated as errors
- Code must be formatted with `rustfmt`
- All tests must pass

### Commit Guidelines
- Use conventional commit messages
- Keep commits focused and atomic
- Include tests for new features
- Update documentation as needed

### Pull Request Process
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all CI checks pass
6. Submit a pull request

## Testing

### Running Tests Locally
```bash
# Run all tests
cargo test

# Run tests with specific features
cargo test --features web
cargo test --features desktop
cargo test --features transitions

# Run tests with all features
cargo test --all-features
```

### Running Clippy
```bash
# Run clippy with all features
cargo clippy --all-features -- -D warnings

# Run clippy on workspace
cargo clippy --workspace --all-features -- -D warnings
```

## Feature Development

### Adding New Features
1. Create a new branch for your feature
2. Implement the feature with appropriate tests
3. Update documentation
4. Ensure all CI checks pass
5. Submit a pull request

### Feature Flags
The project uses feature flags to control functionality:
- `web`: Web platform support (default)
- `desktop`: Desktop platform support
- `transitions`: Page transition support

## Documentation

### Code Documentation
- All public APIs must be documented
- Use Rust doc comments (`///`)
- Include examples in documentation
- Update README.md for user-facing changes

### Testing Documentation
- Write clear test descriptions
- Include edge case tests
- Test both success and failure scenarios

## Release Process

Releases are automated using `release-plz`:
1. Changes are merged to main
2. `release-plz` creates a PR with version bumps
3. After review and merge, `release-plz` publishes to crates.io

## Getting Help

- Open an issue for bugs or feature requests
- Join discussions in GitHub issues
- Check existing documentation

## License

By contributing to Dioxus Motion, you agree that your contributions will be licensed under the MIT License. 