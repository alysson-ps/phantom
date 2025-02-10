pub mod enforce_namespace;

use chumsky::{error::Rich, input::Emitter};

use crate::{config::RuleParams, Statement, Token};

pub trait RuleValidator {
    fn run(
        &self,
        statements: &Vec<Statement>,
        params: RuleParams,
        emitter: &mut Emitter<Rich<Token>>,
    );
}
