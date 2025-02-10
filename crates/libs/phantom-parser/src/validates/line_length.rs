pub struct LineLength;

use chumsky::{error::Rich, input::Emitter};

use crate::{config::RuleParams, Statement, Token};

use super::RuleValidator;

impl RuleValidator for LineLength {
    fn run(
        &self,
        _statements: &Vec<Statement>,
        _params: RuleParams,
        _emitter: &mut Emitter<Rich<Token>>,
    ) {
        // TODO: Implement line length validation
    }
}
