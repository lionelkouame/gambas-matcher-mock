# Contributing to Gambas Matcher Mock

Thank you for your interest in contributing to Gambas Matcher Mock! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/gambas-matcher-mock.git`
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Build the project: `cargo build --release`
7. Commit your changes: `git commit -am 'Add some feature'`
8. Push to the branch: `git push origin feature/your-feature-name`
9. Create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running in Development Mode

```bash
cargo run -- --config examples/config.yaml
```

## Code Style

- Follow Rust naming conventions
- Use `rustfmt` to format your code: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Add tests for new features
- Update documentation as needed

## Commit Messages

- Use clear and descriptive commit messages
- Start with a verb in the imperative mood (e.g., "Add", "Fix", "Update")
- Keep the first line under 50 characters
- Add detailed description if needed

## Testing

- Write unit tests for new functionality
- Ensure all tests pass before submitting a PR
- Add integration tests for complex features

## Reporting Issues

- Use the GitHub issue tracker
- Provide a clear description of the problem
- Include steps to reproduce
- Mention your environment (OS, Rust version, etc.)

## Feature Requests

- Use the GitHub issue tracker
- Clearly describe the feature
- Explain why it would be useful
- Provide examples if possible

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
