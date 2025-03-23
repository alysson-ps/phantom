use phantom_core::{rich::RichError, token::Token, Program, Span};

#[derive(Debug)]
pub struct Event<'a> {
    pub path: &'a str,
    pub tokens: Vec<(Token<'a>, Span)>,
    pub ast: Program<'a>,
    pub errs: Vec<RichError<'a, Token<'a>>>,
}

pub fn run<'a>(event: Event<'a>) {
    event.errs.iter().for_each(|err| {
        let formatted = format(event.ast.clone(), event.tokens.clone(), err.clone());

        println!("Reconstructed: {}", formatted);
    
        // std::fs::write(event.path, formatted).unwrap();
    });
}

fn format<'a>(
    ast: Program<'a>,
    tokens: Vec<(Token<'a>, Span)>,
    err: RichError<'a, Token<'a>>,
) -> String {
    match err.fixer() {
        Some(phantom_core::Rule::LineLength) => "".to_string(),

        Some(phantom_core::Rule::DisallowDebugFunctions) => {
            let span = err.span();
            let start = span.start;
            let end = span.end;

            let mut new_tokens: Vec<Token<'a>> = Vec::new();

            for (_index, (token, span)) in tokens.iter().enumerate() {
                let start_token_delimiter = if span.end == start { Some(token) } else { None };

                let end_token_delimiter = if span.start == end { Some(token) } else { None };

                if span.start >= start && span.end <= end {
                    continue;
                }

                if matches!(start_token_delimiter, Some(Token::Tab(_))) {
                    continue;
                }

                if matches!(end_token_delimiter, Some(Token::Semicolon)) {
                    continue;
                }

                new_tokens.push(token.clone());
            }

            mount_code(new_tokens)
        }

        Some(phantom_core::Rule::TooManyConsecutiveNewlines) => {
            let span = err.span();
            let start = span.start;
            let end = span.end;

            let mut new_tokens: Vec<Token<'a>> = Vec::new();

            for (_index, (token, span)) in tokens.iter().enumerate() {
                if span.start >= start && span.end <= end {
                    continue;
                }

                new_tokens.push(token.clone());
            }

            mount_code(new_tokens)
        }

        Some(phantom_core::Rule::ClassMemberOrder(_order, order_span)) => {
            let _span = err.span();
            let start = _span.start;
            let end = _span.end;

            let mut top: Vec<Token<'a>> = Vec::new();
            let mut a: Vec<Token<'a>> = Vec::new();
            let mut between: Vec<Token<'a>> = Vec::new();
            let mut b: Vec<Token<'a>> = Vec::new();
            let mut bottom: Vec<Token<'a>> = Vec::new();

            for (_index, (token, span)) in tokens.iter().enumerate() {
                if span.start >= start && span.end <= end {
                    b.push(token.clone());
                } else if span.start >= order_span.start && span.end <= order_span.end {
                    a.push(token.clone());
                } else if span.start >= order_span.end && span.end <= start {
                    between.push(token.clone());
                } else if span.end < start {
                    top.push(token.clone());
                } else if span.start >= end {
                    bottom.push(token.clone());
                }
            }

            let new_tokens = vec![top, b, between, a, bottom].concat();

            mount_code(new_tokens)
        }

        _ => "".to_string(),
    }
}

fn mount_code<'a>(tokens: Vec<Token<'a>>) -> String {
    tokens.iter().map(|tok| tok.code().to_string()).collect::<Vec<_>>().join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mount_empty_tokens_test() {
        let tokens = vec![];
        let result = mount_code(tokens);
        assert_eq!(result, "");
    }

    #[test]
    fn mount_single_token_test() {
        let tokens = vec![Token::OpenTag];
        let result = mount_code(tokens);
        assert_eq!(result, "<?php");
    }

    #[test]
    fn mount_multiple_tokens_test() {
        let tokens = vec![Token::OpenTag, Token::Newline, Token::CloseTag];
        let result = mount_code(tokens);
        assert_eq!(result, "<?php\n?>");
    }

    #[test]
    fn format_to_many_consecutive_new_lines_test() {
        let tokens = vec![
            (Token::OpenTag, Span::new(0, 5)),
            (Token::Newline, Span::new(5, 6)),
            (Token::Newline, Span::new(6, 7)),
            (Token::Newline, Span::new(7, 8)),
            (Token::Newline, Span::new(8, 9)),
            (Token::CloseTag, Span::new(9, 11)),
        ];

        let err = RichError::custom(
            Span::new(7, 9),
            "error".to_string(),
            "Too many consecutive new lines".to_string(),
            Some(phantom_core::Rule::TooManyConsecutiveNewlines),
        );

        let formated = format(Program::default(), tokens, err);

        assert_eq!(formated, "<?php\n\n?>");
    }

    #[test]
    fn format_disallow_debug_functions_test() {
        let tokens = vec![
            (Token::OpenTag, Span::new(0, 5)),
            (Token::Newline, Span::new(5, 6)),
            (Token::Tab(4), Span::new(6, 7)),
            (Token::Identifier("var_dump"), Span::new(7, 15)),
            (Token::LParen, Span::new(15, 16)),
            (Token::Variable("$text"), Span::new(16, 21)),
            (Token::Comma, Span::new(21, 22)),
            (Token::Variable("$prefix"), Span::new(23, 30)),
            (Token::RParen, Span::new(30, 31)),
            (Token::Semicolon, Span::new(31, 32)),
            (Token::Newline, Span::new(32, 33)),
            (Token::CloseTag, Span::new(33, 35)),
        ];

        let err = RichError::custom(
            Span::new(7, 32),
            "error".to_string(),
            "Debug function var_dump is not allowed".to_string(),
            Some(phantom_core::Rule::DisallowDebugFunctions),
        );

        let formated = format(Program::default(), tokens, err);

        assert_eq!(formated, "<?php\n\n?>");
    }

    #[test]
    fn format_class_member_order_test() {
        let tokens = vec![];

        let err = RichError::custom(
            Span::new(0, 0),
            "error".to_string(),
            "Member \"test\" should be declared after properties".to_string(),
            Some(phantom_core::Rule::ClassMemberOrder(
                vec![
                    "constants".to_string(),
                    "properties".to_string(),
                    "methods".to_string(),
                ],
                Span::new(0, 0),
            )),
        );

        let formated = format(Program::default(), tokens, err);

        assert_eq!(formated, "");
    }
}
