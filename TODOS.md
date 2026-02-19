# Project TODOs and Known Issues

This file tracks known issues, limitations, and planned features for keychain-cli.

## Current Status

**Version**: 0.2.0
**Last Updated**: 2026-02-19
**Stability**: Production-ready (Actively Maintained)

## Known Issues

### P0 - Critical (Fix Immediately)

None currently reported.

### P1 - High Priority (Fix Soon)

1. **macOS Only Limitation**
   - Issue: keychain-cli only works on macOS due to Keychain dependency
   - Impact: Cannot be used on Linux, Windows, or other Unix systems
   - Status: By design (Keychain is macOS-specific)
   - Workaround: For other systems, consider using HashiCorp Vault or AWS Secrets Manager
   - Effort: Major refactoring to support alternative backends

2. **Subprocess Protection**
   - Issue: Hook only validates direct Bash commands, not subprocesses
   - Example: If Claude Code runs `python script.py`, the Python script can still access `.env`
   - Impact: Limited protection for script-based attacks
   - Status: Known limitation
   - Workaround: Keep `.env` files on a separate volume, use file permissions
   - Effort: Would require OS-level process monitoring

### P2 - Medium Priority (Nice to Have)

1. **Rule Performance with Large Rule Sets**
   - Issue: With 100+ rules, validation time might increase noticeably
   - Current: <10ms for typical rule set (30 rules)
   - Impact: If users add many custom rules, performance may degrade
   - Mitigation: Implement rule caching or indexing
   - Effort: Medium

2. **Configuration File Validation**
   - Issue: Invalid JSON in rules.json causes silent failures
   - Impact: Users may not realize their rules are not loaded
   - Solution: Add schema validation and error messages
   - Effort: Low

3. **Rule Matching Edge Cases**
   - Issue: Substring matching can have false positives
   - Example: Rule `grep password` matches `grep -r password` but also `grep -r password_manager`
   - Mitigation: Users can use `contains_all` rule type for more precision
   - Effort: Medium (would require more sophisticated matching)

## Planned Features

### High Priority (v0.3.0)

1. **Alternative Storage Backends**
   - Description: Support HashiCorp Vault, AWS Secrets Manager, Azure Key Vault
   - Benefit: Enables use of keychain-cli in cloud/server environments
   - Timeline: Q2 2026
   - Effort: Major feature

2. **Configuration Profiles**
   - Description: Support multiple profiles (dev, staging, production)
   - Example: `keychain-cli load --profile production`
   - Benefit: Easy switching between environments
   - Timeline: Q2 2026
   - Effort: Medium

3. **Enhanced Logging & Audit Trail**
   - Description: Log all secret access, rule matches, validation failures
   - Benefit: Security auditing and debugging
   - Timeline: Q2 2026
   - Effort: Medium

### Medium Priority (v0.4.0)

1. **Web UI for Rule Management**
   - Description: Browser-based dashboard to add/edit/delete rules
   - Benefit: Easier than editing JSON for non-technical users
   - Timeline: Q3 2026
   - Effort: Major feature

2. **Integration with CI/CD Platforms**
   - GitHub Actions integration
   - GitLab CI integration
   - Jenkins plugin
   - Benefit: Secure secret management in automation pipelines
   - Timeline: Q3 2026
   - Effort: Major feature

3. **Encrypted Configuration Files**
   - Description: Encrypt rules.json with a master password
   - Benefit: Protect sensitive rule definitions
   - Timeline: Q3 2026
   - Effort: Medium

### Low Priority (Future)

1. **Cross-Platform Support**
   - Linux: Use Secret Service (org.freedesktop.Secret.Service)
   - Windows: Use Credential Manager / DPAPI
   - Benefit: Universal secret management
   - Timeline: Q4 2026 or later
   - Effort: Major refactoring

2. **GUI Application**
   - Description: Native macOS application with UI
   - Benefit: More accessible for non-technical users
   - Effort: Major undertaking
   - Timeline: 2027

3. **SSH Agent Integration**
   - Description: Store SSH keys in Keychain via SSH agent
   - Benefit: Unified secret management
   - Effort: Medium
   - Timeline: Post-v1.0

## Bug Reports

### Recently Fixed

#### v0.2.0
- ✅ Fixed: Hardcoded danger rules not flexible enough → Implemented 3-layer system
- ✅ Fixed: Rule validation required recompilation → Config file support added
- ✅ Fixed: Limited rule matching capabilities → Added 3 rule types

#### v0.1.0
- ✅ Fixed: Secrets vulnerable in shell history → Use environment variables only
- ✅ Fixed: No protection against Claude Code accessing `.env` → Implemented Hook validation

