pub mod enforce_namespace;
pub mod disallow_debug_functions;
pub mod line_length;
pub mod single_class_per_file;
pub mod class_member_order;

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
