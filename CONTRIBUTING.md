# Contributing to Keychain CLI

Thank you for your interest in contributing to Keychain CLI! This document provides guidelines and instructions for contributing.

## Code of Conduct

- Be respectful and constructive in all interactions
- Welcome diverse perspectives and experiences
- Focus on what's best for the project and community
- Report unacceptable behavior to maintainers

## How to Contribute

### Reporting Bugs

Before submitting a bug report, please check the [issue tracker](https://github.com/mason0510/keychain-cli/issues) to avoid duplicates.

**When submitting a bug report, include**:
- Clear, descriptive title
- Detailed description of the problem
- Steps to reproduce the issue
- Expected behavior vs actual behavior
- Your environment:
  - macOS version
  - Rust version (if building from source)
  - keychain-cli version
- Relevant logs or error messages

**Example**:
```
Title: keychain-cli validate fails with "permission denied" on zsh

Steps to reproduce:
1. Set up keychain-cli as documented
2. Add validate hook to Claude Code settings
3. Try to run "cat .env" command in Claude Code

Expected: Command blocked with exit code 2
Actual: Permission denied error, exit code 1

Environment:
- macOS 14.2
- Rust 1.75.0
- keychain-cli 0.2.0
```

### Suggesting Enhancements

**Before suggesting a feature**:
1. Check the [issues list](https://github.com/mason0510/keychain-cli/issues)
2. Check the [TODOS.md](TODOS.md) file
3. Review the [roadmap](#roadmap) section below

**When suggesting a feature, include**:
- Clear use case and motivation
- Detailed description of proposed behavior
- Possible alternatives you've considered
- Example usage or mockup if applicable

### Pull Requests

We welcome pull requests! Follow these guidelines:

#### Before You Start

1. **Fork the repository** and create a new branch
2. **Choose a branch name**: `feature/description` or `fix/description`
3. **Create an issue first** for significant changes (let's discuss before you code)

#### Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/keychain-cli.git
cd keychain-cli

# Add upstream remote
git remote add upstream https://github.com/mason0510/keychain-cli.git

# Create feature branch
git checkout -b feature/your-feature-name
```

#### Making Changes

**Code Style**:
- Follow Rust conventions (enforced by `cargo fmt`)
- Run `cargo clippy` for linting
- Aim for clear, self-documenting code
- Add comments for non-obvious logic

**Commit Messages**:
```
<type>: <short description (50 chars max)>

<longer explanation if needed (wrap at 72 chars)>

Fixes #123
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation updates
- `refactor`: Code restructuring (no behavior change)
- `test`: Add/modify tests
- `chore`: Build, dependencies, tooling

**Example**:
```
feat: Add dynamic rule management via environment variables

Implements Layer 3 of the 3-layer rule system, allowing users
to add temporary security rules via KEYCHAIN_CUSTOM_RULES env var
without modifying config files.

Fixes #45
```

#### Testing

**Before submitting**:
```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test

# Format code
cargo fmt

# Lint with clippy
cargo clippy

# Build release version
cargo build --release
```

**Testing Checklist**:
- [ ] All existing tests pass
- [ ] New functionality has tests
- [ ] Code compiles without warnings
- [ ] `cargo fmt` passes
- [ ] `cargo clippy` passes
- [ ] Manual testing completed
- [ ] Documentation updated

#### Submitting a PR

1. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Open a Pull Request** on GitHub:
   - Clear title describing what you changed
   - Reference related issues: "Fixes #123"
   - Describe your changes in detail
   - Include testing notes

3. **Respond to reviews**: Be open to feedback and discussion

### Documentation Improvements

Documentation is crucial! You can help by:
- Fixing typos or unclear explanations
- Adding examples
- Improving error messages
- Writing tutorials or guides
- Updating API documentation

**Documentation Files**:
- `README.md` / `README_EN.md`: User guide
- `ARCHITECTURE.md`: Technical architecture
- `CONTRIBUTING.md`: This file
- `CHANGELOG.md`: Release notes
- Code comments: Inline documentation

## Project Guidelines

### Architecture Decisions

We follow these principles:

1. **Security First**: Any change affecting security gets extra scrutiny
2. **Minimal Dependencies**: Avoid external dependencies when possible
3. **Single Responsibility**: Each module has one clear purpose
4. **Testability**: Code should be easy to test
5. **Performance**: Critical paths should be optimized

### The 3-Layer Rule System

When modifying rule-related code, remember:
- **L1 (Built-in)**: Core protection, hardcoded in Rust
- **L2 (Config file)**: User customization via JSON
- **L3 (Environment)**: Temporary testing patterns

All changes should respect this separation of concerns.

### Backward Compatibility

- Never remove existing CLI options without deprecation
- New options should be backward compatible
- Document any breaking changes clearly
- Provide migration guides for breaking changes

## Development Workflow

### Setting Up Your Development Environment

```bash
# Clone the repo
git clone https://github.com/mason0510/keychain-cli.git
cd keychain-cli

# Install dependencies (Rust)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify setup
rustc --version
cargo --version

# Build the project
cargo build

# Run tests
cargo test

# Build documentation
cargo doc --open
```

### Common Development Tasks

**Run the CLI locally**:
```bash
cargo run -- --help
cargo run -- check --verbose
cargo run -- validate < command.txt
```

**Debug with logging**:
```bash
RUST_LOG=debug cargo run -- check
```

**Benchmarking**:
```bash
time cargo run --release -- load
time cargo run --release -- validate
```

**Code coverage**:
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

### Release Process

Maintainers follow this process for releases:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with new features/fixes
3. Run full test suite
4. Create git tag: `git tag v1.x.x`
5. Push tag: `git push --tags`
6. Publish to crates.io: `cargo publish`
7. Create GitHub Release with notes

**Version Format**: Semantic Versioning (MAJOR.MINOR.PATCH)

## Roadmap

### Planned Features

- [ ] Support for other secret management systems (HashiCorp Vault, AWS Secrets Manager)
- [ ] Configuration profile support for different environments
- [ ] Encrypted configuration files for sensitive rule definitions
- [ ] Integration with more CI/CD platforms
- [ ] Web UI for rule management
- [ ] Audit logging for all secret access

### Known Limitations

- macOS only (Keychain is macOS-specific)
- Requires `security` command-line tool (installed by default on macOS)
- Cannot recursively protect subprocesses (only direct Bash commands)

### Discussion Topics

- Alternative storage backends for non-macOS systems
- Performance optimizations for large secret sets
- Integration with other security tools

## Getting Help

### Documentation
- [README.md](README.md) - Quick start guide
- [ARCHITECTURE.md](ARCHITECTURE.md) - Technical details
- [API Documentation](https://docs.rs/keychain-cli/): `cargo doc --open`

### Community
- [GitHub Issues](https://github.com/mason0510/keychain-cli/issues) - Ask questions
- [GitHub Discussions](https://github.com/mason0510/keychain-cli/discussions) - General discussion

### Maintainers
- Mason (@mason0510)

## License

By contributing to Keychain CLI, you agree that your contributions will be licensed under its MIT License.

## Acknowledgments

Thank you to all contributors who have helped make Keychain CLI better!

---

**Last Updated**: 2026-02-19
**Guidelines Version**: 1.0