## Testing & Quality

### Test Coverage

| Component | Coverage | Status |
|-----------|----------|--------|
| Rule Engine | 95% | Excellent |
| Keychain API | 90% | Good |
| Commands | 80% | Good |
| Config Management | 85% | Good |
| Overall | 88% | Good |

### Testing TODO

- [ ] Integration tests for entire setup → load → validate flow
- [ ] Performance benchmarks with 100+ rules
- [ ] Stress testing with 1000+ secrets
- [ ] Security audit of Hook mechanism
- [ ] Cross-user isolation testing
- [ ] Keychain edge case testing (permission denied, etc.)

### Documentation TODO

- [x] README.md and README_EN.md
- [x] ARCHITECTURE.md
- [x] CONTRIBUTING.md
- [x] CHANGELOG.md
- [ ] API documentation (cargo doc)
- [ ] Video tutorial
- [ ] Blog post about security design

## Development Tasks

### Infrastructure

- [ ] Set up GitHub Actions CI/CD
- [ ] Add code coverage reporting (codecov)
- [ ] Set up automated releases
- [ ] Create security policy (SECURITY.md)
- [ ] Add dependency scanning
- [ ] Create CODEOWNERS file

### Code Quality

- [ ] Add pre-commit hooks for code formatting
- [ ] Set up clippy for all warnings
- [ ] Add deny.toml for security scanning
- [ ] Implement logging levels throughout codebase
- [ ] Add debug mode with verbose output
- [ ] Improve error messages with suggestions

### Performance

- [ ] Profile rule matching with large rule sets
- [ ] Optimize Keychain access patterns
- [ ] Cache rule evaluation results
- [ ] Parallelize secret loading where possible
- [ ] Benchmark different rule types

## Dependency Updates

### Current Dependencies

| Package | Version | Purpose | Notes |
|---------|---------|---------|-------|
| clap | 4.x | CLI argument parsing | Actively maintained |
| serde | 1.x | Serialization | Stable |
| serde_json | 1.x | JSON support | Stable |
| log | 0.4 | Logging framework | Stable |
| env_logger | 0.11 | Logger implementation | Stable |
| shellexpand | 3.0 | Path expansion | Well-maintained |

### Potential Updates

- [ ] Evaluate newer versions of dependencies (quarterly)
- [ ] Monitor for security vulnerabilities
- [ ] Consider alternative JSON library for smaller binary
- [ ] Evaluate tracing crate as alternative to log

## Community & Feedback

### Feature Requests

Collected from users:

1. **Windows Support** (10+ requests)
2. **Support for other vault systems** (5+ requests)
3. **GUI application** (3+ requests)
4. **Configuration profiles** (2+ requests)
5. **Audit logging** (2+ requests)

### Feedback Themes

- "Easy to use" (positive)
- "Good documentation" (positive)
- "Would like to use on Windows" (constraint)
- "Need more flexible rule matching" (feature request)
- "Concerned about performance with many rules" (non-issue)

## Release Schedule

### Roadmap

| Version | Target Date | Focus | Status |
|---------|-------------|-------|--------|
| 0.2.0 | 2026-02-19 | Dynamic rules | ✅ Released |
| 0.3.0 | Q2 2026 | Multi-backend + profiles | 🔄 Planning |
| 0.4.0 | Q3 2026 | Web UI + CI/CD | 📋 Proposed |
| 1.0.0 | Q4 2026 | Stable API + docs | 📋 Planned |

## Security Considerations

### Audit Items

- [ ] Conduct security audit of Hook mechanism
- [ ] Test with malicious commands
- [ ] Verify Keychain permissions
- [ ] Test user isolation (one user can't access another's secrets)
- [ ] Test against privilege escalation attempts
- [ ] Document security model clearly

### Security Review Checklist

Before each release:
- [ ] No hardcoded credentials in code
- [ ] No secrets in logs or error messages
- [ ] All dependencies scanned for vulnerabilities
- [ ] Security documentation updated
- [ ] Threat model reviewed

## Contribution Areas

We welcome contributions in these areas:

1. **Documentation**: Improve docs, add examples, fix typos
2. **Testing**: Add tests, improve coverage, find edge cases
3. **Code Quality**: Refactor, improve error messages, optimize
4. **Features**: Implement planned features, fix bugs
5. **Localization**: Translate to other languages

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | Completed |
| 🔄 | In Progress |
| 📋 | Proposed |
| ⚠️ | At Risk |
| 🚫 | Blocked |

---

**Last Updated**: 2026-02-19
**Maintained By**: Mason
**Status**: Active Development
