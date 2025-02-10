pub struct ClassMemberOrder;

use chumsky::{error::Rich, input::Emitter};

use crate::{config::RuleParams, Statement, Token};

use super::RuleValidator;

impl RuleValidator for ClassMemberOrder {
    fn run(
        &self,
        statements: &Vec<Statement>,
        _params: RuleParams,
        emitter: &mut Emitter<Rich<Token>>,
    ) {
        // TODO: Implement class member order validation
    }
}
