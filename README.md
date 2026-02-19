# Keychain CLI (keychain-cli)

![Language](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Crate](https://img.shields.io/badge/crates.io-keychain--cli-blue.svg)
![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)

> Secure Secret Management for macOS - Protect API keys in Keychain and block dangerous commands via Hook validation.

**[English Documentation](README_EN.md)** | [中文文档](README.md)

## 核心特性

- 🔐 **Keychain 加密存储** - 所有密钥存储在 macOS Keychain（受生物识别保护）
- 🛡️ **Hook 验证** - 自动拦截危险命令（.env 访问、docker compose config、grep PASSWORD 等）
- ⚡ **环境变量注入** - 密钥仅在内存中作为环境变量存在，永不写入磁盘
- 📋 **动态规则管理** - 无需重新编译即可添加/修改/删除安全规则
- 🎯 **多输出格式** - 支持 bash、json、export 多种格式
- 🔍 **状态检查** - 验证 Keychain 配置和密钥完整性

## 问题与方案

**问题**: Claude Code（AI 助手）可以读取 `.env` 文件、运行 `docker compose config`、使用 `grep PASSWORD` 等命令访问敏感信息。

**解决方案**：
1. 将所有密钥存储在 macOS Keychain（受生物识别保护）
2. 密钥仅在需要时作为环境变量注入，永不写入磁盘
3. 通过 Hook 机制拦截所有危险命令（无需 Claude 配合）

## 🚀 快速开始

### 安装

**From Source**
```bash
git clone https://github.com/mason0510/keychain-cli
cd keychain-cli
cargo build --release
cp target/release/keychain-cli /usr/local/bin/
```

**Verify Installation**
```bash
keychain-cli --version
```

### 初始化（一次性）

```bash
# 1. 准备 .env 文件（包含所有敏感密钥）
# 示例：
# ANTHROPIC_AUTH_TOKEN=sk-xxx
# MYSQL_PASSWORD=xxxx
# AWS_SECRET_ACCESS_KEY=xxxx

# 2. 将密钥存储到 Keychain
keychain-cli setup --env-file /path/to/.env --force

# 输出：✓ 已存储 61 个密钥到 Keychain
# 输出：✓ 创建状态文件 ~/.keychain/claude-dev.keys
```

### 在 Shell 中加载密钥

```bash
# 方案 A: 在 ~/.zshrc 或 ~/.bash_profile 中自动加载
echo 'eval "$(keychain-cli load --format export)"' >> ~/.zshrc

# 方案 B: 通过启动脚本加载
eval "$(keychain-cli load --format export)"

# 验证
echo $ANTHROPIC_AUTH_TOKEN  # 应该显示密钥值
```

### 验证配置

```bash
keychain-cli check --verbose

# 输出示例：
# ✓ Keychain 已初始化
# ✓ 已存储 61 个密钥
# ✓ 状态文件完整
# ✓ Hook 配置就绪
```

## Commands

### `setup` - Store secrets
```bash
keychain-cli setup --env-file <PATH> [--force] [--service-name claude-dev]
```
- Reads .env file
- Identifies sensitive variables (PASSWORD, SECRET, KEY, TOKEN, API_KEY, etc.)
- Stores in Keychain + creates state file `~/.keychain/claude-dev.keys`

### `load` - Retrieve secrets
```bash
keychain-cli load [--format bash|json|export] [--service-name claude-dev]
```
- **bash**: `export VAR=value` format
- **json**: JSON object format
- **export**: Shell-sourcing format

Use in shell: `eval "$(keychain-cli load --format export)"`

### `validate` - Hook validation (for Claude Code)
```bash
echo "cat .env" | keychain-cli validate
# Exit 2 if dangerous, 0 if safe
```

Blocks these patterns:
- `.env*` file access
- `docker compose config`
- `security find-generic` / `grep PASSWORD` / `grep SECRET`
- `~/.ssh/` and `~/.aws/` access
- `.bash_history`, `.zsh_history`
- `find ... -name password/secret/key`

### `check` - Verify configuration
```bash
keychain-cli check [--verbose]
```
Shows Keychain status, stored secrets count, and next steps.

## Claude Code Integration

### Step 1: Configure Hook
Create/update `~/.claude/settings.json`:
```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "/usr/local/bin/keychain-cli validate"
          }
        ]
      }
    ]
  }
}
```

### Step 2: Create Startup Script
Create `~/start-claude.sh`:
```bash
#!/bin/bash
eval "$(keychain-cli load --format export --service-name claude-dev)"
exec "$@"
```

### Step 3: Load Secrets in Profile
Add to `~/.zshrc`:
```bash
eval "$(keychain-cli load --format export)"
```

## File Locations

| File | Path | Purpose |
|------|------|---------|
| CLI Binary | `/usr/local/bin/keychain-cli` | Main tool |
| Secrets Metadata | `~/.keychain/claude-dev.keys` | Key names list (not values) |
| Hook Config | `~/.claude/settings.json` | Claude Code configuration |
| Original .env | `/Volumes/otherdata/mac/claudecode-safe/credentials/.env` | **NEVER accessed by Claude Code** |

## Rule Management (Dynamic)

The CLI uses a **3-layer rule system** for command validation, allowing you to customize security rules without recompilation:

### Layer 1: Built-in Rules
- Hardcoded in Rust source (`src/rules/mod.rs`)
- 30+ core security rules (e.g., `.env` access, `docker compose config`, `grep PASSWORD`)
- Always active unless overridden

### Layer 2: Configuration File Rules
- Path: `~/.keychain/rules.json`
- JSON format with enable/disable toggle
- Load automatically at startup
- No recompilation needed

**Example `~/.keychain/rules.json`:**
```json
{
  "rules": [
    {
      "id": "custom_mysql_dump",
      "type": "substring",
      "pattern": "mysqldump",
      "description": "Block database exports",
      "enabled": true
    },
    {
      "id": "custom_aws_creds",
      "type": "contains_all",
      "patterns": ["aws", "credentials"],
      "description": "Block AWS credential access",
      "enabled": true
    }
  ]
}
```

**Rule Types:**
- `substring`: Match if command contains pattern (case-insensitive)
- `contains_all`: Match only if ALL patterns are present
- `contains_any`: Match if ANY pattern is present

### Layer 3: Environment Variable Rules
- Format: `KEYCHAIN_CUSTOM_RULES="pattern1|pattern2|pattern3"`
- Temporary rules for testing
- No configuration file needed

**Example:**
```bash
# Add temporary rules via environment variable
export KEYCHAIN_CUSTOM_RULES="test_pattern1|payment_api"
keychain-cli validate "payment_api call"  # Will be blocked
```

### Adding New Rules (Without Recompilation)

**Option A: Edit configuration file**
```bash
# 1. Edit the config file
vim ~/.keychain/rules.json

# 2. Add a new rule (copy-paste existing, modify id/pattern/description)
# 3. Set "enabled": true
# 4. Save file

# 5. Test immediately (no recompilation needed!)
keychain-cli validate "your test command"
```

**Option B: Disable troublesome rules**
```bash
# If a rule causes false positives, temporarily disable it
vim ~/.keychain/rules.json
# Change "enabled": true to "enabled": false
```

**Option C: Temporary testing with environment variables**
```bash
# Test a rule before adding to config
export KEYCHAIN_CUSTOM_RULES="experimental_pattern"
keychain-cli validate "experimental_pattern test"

# Unset when done testing
unset KEYCHAIN_CUSTOM_RULES
```

### Best Practices for Rules

1. **Use simple patterns**: `substring` rules are preferred for most cases
2. **Use `contains_all` for compound rules**: Requires ALL keywords present
3. **Test before enabling**: Use `KEYCHAIN_CUSTOM_RULES` to test first
4. **Add descriptive `id` and `description`**: Helps with debugging
5. **Keep rules maintainable**: Comment why you added each rule

## Security Guarantees

1. **Keychain Protected**: All secret values in macOS Keychain (biometric authentication)
2. **Memory-Only**: Secrets as environment variables, never written to disk logs
3. **Hook Validation**: Every Bash command verified before execution
4. **Metadata Only**: `~/.keychain/*.keys` contains only key names, not values
5. **Atomic Operations**: setup and load are atomic (no partial state)

## Troubleshooting

**"No secrets found"**
```bash
# Re-run setup
keychain-cli setup --env-file /Volumes/otherdata/mac/claudecode-safe/credentials/.env --force
```

**Secrets not loading**
```bash
# Verify state file exists
cat ~/.keychain/claude-dev.keys | wc -l  # Should show ~61

# Check keychain-cli is in PATH
which keychain-cli  # Should show /usr/local/bin/keychain-cli
```

**"Keychain authorization required"**
- Normal macOS security behavior
- Enter Mac password or use Touch ID
- Cached in session, no prompt on subsequent accesses

**$ANTHROPIC_AUTH_TOKEN is empty**
```bash
# Verify you ran eval command
eval "$(keychain-cli load --format export)"

# Check if secrets exist
keychain-cli load --format bash | head -5
```

## Best Practices

✅ **DO**:
- Load secrets in shell profile
- Use Hook to block dangerous commands
- Verify setup with `keychain-cli check`
- Keep ~/.keychain directory permission 600

❌ **DON'T**:
- Let Claude Code read .env files
- Hardcode API keys in scripts
- Disable Hook validation
- Share ~/.keychain directory contents

## Technical Details

**State File Format** (`~/.keychain/claude-dev.keys`):
```
ANTHROPIC_AUTH_TOKEN
AZURE_SPEECH_KEY
BARK_KEY
[... one key per line, sorted ...]
```

**Exit Codes**:
- `0`: Success or safe command
- `1`: General error (missing secrets, I/O error)
- `2`: Hook validation failed (dangerous command blocked)

**Performance**:
- setup 61 secrets: ~3 seconds
- load 61 secrets: ~1 second
- validate command: <10ms
- Binary size: 2.4 MB (release)

## 🛠️ 开发

### 环境要求

- Rust 1.70+
- macOS 10.15+
- Cargo

### 本地开发

```bash
# 克隆项目
git clone https://github.com/mason0510/keychain-cli
cd keychain-cli

# 构建
cargo build

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 构建发布版本
cargo build --release
```

### 项目结构

```
src/
├── main.rs           # CLI 入口点
├── commands/         # 子命令实现
│   ├── setup.rs      # setup 命令
│   ├── load.rs       # load 命令
│   ├── validate.rs   # validate 命令
│   └── check.rs      # check 命令
├── rules/           # 规则引擎
│   └── mod.rs       # 规则定义和匹配
├── keychain/        # Keychain API
│   └── mod.rs
├── config.rs        # 配置管理
└── error.rs         # 错误处理
```

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE)

## 🤝 贡献

欢迎贡献！请参考 [CONTRIBUTING.md](CONTRIBUTING.md)

- 报告 Bug：[GitHub Issues](https://github.com/mason0510/keychain-cli/issues)
- 提交特性请求：[GitHub Issues](https://github.com/mason0510/keychain-cli/issues)
- 提交 PR：[GitHub Pull Requests](https://github.com/mason0510/keychain-cli/pulls)

## 📧 联系方式

- GitHub: [@mason0510](https://github.com/mason0510)
- Issues: [GitHub Issues](https://github.com/mason0510/keychain-cli/issues)

## 致谢

感谢所有贡献者的支持！

---

**最后更新**: 2026-02-19
**维护者**: Mason
**项目状态**: 积极维护中
