use chumsky::{error::Rich, span::SimpleSpan};

use crate::Token;

#[derive(Debug, Clone)]
pub struct LintError {
    pub message: String,
    pub level: String,
    pub span: SimpleSpan,
    pub fixable: bool,
}

impl<'a> From<LintError> for Rich<'a, Token<'a>> {
    fn from(value: LintError) -> Self {
        Rich::custom(value.span, format!("[{}] {}", value.level, value.message))
    }
}

use chumsky::{prelude::*, error::{Error, LabelError}, util::MaybeRef, DefaultExpected};
type Span = SimpleSpan<usize>;

// A custom error type
#[derive(Debug, PartialEq)]
pub enum MyError {
    ExpectedFound {
        span: Span,
        expected: Vec<DefaultExpected<'static, char>>,
        found: Option<char>,
    },
    NotADigit(Span, char),
}

impl<'a> Error<'a, &'a str> for MyError {
    fn merge(mut self, mut other: Self) -> Self {
        if let (Self::ExpectedFound { expected, .. }, Self::ExpectedFound { expected: expected_other, .. }) = (
            &mut self,
            &mut other,
        ) {
            expected.append(expected_other);
        }
        self
    }
}

impl<'a> LabelError<'a, &'a str, DefaultExpected<'a, char>> for MyError {
    fn expected_found<Iter: IntoIterator<Item = DefaultExpected<'a, char>>>(
        expected: Iter,
        found: Option<MaybeRef<'a, char>>,
        span: Span,
    ) -> Self {
        Self::ExpectedFound {
            span,
            expected: expected
                .into_iter()
                .map(|e| e.into_owned())
                .collect(),
            found: found.as_deref().copied(),
        }
    }
}