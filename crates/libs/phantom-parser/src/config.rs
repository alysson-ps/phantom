use chumsky::span::SimpleSpan;
use serde::Deserialize;
use serde_json::Value;
use std::{any::Any, collections::HashMap, fmt::Debug, fs};

use crate::{err::rich::RichError, factory::RuleFactory, Program, Token};

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
    tokens: &'a Box<Vec<(Token<'a>, SimpleSpan)>>,
    program: &'a Box<Program<'a>>,
    config: &Config,
    errors: &'a mut Vec<RichError<Token<'a>>>,
) {
    config.rules.iter().for_each(|(name, params)| {
        // let tokens_ref = &mut tokens.clone();
        // let mut contents = Box::new(Content {
        //     source,
        //     tokens: tokens_ref,
        //     program: program.clone(),
        // });

        let rule = RuleFactory::get("single-class-per-file");

        dbg!(rule.run(params.clone(), errors))

        // if let Some(rule) = RuleFactory::new().get_rule(name) {
        //     let extra = match rule.name() {
        //         "single_class_per_file" => Some(program),
        //         _ => None,
        //     };

        //     rule.run(params.clone(), errors, extra);
        // }
    });
}
