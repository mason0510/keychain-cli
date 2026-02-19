use log::debug;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// 单个安全规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    #[serde(rename = "type")]
    pub rule_type: RuleType,
    pub description: String,
    pub enabled: bool,
}

/// 规则类型：定义如何检查命令
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RuleType {
    /// 简单的子字符串匹配（不区分大小写）
    Substring { pattern: String },

    /// 必须包含所有指定的模式
    ContainsAll { patterns: Vec<String> },

    /// 必须包含至少一个指定的模式
    ContainsAny { patterns: Vec<String> },
}

impl Rule {
    /// 检查命令是否匹配此规则
    pub fn check(&self, command: &str) -> bool {
        if !self.enabled {
            return false;
        }

        let cmd_lower = command.to_lowercase();

        match &self.rule_type {
            RuleType::Substring { pattern } => {
                let pattern_lower = pattern.to_lowercase();
                cmd_lower.contains(&pattern_lower)
            }
            RuleType::ContainsAll { patterns } => {
                patterns.iter().all(|p| {
                    let p_lower = p.to_lowercase();
                    cmd_lower.contains(&p_lower)
                })
            }
            RuleType::ContainsAny { patterns } => {
                patterns.iter().any(|p| {
                    let p_lower = p.to_lowercase();
                    cmd_lower.contains(&p_lower)
                })
            }
        }
    }
}

/// 规则引擎：管理和执行所有规则
pub struct RuleEngine {
    rules: Vec<Rule>,
}

impl RuleEngine {
    /// 创建一个新的规则引擎，加载所有可用规则
    pub fn new() -> Self {
        let mut rules = Vec::new();

        // L1: 硬编码的内置规则
        debug!("Loading built-in rules");
        rules.extend(Self::load_builtin_rules());

        // L2: 从配置文件加载规则 (~/.keychain/rules.json)
        debug!("Loading rules from configuration file");
        match Self::load_config_rules() {
            Ok(config_rules) => {
                debug!("Loaded {} rules from config file", config_rules.len());
                rules.extend(config_rules);
            }
            Err(e) => debug!("No custom config rules loaded: {}", e),
        }

        // L3: 从环境变量加载临时规则 ($KEYCHAIN_CUSTOM_RULES)
        debug!("Loading rules from environment variables");
        match Self::load_env_rules() {
            Ok(env_rules) => {
                debug!("Loaded {} rules from environment", env_rules.len());
                rules.extend(env_rules);
            }
            Err(e) => debug!("No environment rules loaded: {}", e),
        }

        debug!("Rule engine initialized with {} total rules", rules.len());
        RuleEngine { rules }
    }

    /// 检查命令是否危险
    pub fn is_dangerous(&self, command: &str) -> bool {
        for rule in &self.rules {
            if rule.check(command) {
                debug!("Command matched rule: {} ({})", rule.id, rule.description);
                return true;
            }
        }
        false
    }

    /// 获取所有活跃规则的数量
    pub fn active_rules_count(&self) -> usize {
        self.rules.iter().filter(|r| r.enabled).count()
    }

    /// L1: 硬编码的内置规则 (Turing 推荐的混合方案)
    fn load_builtin_rules() -> Vec<Rule> {
        vec![
            // ========== .env 文件访问 ==========
            Rule {
                id: "env_file_access".to_string(),
                rule_type: RuleType::Substring {
                    pattern: ".env".to_string(),
                },
                description: "Block access to .env files".to_string(),
                enabled: true,
            },

            // ========== Docker Compose 配置 ==========
            Rule {
                id: "docker_compose_config".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec![
                        "docker".to_string(),
                        "compose".to_string(),
                        "config".to_string(),
                    ],
                },
                description: "Block docker compose config access".to_string(),
                enabled: true,
            },
            Rule {
                id: "docker_hyphen_compose_config".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec![
                        "docker-compose".to_string(),
                        "config".to_string(),
                    ],
                },
                description: "Block docker-compose config access".to_string(),
                enabled: true,
            },

