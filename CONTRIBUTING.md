# Contributing to catopy

Thank you for your interest in contributing! This document provides guidelines and instructions.

## Getting Started

### Prerequisites

- Rust 1.70+ (2024 edition)
- Cargo
- `cross` for cross-compilation (optional): `cargo install cross`

### Development Setup

```bash
git clone https://github.com/febrezo/catopy.git
cd catopy
cargo build
cargo test
```

## Development Workflow

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test output::tests::success_format

# With output
cargo test -- --nocapture
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Cross-compilation (requires `cross`)
cross build --target x86_64-unknown-linux-musl
cross build --target aarch64-unknown-linux-gnu
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Check (fast)
cargo check
```

## Submitting Changes

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/my-feature`
3. **Commit** with clear messages: `git commit -m "feat: add X feature"`
4. **Test** locally: `cargo test && cargo clippy`
5. **Push** and create a **Pull Request**

### Commit Message Format

Use conventional commits:

- `feat:` — new feature
- `fix:` — bug fix
- `docs:` — documentation changes
- `test:` — test additions/changes
- `chore:` — build, deps, etc.
- `perf:` — performance improvements

Example:
```
feat: add --max-lines flag to limit output lines

- Implement new CLI option
- Add unit tests
- Update man pages
```

## Reporting Bugs

1. Check existing **Issues** first
2. Provide:
   - OS and Rust version (`rustc --version`)
   - Exact command that fails
   - Error output
   - File size/type (if relevant)

## Documentation

- **Code comments**: Explain *why*, not what
- **Man pages**: Update `man/catopy.1*` files
- **README.md**: Update installation/usage sections
- **CHANGELOG.md**: Document changes

## License

By contributing, you agree your code is licensed under **GPLv3** (same as the project).

## Questions?

Open an **Issue** with tag `question:` or start a **Discussion**.

---

**Thank you for contributing to catopy!** 🎉
