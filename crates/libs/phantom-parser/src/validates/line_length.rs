pub struct LineLength;

use chumsky::{error::Rich, input::Emitter, span::SimpleSpan};

use crate::{config::RuleParams, err::LintError, Statement, Token};

use super::RuleValidator;

impl RuleValidator for LineLength {
    fn run(
        &self,
        _tokens: &Vec<(Token, SimpleSpan)>,
        _statements: &Vec<Statement>,
        _params: RuleParams,
        emitter: &mut Emitter<Rich<Token>>,
    ) {
        // TODO: Implement line length validation
    }
}
