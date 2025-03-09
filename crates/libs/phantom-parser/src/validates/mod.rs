pub mod class_member_order;
pub mod disallow_debug_functions;
pub mod enforce_namespace;
pub mod line_length;
pub mod single_class_per_file;

use chumsky::{input::Emitter, span::SimpleSpan};

use crate::{config::RuleParams, err::rich::RichError, Statement, Token};

pub struct Content<'a> {
    pub source: &'a str,
    pub tokens: &'a mut Box<Vec<(Token<'a>, SimpleSpan)>>,
    pub statements: Box<Vec<Statement<'a>>>,
}

pub trait RuleValidator {
    fn run<'a>(
        &self,
        contents: &'a mut Content<'a>,
        params: RuleParams,
        emitter: &mut Emitter<RichError<Token>>,
    );
}
