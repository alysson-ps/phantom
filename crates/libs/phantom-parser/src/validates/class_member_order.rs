pub struct ClassMemberOrder;

use chumsky::{error::Rich, input::Emitter, span::SimpleSpan};

use crate::{config::RuleParams, Statement, Token};

use super::{Content, RuleValidator};

impl RuleValidator for ClassMemberOrder {
    fn run(
        &self,
        _contents: &Content,
        _params: RuleParams,
        _emitter: &mut Emitter<Rich<Token>>,
    ) {
        // TODO: Implement class member order validation
    }
}
