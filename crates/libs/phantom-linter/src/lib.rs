use factory::{Extra, RuleFactory};

use phantom_config::Config;
use phantom_core::{rich::RichError, token::Token, Program, Span};

mod factory;
mod validates;

pub fn validate<'a>(
    source: &'a str,
    tokens: Box<Vec<(Token<'a>, Span)>>,
    program: Box<Program<'a>>,
    config: &Config,
    errors: Vec<RichError<'a, Token<'a>>>,
) -> Vec<RichError<'a, Token<'a>>> {
    let mut errs = errors.clone();

    config.rules.iter().for_each(|(name, params)| {
        let rule = RuleFactory::get(&name);

        let extra = Box::new(Extra {
            source,
            program: program.as_ref().clone(),
            tokens: tokens.iter().map(|(token, _)| token.clone()).collect(),
        });

        rule.run(params.clone(), &mut errs, extra.clone());
    });

    errs
}
