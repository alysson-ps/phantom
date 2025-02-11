use chumsky::error::Rich;

use crate::Token;

#[derive(Debug, Clone)]
pub struct LintError<'a> {
    pub rich: Rich<'a, Token<'a>>,
    pub level: LintLevel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LintLevel {
    Error,
    Warning,
}

impl<'a> From<Rich<'a, Token<'a>>> for LintError<'a> {
    fn from(rich: Rich<'a, Token<'a>>) -> Self {
        LintError {
            rich,
            level: LintLevel::Error,
        }
    }
}
