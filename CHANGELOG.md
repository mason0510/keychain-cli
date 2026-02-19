# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned Features
- Support for multiple secret storage backends (Vault, AWS Secrets Manager)
- Configuration profiles for different environments
- Encrypted configuration files
- Web UI for rule management
- Audit logging for secret access
- Cross-platform support (Windows, Linux)

## [0.2.0] - 2026-02-19

### Added
- **Dynamic Rule Management (3-Layer System)**
  - Layer 1: Built-in hardcoded security rules (30+ patterns)
  - Layer 2: Configuration file rules via `~/.keychain/rules.json`
  - Layer 3: Environment variable rules via `$KEYCHAIN_CUSTOM_RULES`
  - Support for three rule types: `substring`, `contains_all`, `contains_any`
  - Rules can be added/modified/deleted without recompilation

- **Rule Engine Module** (`src/rules/mod.rs`)
  - `RuleEngine` struct with 3-layer loading
  - Automatic rule loading from config files and environment
  - Case-insensitive matching algorithm (Turing Option 2 - Hybrid approach)
  - Comprehensive unit tests for all rule types

- **Enhanced Validation Command**
  - Updated `validate` command to use dynamic RuleEngine
  - Support for custom rule patterns via environment variables
  - Improved error handling and logging

- **Configuration File Support**
  - JSON-based rule configuration at `~/.keychain/rules.json`
  - Example configuration file with 4 sample rules
  - Automatic config loading without recompilation

- **Open Source Preparation**
  - Complete documentation suite (README_EN.md, ARCHITECTURE.md)
  - Contributing guidelines (CONTRIBUTING.md)
  - Changelog and licensing information
  - Badges and proper project structure

### Changed
- **Architecture Documentation**: Added comprehensive ARCHITECTURE.md explaining:
  - 3-layer rule system design
  - Module structure and responsibilities
  - Data flow diagrams
  - Integration points
  - Extension points for future development

- **README.md Reorganization**: Updated to follow open source conventions:
  - Added language and license badges
  - Emoji-prefixed section headers
  - Proper feature list with descriptions
  - Expanded rule management documentation
  - Improved development section with project structure

### Fixed
- Improved command validation accuracy with hybrid matching algorithm
- Better error messages for configuration issues
- Enhanced logging for debugging rule engine behavior

### Dependencies
- Added: `shellexpand = "3.0"` for path expansion in rule loading

### Performance
- Rule loading optimized for startup speed
- Multiple rule evaluation combined into single pass
- No performance regression compared to v0.1.0

## [0.1.0] - 2026-02-10

### Added
- **Core Secret Management**
  - `setup` command: Store secrets from `.env` files to macOS Keychain
  - `load` command: Retrieve secrets from Keychain
  - `check` command: Verify configuration status
  - `validate` command: Hook validation for Claude Code integration

- **Keychain Integration**
  - Secure storage of secrets using macOS Keychain
  - Biometric authentication protection
  - Automatic sensitive variable detection (PASSWORD, SECRET, KEY, TOKEN, API_KEY)
  - State file tracking (key names, not values)

- **Claude Code Hook Support**
  - Pre-execution command validation
  - Blocking dangerous patterns:
    - `.env*` file access
    - `docker compose config`
    - `grep PASSWORD/SECRET/KEY`
    - SSH/AWS credential access
    - History file access
  - Integration with `~/.claude/settings.json`

- **Output Formats**
  - bash: `export VAR=value` format
  - json: JSON object with all secrets
  - export: Shell-sourcing format

- **Configuration**
  - Configurable service name
  - Custom state file location
  - Environment variable support

- **Documentation**
  - Initial README.md with quick start guide
  - MAINTENANCE.md with Q&A and troubleshooting
  - Code comments and inline documentation

### Initial Features
- Security validation (30+ hardcoded danger rules)
- Keychain API wrapper
- Config management
- Error handling
- Logging framework
- Shell integration support

### Technical Details
- Written in Rust for security and performance
- Single binary (2.4 MB release size)
- No external dependencies for Keychain (uses system `security` command)
- Cross-platform compatible (designed for macOS)

### Performance Metrics
- setup 61 secrets: ~3 seconds
- load 61 secrets: ~1 second
- validate command: <10ms
- Binary size: 2.4 MB

## Security Notes

### v0.2.0 Improvements
- Dynamic rule system eliminates need to recompile for new security patterns
- Environment variable rules enable quick testing of new patterns
- Configuration-based rules make security policies easier to maintain

### v0.1.0 Foundation
- All secrets encrypted by macOS Keychain
- No secrets written to disk
- Secrets exist only in memory as environment variables
- Hook mechanism prevents Claude Code from accessing sensitive files

## Migration Guide

### From v0.1.0 to v0.2.0

**New Capability**: Add security rules without recompiling!

**Migration Steps**:
1. Update binary: `cargo build --release`
2. (Optional) Create `~/.keychain/rules.json` to customize rules
3. Existing `setup` and `load` commands work identically
4. `validate` command now supports dynamic rules

**Breaking Changes**: None. v0.2.0 is fully backward compatible.

**New Usage**:
```bash
# Add custom rules via config file
vim ~/.keychain/rules.json

# Or test rules via environment variable
export KEYCHAIN_CUSTOM_RULES="pattern1|pattern2"
```

## Known Issues

- macOS only (Keychain is macOS-specific feature)
- Requires macOS 10.15 or later
- Cannot protect sub-processes (only direct Bash commands)
- Rule matching is case-insensitive (by design)

## Contributors

- Mason (@mason0510) - Creator and maintainer

## References

- [Keep a Changelog](https://keepachangelog.com/)
- [Semantic Versioning](https://semver.org/)
- [GitHub Releases](https://github.com/mason0510/keychain-cli/releases)

---

**Latest Update**: 2026-02-19
**Maintainer**: Mason
**Project Status**: Actively Maintained

[Unreleased]: https://github.com/mason0510/keychain-cli/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/mason0510/keychain-cli/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/mason0510/keychain-cli/releases/tag/v0.1.0
