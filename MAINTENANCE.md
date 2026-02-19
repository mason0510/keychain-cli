# Keychain CLI - 维护指南

## 常见问题（QA）

### Q1: 如何添加新的敏感密钥？

```bash
# 新增密钥到 .env 文件
echo "MY_NEW_API_KEY=sk-xxxx" >> /Volumes/otherdata/mac/claudecode-safe/credentials/.env

# 重新运行 setup（仅存储新密钥）
keychain-cli setup --env-file /Volumes/otherdata/mac/claudecode-safe/credentials/.env --force

# 验证
keychain-cli check --verbose
```

### Q2: 如何删除已过期的密钥？

```bash
# 方案 A: 使用 macOS security 命令直接删除
security delete-generic-password -a claude-dev -s MY_OLD_API_KEY

# 验证已删除
keychain-cli check --verbose

# 方案 B: 从 .env 文件删除后重新 setup
# 1. 编辑 /Volumes/otherdata/mac/claudecode-safe/credentials/.env
# 2. 删除对应行
# 3. keychain-cli setup --env-file /Volumes/otherdata/mac/claudecode-safe/credentials/.env --force
```

### Q3: 如何 rotate 密钥（更新值）？

```bash
# 更新 .env 文件中的密钥值
vi /Volumes/otherdata/mac/claudecode-safe/credentials/.env

# 例如：
# 旧值：ANTHROPIC_AUTH_TOKEN=sk-old-xxx
# 新值：ANTHROPIC_AUTH_TOKEN=sk-new-yyy

# 重新 setup（会自动更新）
keychain-cli setup --env-file /Volumes/otherdata/mac/claudecode-safe/credentials/.env --force

# 验证
keychain-cli load | grep ANTHROPIC_AUTH_TOKEN
```

### Q4: 密钥丢失或 Keychain 损坏怎么办？

```bash
# 步骤 1: 检查状态
keychain-cli check --verbose

# 步骤 2: 如果检测到问题，清除所有密钥并重新设置
# 删除所有该 service 的 Keychain 条目
security delete-generic-password -a claude-dev

# 删除 State File
rm -f ~/.keychain/claude-dev.keys

# 步骤 3: 重新设置
keychain-cli setup --env-file /Volumes/otherdata/mac/claudecode-safe/credentials/.env --force

# 步骤 4: 验证
keychain-cli check --verbose
```

### Q5: Hook 拦截了错误的命令怎么办？

```bash
# 临时绕过（重新配置 settings.json）
# 1. 编辑 ~/.claude/settings.json
# 2. 注释掉或删除 Hook 配置（暂时）
# 3. 测试命令
# 4. 向开发者报告误报

# 永久修复：更新验证规则
# 编辑 keychain/src/commands/validate.rs
# 修改 dangerous_patterns 黑名单
# 重新编译：cd /Users/houzi/code/06-production-business-money-live/keychain && cargo build --release
# 重新安装：cp target/release/keychain-cli /usr/local/bin/
```

### Q6: 如何在多个项目中使用？

```bash
# 不同的 service_name 隔离不同的密钥集合

# 项目 A（sub2api - 主项目）
keychain-cli load --service-name claude-dev

# 项目 B（新项目）
keychain-cli setup --env-file /path/to/projectB/.env --service-name projectB
keychain-cli load --service-name projectB

# 每个项目的 settings.json 可以指向不同的 service-name
# 在 ~/.claude/settings.json 中：
# "command": "/usr/local/bin/keychain-cli validate --service-name projectB"
```

### Q7: 如何审计谁访问了哪些密钥？

```bash
# 日志检查：使用 --verbose 模式查看详细操作
keychain-cli load --verbose
# 输出：
# [DEBUG] Retrieving ANTHROPIC_AUTH_TOKEN from Keychain
# [DEBUG] Retrieving MYSQL_PASSWORD from Keychain
# ...

# 保存审计日志到文件
keychain-cli load --verbose 2>&1 | tee /var/log/keychain-audit.log

# 查看历史操作
grep "Retrieving" /var/log/keychain-audit.log
```

### Q8: 环境变量在 child shell 中不可用

