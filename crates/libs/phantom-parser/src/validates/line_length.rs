pub struct LineLength;

use chumsky::{error::Rich, input::Emitter};

use crate::{config::RuleParams, Token};

use super::{Content, RuleValidator};

impl RuleValidator for LineLength {
    fn run(
        &self,
        _contents: &Content,
        _params: RuleParams,
        _emitter: &mut Emitter<Rich<Token>>,
    ) {
        // TODO: Implement line length validation
    }
}
