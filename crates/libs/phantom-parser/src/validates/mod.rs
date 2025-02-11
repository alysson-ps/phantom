pub mod class_member_order;
pub mod disallow_debug_functions;
pub mod enforce_namespace;
pub mod line_length;
pub mod single_class_per_file;

use chumsky::{error::Rich, input::Emitter, span::SimpleSpan};

use crate::{config::RuleParams, err::LintError, Statement, Token};

pub trait RuleValidator {
    fn run(
        &self,
        tokens: &Vec<(Token, SimpleSpan)>,
        statements: &Vec<Statement>,
        params: RuleParams,
        emitter: &mut Emitter<Rich<Token>>,
    );
}
