use chumsky::{error::Rich, span::SimpleSpan};

use crate::Token;

#[derive(Debug, Clone)]
pub struct LintError {
    pub message: String,
    pub level: String,
    pub span: SimpleSpan,
}

impl<'a> From<LintError> for Rich<'a, Token<'a>> {
    fn from(value: LintError) -> Self {
        Rich::custom(value.span, format!("[{}] {}", value.level, value.message))
    }
}