            // ========== macOS security 命令 ==========
            Rule {
                id: "security_find_generic".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec![
                        "security".to_string(),
                        "find-generic".to_string(),
                    ],
                },
                description: "Block security find-generic (keychain access)".to_string(),
                enabled: true,
            },
            Rule {
                id: "security_find_internet".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec![
                        "security".to_string(),
                        "find-internet".to_string(),
                    ],
                },
                description: "Block security find-internet (keychain access)".to_string(),
                enabled: true,
            },
            Rule {
                id: "security_get_keychain".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec![
                        "security".to_string(),
                        "get-keychain".to_string(),
                    ],
                },
                description: "Block security get-keychain".to_string(),
                enabled: true,
            },

            // ========== 敏感目录访问 ==========
            Rule {
                id: "volumes_keys_access".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["/Volumes".to_string(), "keys".to_string()],
                },
                description: "Block access to /Volumes/.../keys".to_string(),
                enabled: true,
            },
            Rule {
                id: "volumes_secret_access".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["/Volumes".to_string(), "secret".to_string()],
                },
                description: "Block access to /Volumes/.../secret".to_string(),
                enabled: true,
            },
            Rule {
                id: "volumes_password_access".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["/Volumes".to_string(), "password".to_string()],
                },
                description: "Block access to /Volumes/.../password".to_string(),
                enabled: true,
            },
            Rule {
                id: "volumes_credential_access".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["/Volumes".to_string(), "credential".to_string()],
                },
                description: "Block access to /Volumes/.../credential".to_string(),
                enabled: true,
            },

            // ========== 敏感搜索 ==========
            Rule {
                id: "grep_password".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["grep".to_string(), "password".to_string()],
                },
                description: "Block grep for password patterns".to_string(),
                enabled: true,
            },
            Rule {
                id: "grep_secret".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["grep".to_string(), "secret".to_string()],
                },
                description: "Block grep for secret patterns".to_string(),
                enabled: true,
            },
            Rule {
                id: "grep_key".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["grep".to_string(), "key".to_string()],
                },
                description: "Block grep for key patterns".to_string(),
                enabled: true,
            },
            Rule {
                id: "grep_token".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["grep".to_string(), "token".to_string()],
                },
                description: "Block grep for token patterns".to_string(),
                enabled: true,
            },
            Rule {
                id: "grep_api_key".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["grep".to_string(), "api_key".to_string()],
                },
                description: "Block grep for api_key patterns".to_string(),
                enabled: true,
            },

            // ========== SSH/AWS 配置 ==========
            Rule {
                id: "ssh_dir_access".to_string(),
                rule_type: RuleType::Substring {
                    pattern: "/.ssh/".to_string(),
                },
                description: "Block access to ~/.ssh directory".to_string(),
                enabled: true,
            },
            Rule {
                id: "aws_dir_access".to_string(),
                rule_type: RuleType::Substring {
                    pattern: "/.aws/".to_string(),
                },
                description: "Block access to ~/.aws directory".to_string(),
                enabled: true,
            },

            // ========== Shell 历史 ==========
            Rule {
                id: "bash_history".to_string(),
                rule_type: RuleType::Substring {
                    pattern: ".bash_history".to_string(),
                },
                description: "Block access to .bash_history".to_string(),
                enabled: true,
            },
            Rule {
                id: "zsh_history".to_string(),
                rule_type: RuleType::Substring {
                    pattern: ".zsh_history".to_string(),
                },
                description: "Block access to .zsh_history".to_string(),
                enabled: true,
            },

            // ========== 敏感数据库操作 ==========
            Rule {
                id: "mysqldump".to_string(),
                rule_type: RuleType::Substring {
                    pattern: "mysqldump".to_string(),
                },
                description: "Block mysqldump (database export)".to_string(),
                enabled: true,
            },
            Rule {
                id: "pg_dump".to_string(),
                rule_type: RuleType::Substring {
                    pattern: "pg_dump".to_string(),
                },
                description: "Block pg_dump (PostgreSQL export)".to_string(),
                enabled: true,
            },
            Rule {
                id: "redis_cli_keys".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["redis-cli".to_string(), "keys".to_string()],
                },
                description: "Block redis-cli keys (Redis inspection)".to_string(),
                enabled: true,
            },

            // ========== Git 敏感信息 ==========
            Rule {
                id: "git_config_get".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["git".to_string(), "config".to_string(), "get".to_string()],
                },
                description: "Block git config get (credential access)".to_string(),
                enabled: true,
            },

            // ========== find 命令查找敏感文件 ==========
            Rule {
                id: "find_password".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["find".to_string(), "password".to_string()],
                },
                description: "Block find for password files".to_string(),
                enabled: true,
            },
            Rule {
                id: "find_secret".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["find".to_string(), "secret".to_string()],
                },
                description: "Block find for secret files".to_string(),
                enabled: true,
            },
            Rule {
                id: "find_key".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["find".to_string(), "key".to_string()],
                },
                description: "Block find for key files".to_string(),
                enabled: true,
            },

            // ========== 其他敏感操作 ==========
            Rule {
                id: "cat_env".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["cat".to_string(), ".env".to_string()],
                },
                description: "Block cat .env".to_string(),
                enabled: true,
            },
            Rule {
                id: "ls_ssh".to_string(),
                rule_type: RuleType::ContainsAll {
                    patterns: vec!["ls".to_string(), "/.ssh".to_string()],
                },
                description: "Block ls ~/.ssh".to_string(),
                enabled: true,
            },
        ]
    }

    /// L2: 从配置文件加载规则 (~/.keychain/rules.json)
    fn load_config_rules() -> Result<Vec<Rule>, String> {
        let config_path = PathBuf::from(
            shellexpand::tilde("~/.keychain/rules.json")
                .as_ref(),
        );

        if !config_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read rules.json: {}", e))?;

        let config: ConfigFile = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse rules.json: {}", e))?;

        Ok(config.rules)
    }

    /// L3: 从环境变量加载临时规则 ($KEYCHAIN_CUSTOM_RULES)
    /// 格式: pattern1|pattern2|pattern3
    fn load_env_rules() -> Result<Vec<Rule>, String> {
        match std::env::var("KEYCHAIN_CUSTOM_RULES") {
            Ok(env_rules) => {
                let rules = env_rules
                    .split('|')
                    .enumerate()
                    .filter(|(_, pattern)| !pattern.trim().is_empty())
                    .map(|(i, pattern)| Rule {
                        id: format!("env_custom_{}", i),
                        rule_type: RuleType::Substring {
                            pattern: pattern.trim().to_string(),
                        },
                        description: format!("Custom rule from env: {}", pattern),
                        enabled: true,
                    })
                    .collect();
                Ok(rules)
            }
            Err(_) => Ok(Vec::new()),
        }
    }
}

