use std::{collections::HashMap, fs};

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub logs: LogsConfig,
    pub imports: ImportsConfig,
    pub rules: HashMap<String, RuleParams>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct LogsConfig {
    pub level: String,
    pub file: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ImportsConfig {
    pub organize: OrganizeConfig,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct OrganizeConfig {
    pub enabled: bool,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct RuleParams(pub String, pub Option<Value>);

pub fn load(path: &str) -> Result<Config, String> {
    let config_content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config file: {}", e.to_string()))?;

    let config: Config = serde_json::from_str(&config_content)
        .map_err(|err| format!("Failed to parse config file: {}", err.to_string()))?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logs_test() {
        let source = r#"
        {
            "level": "info",
            "file": "logs/app.log"
        }"#;

        let config: LogsConfig = serde_json::from_str(source).unwrap();
        assert_eq!(config.level, "info");
        assert_eq!(config.file, "logs/app.log");
    }

    #[test]
    fn imports_test() {
        let source = r#"
        {
            "organize": {
                "enabled": true
            }
        }"#;

        let config: ImportsConfig = serde_json::from_str(source).unwrap();
        assert_eq!(config.organize.enabled, true);
    }

    #[test]
    fn config_test() {
        let source = r#"
        {
            "logs": {
                "level": "info",
                "file": "logs/app.log"
            },
            "imports": {
                "organize": {
                    "enabled": true
                }
            },
            "rules": {
                "rule1": ["param1", null],
                "rule2": ["param2", {"key": "value"}]
            }
        }"#;

        let config: Config = serde_json::from_str(source).unwrap();
        assert_eq!(config.logs.level, "info");
        assert_eq!(config.logs.file, "logs/app.log");
        assert_eq!(config.imports.organize.enabled, true);
        assert_eq!(config.rules.get("rule1").unwrap().0, "param1");
        assert_eq!(config.rules.get("rule1").unwrap().1, None);
        assert_eq!(config.rules.get("rule2").unwrap().0, "param2");
        assert_eq!(
            config.rules.get("rule2").unwrap().1,
            Some(serde_json::json!({"key": "value"}))
        );
    }

    // TODO: load function test
}
