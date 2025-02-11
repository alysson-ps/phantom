pub struct SingleClassPerFile;

use chumsky::{error::Rich, input::Emitter, span::SimpleSpan};

use crate::{config::RuleParams, err::LintError, Statement, Token};

use super::RuleValidator;

impl RuleValidator for SingleClassPerFile {
    fn run(
        &self,
        _tokens: &Vec<(Token, SimpleSpan)>,
        statements: &Vec<Statement>,
        _params: RuleParams,
        emitter: &mut Emitter<Rich<Token>>,
    ) {
        // TODO: implement single class per file validation
    }
}
