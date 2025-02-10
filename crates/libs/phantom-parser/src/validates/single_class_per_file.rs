pub struct SingleClassPerFile;

use chumsky::{error::Rich, input::Emitter};

use crate::{config::RuleParams, Statement, Token};

use super::RuleValidator;

impl RuleValidator for SingleClassPerFile {
    fn run(
        &self,
        statements: &Vec<Statement>,
        _params: RuleParams,
        emitter: &mut Emitter<Rich<Token>>,
    ) {
        // TODO: implement single class per file validation
    }
}