```bash
# 问题：某些 shell 嵌套中无法访问密钥
# 原因：环境变量没有被正确继承

# 解决方案 1: 使用 start-claude.sh
/Users/houzi/code/06-production-business-money-live/sub2api/start-claude.sh python app.py

# 解决方案 2: 在 .zshrc 中自动加载
eval "$(keychain-cli load --format export)"

# 解决方案 3: 显式在 shell 中加载
bash -c 'eval "$(keychain-cli load --format export)" && python app.py'
```

### Q9: 如何验证密钥格式和有效性？

```bash
# 列出所有密钥
keychain-cli load --format bash

# 查看 State File 中的密钥名称
cat ~/.keychain/claude-dev.keys

# 检查特定密钥
keychain-cli load --keys ANTHROPIC_AUTH_TOKEN --format bash

# 验证密钥开头（例如检查是否是 sk- 格式）
keychain-cli load --format bash | grep "ANTHROPIC_AUTH_TOKEN" | head -c 50
```

### Q10: 如何处理包含特殊字符的密钥值？

```bash
# 密钥值可能包含：@#$%^&*()'"\等特殊字符
# keychain-cli 会自动转义

# 验证转义是否正确：
keychain-cli load --format bash | cat -v

# 在代码中安全使用：
# Python: os.environ.get('KEY')
# Go: os.Getenv("KEY")
# Node.js: process.env.KEY

# 这些都会正确处理转义后的值
```

### Q11: 如何添加新的安全规则（不需要重新编译）？

```bash
# 方案 A: 编辑配置文件 ~/.keychain/rules.json
vim ~/.keychain/rules.json

# 示例：添加一个新规则来拦截 mysqldump
# {
#   "id": "custom_mysql_dump",
#   "type": "substring",
#   "pattern": "mysqldump",
#   "description": "Block database export",
#   "enabled": true
# }

# 保存后立即生效，无需重新编译！
keychain-cli validate "mysqldump -u root -p"  # 应该被拦截

# 方案 B: 使用环境变量进行临时测试
export KEYCHAIN_CUSTOM_RULES="pattern1|pattern2|pattern3"
keychain-cli validate "pattern1 test command"  # 应该被拦截

# 取消临时规则
unset KEYCHAIN_CUSTOM_RULES
```

### Q12: 什么是规则类型，如何选择？

```bash
# 类型 1: substring - 简单子字符串匹配（推荐）
# 匹配：命令包含该字符串（不区分大小写）
# 适合：简单关键词（如 ".env", "mysqldump", "password"）
{
  "id": "rule1",
  "type": "substring",
  "pattern": "mysqldump"
}

# 类型 2: contains_all - 必须包含所有指定的模式
# 匹配：命令同时包含所有指定的关键词
# 适合：复合条件（如 "docker" 且 "compose" 且 "config"）
{
  "id": "rule2",
  "type": "contains_all",
  "patterns": ["docker", "compose", "config"]
}

# 类型 3: contains_any - 包含任意一个指定的模式
# 匹配：命令包含任何一个指定的关键词
# 适合：多个同义词（如 "password" 或 "pwd" 或 "pass"）
{
  "id": "rule3",
  "type": "contains_any",
  "patterns": ["password", "pwd", "pass"]
}
```

### Q13: 如何调试规则是否有效？

```bash
# 测试规则是否真的被拦截
keychain-cli validate "your test command"
echo "Exit code: $?"  # 0 = 允许, 2 = 被拦截

# 启用详细日志查看匹配了哪个规则
keychain-cli validate "test command" --verbose

# 临时启用一个规则进行测试
export KEYCHAIN_CUSTOM_RULES="test_pattern"
keychain-cli validate "test_pattern command"
unset KEYCHAIN_CUSTOM_RULES
```

### Q14: 如何管理规则版本和备份？

