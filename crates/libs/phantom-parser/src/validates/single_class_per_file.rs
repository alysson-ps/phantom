pub struct SingleClassPerFile;

use chumsky::{error::Rich, input::Emitter};

use crate::{config::RuleParams, Token};

use super::{Content, RuleValidator};

impl RuleValidator for SingleClassPerFile {
    fn run(&self, _contents: &Content, _params: RuleParams, _emitter: &mut Emitter<Rich<Token>>) {
        // TODO: implement single class per file validation
    }
}
