use chumsky::{error::Rich, input::Emitter, span::SimpleSpan};
use logos::Lexer;
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fs};

use crate::{factory::RuleFactory, validates::Content, Statement, Token};

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    // pub logs: LogsConfig,
    // pub imports: ImportsConfig,
    pub rules: HashMap<String, RuleParams>,
}

// #[derive(Debug, Deserialize)]
// pub(crate) struct LogsConfig {
//     pub level: String,
//     pub file: String,
// }

// #[derive(Debug, Deserialize)]
// pub(crate) struct ImportsConfig {
//     pub organize: OrganizeConfig,
// }

// #[derive(Debug, Deserialize)]
// pub(crate) struct OrganizeConfig {
//     pub enabled: bool,
// }

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct RuleParams(pub String, pub Option<Value>);

pub fn load_config(path: &str) -> Config {
    let config_content = fs::read_to_string(path).expect("Failed to read config file");

    let config: Config = serde_json::from_str(&config_content).expect("Invalid config format");

    // dbg!(&config);

    config
}

pub fn validate<'a>(
    source: &'a str,
    tokens: Box<Lexer<'a, Token<'a>>>,
    statements: Box<Vec<Statement<'a>>>,
    config: &Config,
    emitter: &mut Emitter<Rich<Token>>,
) {
    let contents = Content {
        source: Some(source),
        tokens: Some(tokens),
        statements: Some(statements),
    };

    config.rules.iter().for_each(|(name, params)| {
        if let Some(rule) = RuleFactory::new().get_rule(name) {
            rule.run(&contents, params.clone(), emitter);
        }
    });
}