```bash
# 备份当前规则
cp ~/.keychain/rules.json ~/.keychain/rules.json.backup-$(date +%Y%m%d-%H%M%S)

# 查看规则修改历史（如果用 Git 跟踪）
git log -p ~/.keychain/rules.json

# 恢复到某个版本
cp ~/.keychain/rules.json.backup-20260219-101500 ~/.keychain/rules.json

# 查看当前活跃的规则数量
keychain-cli validate --help  # 会显示加载了多少规则

# 禁用某个规则以排查问题
vim ~/.keychain/rules.json
# 将该规则的 "enabled": true 改为 "enabled": false
```

---

## 日常维护清单

### 周维护
```bash
# 检查 Keychain 状态
keychain-cli check --verbose

# 验证所有密钥都能正常加载
keychain-cli load --format bash | wc -l  # 应该显示 ~61

# 测试 Hook 规则
keychain-cli validate "cat .env"  # 应该返回 exit code 2
keychain-cli validate "ls src/"   # 应该返回 exit code 0

# 检查规则配置文件完整性
[ -f ~/.keychain/rules.json ] && echo "✓ rules.json 存在" || echo "⚠ rules.json 缺失"

# 验证规则配置文件格式是否正确
cat ~/.keychain/rules.json | jq . > /dev/null && echo "✓ JSON 格式正确" || echo "✗ JSON 格式错误"
```

### 月维护
```bash
# 检查 State File 和 Keychain 同步
cat ~/.keychain/claude-dev.keys | wc -l
keychain-cli check --verbose  # 对比两者的密钥数

# 验证 .env 文件的完整性
wc -l /Volumes/otherdata/mac/claudecode-safe/credentials/.env

# 审查最近的访问日志
keychain-cli load --verbose 2>&1 | tail -20
```

### 季度维护
```bash
# 审计内置规则（检查是否有新的威胁需要添加）
# 查看 src/rules/mod.rs 中的硬编码规则列表
# 考虑是否需要添加新的规则来防守新的威胁

# 审计自定义规则（检查是否还有用）
cat ~/.keychain/rules.json | jq '.rules[] | select(.enabled==true)' | head -20

# 验证所有密钥仍然有效
keychain-cli load --format bash > /tmp/keys-backup.txt

# 检查是否有密钥需要 rotate
grep "DEPRECATED\|EXPIRED\|OLD" ~/.keychain/claude-dev.keys

# 检查是否有已禁用的规则可以删除
cat ~/.keychain/rules.json | jq '.rules[] | select(.enabled==false)' | grep '"id"'
```

### 年度维护
```bash
# 更新文档（README.md 和 MAINTENANCE.md）
# 评估是否需要新增安全规则
# 考虑将 State File 备份到安全位置
tar -czf ~/.keychain/claude-dev.backup.tar.gz ~/.keychain/

# 安全审查：检查代码是否有安全漏洞
cargo audit  # 检查依赖项漏洞

# 规则审计：检查所有规则是否合适
# 1. 查看规则列表
cat ~/.keychain/rules.json | jq '.rules | length'  # 应该显示当前规则总数

# 2. 检查是否有重复的规则 ID
cat ~/.keychain/rules.json | jq '[.rules[].id] | .[as $arr | keys[] | if $arr[.] == $arr[. + 1] then . else empty end]'

# 3. 清理无用的已禁用规则
# 若某个规则已禁用很久且没有重新启用，可考虑删除它
vim ~/.keychain/rules.json

# 性能审计
time keychain-cli load > /dev/null  # 应该 <1 秒
time keychain-cli validate "test" > /dev/null  # 应该 <10ms
```

---

## 故障排查决策树

