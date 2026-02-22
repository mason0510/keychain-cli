# Keychain CLI - 项目规范（CLAUDE.md）

## 1. 项目信息

- **项目名称**: keychain-cli
- **描述**: Secure Keychain Management CLI for macOS
- **语言**: Rust
- **版本**: 0.2.0
- **License**: MIT
- **GitHub**: https://github.com/mason0510/keychain-cli
- **维护者**: Mason (@mason0510)

---

## 2. --help 标准配置规范

### 2.1 Help 信息结构

所有 Rust CLI 项目使用 `clap` 框架的 `long_about` 属性配置帮助信息，遵循以下标准结构：

```
[简短描述]

[CORE COMMANDS - 最重要的命令示例]

[FEATURES - 核心功能列表]

[EXAMPLES - 常见用法示例]

[DOCUMENTATION - 文档链接]

[LICENSE - 许可证信息]
```

### 2.2 实现方式（Rust + clap）

```rust
#[derive(Parser)]
#[command(name = "keychain-cli")]
#[command(about = "Short one-liner description")]
#[command(long_about =
r#"Detailed description with examples.

CORE COMMANDS (Quick Start):

  # Command 1 description
  command-name arg1 --flag value

  # Command 2 description
  command-name arg2 --another-flag

FEATURES:
  • Feature 1
  • Feature 2
  • Feature 3

EXAMPLES:
  # Use case 1
  command-name subcommand --option value

  # Use case 2
  command-name subcommand --option value

DOCUMENTATION:
  https://github.com/owner/repo#readme

LICENSE:
  MIT - See LICENSE file for details"#)]
#[command(version = "0.2.0")]
struct Cli {
    // ... fields
}
```

### 2.3 keychain-cli 的 --help 配置

**文件**: `src/main.rs`

**当前配置**:

```rust
#[derive(Parser)]
#[command(name = "keychain-cli")]
#[command(about = "Secure Keychain Management CLI for Claude Code")]
#[command(long_about =
r#"Secure secret management for macOS - Store API keys in Keychain with biometric protection.

CORE COMMANDS (Quick Start):

  # 1. Store your secrets
  keychain-cli setup --env-file ~/.env --force

  # 2. Load secrets to shell
  eval "$(keychain-cli load --format export)"

FEATURES:
  • Biometric-protected secret storage
  • <1 second load time for 61+ secrets
  • Hook-based command validation blocks dangerous operations
  • Dynamic rule system (no recompilation needed)
  • Multiple output formats (bash, json, export)

EXAMPLES:
  # Verify configuration
  keychain-cli check --verbose

  # Load specific secrets only
  keychain-cli load --format bash --keys ANTHROPIC_AUTH_TOKEN,MYSQL_PASSWORD

  # Validate command (for Hook integration)
  echo "cat .env" | keychain-cli validate

DOCUMENTATION:
  https://github.com/mason0510/keychain-cli#readme

LICENSE:
  MIT - See LICENSE file for details"#)]
#[command(version = "0.2.0")]
struct Cli {
    // ... fields
}
```

### 2.4 标准元素说明

#### CORE COMMANDS
- **目的**: 展示最常用的 1-2 个命令
- **格式**: 注释 + 完整命令示例
- **例子**:
  ```
  # 1. Store your secrets
  keychain-cli setup --env-file ~/.env --force

  # 2. Load secrets to shell
  eval "$(keychain-cli load --format export)"
  ```

#### FEATURES
- **目的**: 列举项目核心优势（3-5 项）
- **格式**: 用 • 或 - 分隔，每行一个特性
- **字符限制**: 保持在 60 字符以内
- **例子**:
  ```
  • <1 second load time for 61+ secrets
  • Biometric-protected secret storage
  • Hook-based command validation
  ```

#### EXAMPLES
- **目的**: 展示常见的 3-4 个使用场景
- **格式**: 注释 + 完整命令
- **例子**:
  ```
  # Verify configuration
  keychain-cli check --verbose

  # Load specific secrets only
  keychain-cli load --format bash --keys ANTHROPIC_AUTH_TOKEN
  ```

#### DOCUMENTATION
- **必须包含**: GitHub README 链接或官方文档链接
- **格式**: `https://github.com/owner/repo#readme`

#### LICENSE
- **必须包含**: 许可证类型和位置
- **格式**: `MIT - See LICENSE file for details`

---

## 3. 编辑 --help 信息的步骤

### 3.1 修改源代码

编辑 `src/main.rs` 中的 `#[command(long_about = r#"...")]` 部分：

```bash
vim src/main.rs

# 修改 long_about 中的内容
```

### 3.2 编译项目

```bash
cargo build --release
```

### 3.3 验证输出

```bash
./target/release/keychain-cli --help
```

### 3.4 更新系统二进制

```bash
cp ./target/release/keychain-cli /usr/local/bin/keychain-cli

# 验证
keychain-cli --help
```

### 3.5 提交更改

```bash
git add src/main.rs
git commit -m "feat: enhance --help with [description]"
git push origin main
```

---

## 4. 最佳实践

### 4.1 长度限制

- **short description** (`about`): 1 行，保持 < 80 字符
- **CORE COMMANDS**: 2-3 个命令
- **FEATURES**: 3-5 个特性
- **EXAMPLES**: 3-4 个常见用法
- **总长度**: 不超过 500 字符（保持简洁）

### 4.2 格式规范

- 使用 `r#"..."#` 原始字符串（避免转义）
- 各部分用空行分隔
- 代码示例用缩进 2 空格
- 注释用 `#` 开头

### 4.3 内容原则

✅ **DO**:
- 突出最常用的命令
- 用真实示例，可直接复制粘贴
- 包含文档和许可证链接
- 保持简洁清晰

❌ **DON'T**:
- 列举所有命令（那是 `Commands:` 部分的事）
- 过度装饰（避免过多 emoji）
- 链接到不存在的页面
- 包含过时信息

---

## 5. 更新记录

| 日期 | 版本 | 变更 |
|------|------|------|
| 2026-02-19 | 0.2.0 | 添加详细 --help 配置和最佳实践文档 |

---

## 6. 相关文件

- `src/main.rs` - --help 配置源代码
- `README.md` - 用户文档（中文）
- `README_EN.md` - 用户文档（英文）
- `USAGE_AND_MAINTENANCE.md` - 详细使用指南
- `ARCHITECTURE.md` - 架构说明
- `CONTRIBUTING.md` - 贡献指南
- `CHANGELOG.md` - 版本历史
- `LICENSE` - MIT 许可证

---

## 7. 命令行帮助子命令

### 查看子命令帮助

```bash
keychain-cli setup --help
keychain-cli load --help
keychain-cli validate --help
keychain-cli check --help
```

---

**最后更新**: 2026-02-19
**维护者**: Mason
**项目状态**: 积极维护中
