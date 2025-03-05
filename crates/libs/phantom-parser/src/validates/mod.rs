pub mod class_member_order;
pub mod disallow_debug_functions;
pub mod enforce_namespace;
pub mod line_length;
pub mod single_class_per_file;

use chumsky::{error::Rich, input::Emitter};
use logos::Lexer;

use crate::{config::RuleParams, Statement, Token};

pub struct Content<'a> {
    pub source: Option<&'a str>,
    pub tokens: Option<Box<Lexer<'a, Token<'a>>>>,
    pub statements: Option<Box<Vec<Statement<'a>>>>,
}

pub trait RuleValidator {
    fn run(&self, contents: &Content, params: RuleParams, emitter: &mut Emitter<Rich<Token>>);
}