```
密钥无法加载？
├─ 症状：echo $ANTHROPIC_AUTH_TOKEN 为空
│  ├─ Step 1: eval "$(keychain-cli load --format export)"
│  ├─ Step 2: echo $ANTHROPIC_AUTH_TOKEN
│  └─ 如果仍为空：
│     ├─ keychain-cli check --verbose
│     ├─ cat ~/.keychain/claude-dev.keys | wc -l
│     └─ which keychain-cli
│
├─ 症状：keychain-cli load 失败
│  ├─ 检查 Keychain 状态：keychain-cli check
│  ├─ 检查 State File：ls -la ~/.keychain/claude-dev.keys
│  └─ 重新初始化：
│     └─ keychain-cli setup --env-file /Volumes/otherdata/mac/claudecode-safe/credentials/.env --force
│
└─ 症状：生物识别验证失败
   ├─ 使用密码重试
   ├─ 重启 Mac
   └─ 重新配置 Touch ID

Hook 阻止合法命令？
├─ 症状：命令被意外拦截
│  ├─ 测试：keychain-cli validate "your command"
│  ├─ 如果返回 2：命令被识别为危险
│  └─ 解决：
│     ├─ 方案 A（临时）：在 settings.json 中注释 Hook
│     └─ 方案 B（永久）：编辑 validate.rs 更新规则
│
└─ 症状：某些合法命令通过了但不应该通过
   └─ 添加新的危险模式到 dangerous_patterns 列表

密钥值包含特殊字符？
├─ 症状：引号、单引号、$符号等导致错误
│  └─ keychain-cli 会自动转义
│     └─ 在代码中正常使用 os.environ.get() 即可
│
└─ 症状：Shell 脚本中密钥展开异常
   └─ 确保使用双引号：
      export KEY="${VALUE}"  # 而不是 KEY=$VALUE
```

---

## 灾难恢复方案

### 场景：所有密钥丢失

```bash
# 恢复步骤（需要原始 .env 文件）
# 前提条件：/Volumes/otherdata/mac/claudecode-safe/credentials/.env 文件完整

# 1. 清空所有
security delete-generic-password -a claude-dev
rm -f ~/.keychain/claude-dev.keys

# 2. 重新初始化
keychain-cli setup --env-file /Volumes/otherdata/mac/claudecode-safe/credentials/.env --force

# 3. 验证恢复
keychain-cli check --verbose
keychain-cli load | wc -l  # 应该显示 61 行
```

### 场景：State File 损坏或丢失

```bash
# State File 可以从 Keychain 重建
# 但目前没有自动化方式，需要手动：

# 1. 删除损坏的 State File
rm -f ~/.keychain/claude-dev.keys

# 2. 重新运行 setup（会生成新的 State File）
keychain-cli setup --env-file /Volumes/otherdata/mac/claudecode-safe/credentials/.env --force

# 或者从 Keychain 手动恢复（如果你知道所有密钥名）
# 这需要编写小脚本
```

### 场景：keychain-cli 二进制损坏或丢失

```bash
# 如果 /usr/local/bin/keychain-cli 无法执行

# 1. 检查状态
ls -la /usr/local/bin/keychain-cli
file /usr/local/bin/keychain-cli

# 2. 重新安装
cp /Users/houzi/code/06-production-business-money-live/keychain/target/release/keychain-cli /usr/local/bin/

# 3. 验证权限
chmod +x /usr/local/bin/keychain-cli
which keychain-cli  # 应该显示 /usr/local/bin/keychain-cli

# 4. 重新编译（如果 binary 不存在）
cd /Users/houzi/code/06-production-business-money-live/keychain
cargo build --release
cp target/release/keychain-cli /usr/local/bin/
```

---

## 性能优化

### 当前性能基准

```bash
# 测试加载 61 个密钥的时间
time keychain-cli load > /dev/null

# 预期结果
# real    0m0.950s
# user    0m0.120s
# sys     0m0.080s
```

### 优化建议

1. **缓存 Keychain 密钥**（考虑添加）
   - 可以创建内存缓存减少 Keychain 访问
   - 权衡：更快 vs. 更安全

2. **部分加载**
   ```bash
   # 只加载需要的密钥，而不是全部
   keychain-cli load --keys "ANTHROPIC_AUTH_TOKEN,MYSQL_PASSWORD" --format bash
   ```

3. **并行加载**（考虑实现）
   - 当前是顺序加载 61 个密钥
   - 可以并行化以提高速度

---

## 联系方式

遇到问题？参考以下资源：

1. **README.md** - 快速开始和命令参考
2. **MAINTENANCE.md** - 本文件，维护和故障排查
3. **src/commands/validate.rs** - Hook 规则定义
4. **~/.claude/CLAUDE.md** - 全局安全规范
5. **start-claude.sh** - 启动脚本示例

