use factory::{Extra, RuleFactory};

use phantom_config::Config;
use phantom_core::{rich::RichError, Program, Span, Token};

mod factory;
mod validates;

pub fn validate<'a>(
    source: &'a str,
    tokens: &'a Box<Vec<(Token<'a>, Span)>>,
    program: &'a Box<Program<'a>>,
    config: &Config,
    errors: &'a mut Vec<RichError<Token<'a>>>,
) {
    config.rules.iter().for_each(|(name, params)| {
        let rule = RuleFactory::get(&name);

        let extra = Box::new(Extra {
            source,
            program: program.as_ref().clone(),
            tokens: tokens.iter().map(|(token, _)| token.clone()).collect(),
        });

        rule.run(params.clone(), errors, extra.clone());
        // rule.run(params.clone(), errors);
    });
}