/// 配置文件格式
#[derive(Debug, Deserialize)]
struct ConfigFile {
    rules: Vec<Rule>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substring_rule() {
        let rule = Rule {
            id: "test".to_string(),
            rule_type: RuleType::Substring {
                pattern: ".env".to_string(),
            },
            description: "Test".to_string(),
            enabled: true,
        };

        assert!(rule.check("cat .env"));
        assert!(rule.check("cat deploy/.env"));
        assert!(!rule.check("cat .env.example"));
    }

    #[test]
    fn test_contains_all_rule() {
        let rule = Rule {
            id: "test".to_string(),
            rule_type: RuleType::ContainsAll {
                patterns: vec![
                    "docker".to_string(),
                    "compose".to_string(),
                    "config".to_string(),
                ],
            },
            description: "Test".to_string(),
            enabled: true,
        };

        assert!(rule.check("docker compose config"));
        assert!(rule.check("docker  compose  config")); // Multiple spaces
        assert!(!rule.check("docker compose build"));
        assert!(!rule.check("docker config"));
    }

    #[test]
    fn test_case_insensitive() {
        let rule = Rule {
            id: "test".to_string(),
            rule_type: RuleType::Substring {
                pattern: "PASSWORD".to_string(),
            },
            description: "Test".to_string(),
            enabled: true,
        };

        assert!(rule.check("grep PASSWORD file.txt"));
        assert!(rule.check("grep password file.txt"));
        assert!(rule.check("grep PaSsWoRd file.txt"));
    }

    #[test]
    fn test_disabled_rule() {
        let rule = Rule {
            id: "test".to_string(),
            rule_type: RuleType::Substring {
                pattern: ".env".to_string(),
            },
            description: "Test".to_string(),
            enabled: false,
        };

        assert!(!rule.check("cat .env"));
    }
}
