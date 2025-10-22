# Contributing to greetme

Thank you for your interest in contributing to greetme! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/greetme.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Run linter: `cargo clippy -- -D warnings`
7. Format code: `cargo fmt`
8. Commit your changes: `git commit -am 'Add some feature'`
9. Push to the branch: `git push origin feature/your-feature-name`
10. Create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo
- Git

### Building

```bash
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Running Locally

```bash
# Build and run
cargo run -- -c

# Run with arguments
cargo run -- -t "Hello World"
```

## Code Style

- Follow Rust standard naming conventions
- Use `cargo fmt` to format code
- Ensure `cargo clippy` passes with no warnings
- Write descriptive commit messages
- Add tests for new functionality
- Update documentation as needed

## Pull Request Guidelines

- Keep changes focused and atomic
- Write clear descriptions of what the PR does
- Reference any related issues
- Ensure all tests pass
- Update CHANGELOG.md if applicable
- Add yourself to contributors if desired

## Adding New Themes

To add a new theme:

1. Create a new TOML file in `themes/` directory
2. Follow the existing theme format:

```toml
name = "mytheme"
description = "My custom theme description"
foreground = "#rrggbb"
background = "#rrggbb"
accent = "#rrggbb"
strong = "#rrggbb"
dim = "#rrggbb"
ascii_art_style = "bold"  # or "normal", "italic"
```

3. Add the theme to the default themes in `src/config.rs`
4. Add tests if applicable
5. Update documentation

## Adding New Fonts

To add a new font:

1. Obtain a valid FIGlet font file (`.flf` format)
2. Add it to the `fonts/` directory
3. Update `src/render.rs` to handle the new font
4. Add the font to default fonts in `src/config.rs`
5. Update documentation

## Reporting Bugs

When reporting bugs, please include:

- Your operating system and version
- Rust version (`rustc --version`)
- greetme version (`greetme --version`)
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Any error messages or logs

## Feature Requests

Feature requests are welcome! Please:

- Check if the feature has already been requested
- Clearly describe the feature and its use case
- Explain why it would be useful to other users
- Be open to discussion and feedback

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions

## Questions?

Feel free to open an issue for questions or discussion.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
