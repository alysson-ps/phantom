use chumsky::{
    error::Simple,
    prelude::{
        end, filter, filter_map, just, nested_delimiters, none_of, one_of, recursive,
        skip_then_retry_until, take_until,
    },
    text::{self, TextParser},
    Error, Parser, Stream,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};
use tower_lsp::lsp_types::SemanticTokenType;

pub const LEGEND_TYPE: &[SemanticTokenType] = &[
    SemanticTokenType::FUNCTION,
    SemanticTokenType::VARIABLE,
    SemanticTokenType::STRING,
    SemanticTokenType::COMMENT,
    SemanticTokenType::NUMBER,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::PARAMETER,
];

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Null => write!(f, "null"),
            Token::Bool(x) => write!(f, "{}", x),
            Token::Num(n) => write!(f, "{}", n),
            Token::Str(s) => write!(f, "{}", s),
            Token::Op(s) => write!(f, "{}", s),
            Token::Ctrl(c) => write!(f, "{}", c),
            Token::Ident(s) => write!(f, "{}", s),
            Token::Fn => write!(f, "fn"),
            Token::Dollar => write!(f, "$"),
            Token::Echo => write!(f, "echo"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
        }
    }
}

#[derive(Debug)]
pub struct ParserResult {
    pub ast: Option<HashMap<String, Func>>, // Ajuste o tipo de acordo com seu AST
    pub parse_errors: Vec<Simple<String>>,
    pub semantic_tokens: Vec<ImCompleteSemanticToken>,
}

pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Op(String),
    Ctrl(char),
    Ident(String),
    Fn,
    If,
    Else,
    Dollar,

    Echo,
}

fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    // A parser for comments
    let simple_line = just::<_, _, Simple<char>>("//").then(none_of("\n").repeated()).ignored();

    let simple_line_with_sharp =
        just::<_, _, Simple<char>>("#").then(none_of("\n").repeated()).ignored();

    let multi_line = just::<_, _, Simple<char>>("/*").then(take_until(just("*/"))).ignored();

    let comment = simple_line.or(simple_line_with_sharp).or(multi_line);

    // A parser for numbers
    let num = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect()
        .map(Token::Num);

    // A parser for strings
    let str_double = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::Str);

    let str_single = just('\'')
        .ignore_then(filter(|c| *c != '\'').repeated())
        .then_ignore(just('\''))
        .collect::<String>()
        .map(Token::Str);

    let str_ = str_double.or(str_single);

    // A parser for identifiers and keywords
    let ident = text::ident().map(|ident: String| match ident.as_str() {
        "fn" => Token::Fn,
        "echo" => Token::Echo,
        "if" => Token::If,
        "else" => Token::Else,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "null" => Token::Null,
        _ => Token::Ident(ident),
    });

    // A parser for operators
    let op = one_of("+-*/!=<>").repeated().at_least(1).collect::<String>().map(Token::Op);

    // A parser for variables
    let dollar = one_of("$").repeated().at_least(1).collect::<String>().map(|_| Token::Dollar);

    // A parser for control characters
    let ctrl = one_of("(){}[],:?|;.").map(Token::Ctrl);

    // A parser for main tokens
    let token = num
        .or(str_)
        .or(op)
        .or(ctrl)
        .or(dollar)
        .or(ident)
        .recover_with(skip_then_retry_until([]));

    token
        .padded_by(comment.padded().repeated())
        .map_with_span(|tok, span| (tok, span))
        .padded()
        .repeated()
}

pub type Spanned<T> = (T, Span);

#[derive(Debug)]
pub struct Func {
    pub args: Vec<Spanned<String>>,
    pub body: Spanned<Expr>,
    pub name: Spanned<String>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    List(Vec<Value>),
    Func(String),
}

#[derive(Debug)]
pub enum Expr {
    Error,
    Value(Value),
    List(Vec<Spanned<Self>>),
    Local(Spanned<String>),

    Var(String, Box<Spanned<Self>>, Box<Spanned<Self>>, Span),

    Then(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Binary(Box<Spanned<Self>>, BinaryOp, Box<Spanned<Self>>),
    Call(Box<Spanned<Self>>, Spanned<Vec<Spanned<Self>>>),
    If(Box<Spanned<Self>>, Box<Spanned<Self>>, Box<Spanned<Self>>),

    Echo(Box<Spanned<Self>>),
}

pub fn funcs_parser() -> impl Parser<Token, HashMap<String, Func>, Error = Simple<Token>> + Clone {
    let ident = filter_map(|span, tok| match tok {
        Token::Ident(ident) => Ok(ident),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
    });

    let args = ident
        .map_with_span(|name, span| (name, span))
        .separated_by(just(Token::Ctrl(',')))
        .allow_trailing()
        .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')')))
        .labelled("function args");

    let func = just(Token::Fn)
        .ignore_then(ident.map_with_span(|name, span| (name, span)).labelled("function name"))
        .then(args)
        .then(
            expr_parser()
                .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')))
                // Attempt to recover anything that looks like a function body but contains errors
                .recover_with(nested_delimiters(
                    Token::Ctrl('{'),
                    Token::Ctrl('}'),
                    [
                        (Token::Ctrl('('), Token::Ctrl(')')),
                        (Token::Ctrl('['), Token::Ctrl(']')),
                    ],
                    |span| (Expr::Error, span),
                )),
        )
        .map_with_span(|((name, args), body), span| {
            (
                name.clone(),
                Func {
                    args,
                    body,
                    name,
                    span,
                },
            )
        })
        .labelled("function");

    func.repeated()
        .try_map(|fs, _| {
            let mut funcs = HashMap::new();
            for ((name, name_span), f) in fs {
                if funcs.insert(name.clone(), f).is_some() {
                    return Err(Simple::custom(
                        name_span,
                        format!("Function '{}' already exists", name),
                    ));
                }
            }
            Ok(funcs)
        })
        .then_ignore(end())
}

fn expr_parser() -> impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone {
    recursive(|expr| {
        let raw_expr = recursive(|raw| {
            let val = filter_map(|span, token| match token {
                Token::Null => Ok(Expr::Value(Value::Null)),
                Token::Bool(b) => Ok(Expr::Value(Value::Bool(b))),
                Token::Num(num) => Ok(Expr::Value(Value::Num(num.parse().unwrap()))),
                Token::Str(str) => Ok(Expr::Value(Value::Str(str))),
                _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token))),
            })
            .labelled("value");

            let id = filter_map(|span, token| match token {
                Token::Ident(ident) => Ok((ident, span)),
                _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token))),
            })
            .labelled("identifier");

            // A list of expressions
            let items = expr
                .clone()
                .chain(just(Token::Ctrl(',')).ignore_then(expr.clone()).repeated())
                .then_ignore(just(Token::Ctrl(',')).or_not())
                .or_not()
                .map(|item| item.unwrap_or_default());

            // A Variable expression
            let var_ = just(Token::Dollar)
                .ignore_then(id)
                .then_ignore(just(Token::Op("=".to_string())))
                .then(raw)
                .then_ignore(just(Token::Ctrl(';')))
                .then(expr.clone())
                .map(|((name, val), body)| {
                    Expr::Var(name.0, Box::new(val), Box::new(body), name.1)
                });

            let array = items
                .clone()
                .delimited_by(just(Token::Ctrl('[')), just(Token::Ctrl(']')))
                .map(Expr::List);

            // 'Atoms' are expressions that contain no ambiguity
            let atom = val
                .or(id.map(Expr::Local))
                .or(var_)
                .or(array)
                // In Nano Rust, `print` is just a keyword, just like Python 2, for simplicity
                .or(just(Token::Echo)
                    .ignore_then(
                        expr.clone().delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
                    )
                    .map(|expr| Expr::Echo(Box::new(expr))))
                .map_with_span(|expr, span| (expr, span))
                // Atoms can also just be normal expressions, but surrounded with parentheses
                .or(expr.clone().delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))))
                // Attempt to recover anything that looks like a parenthesised expression but contains errors
                .recover_with(nested_delimiters(
                    Token::Ctrl('('),
                    Token::Ctrl(')'),
                    [
                        (Token::Ctrl('['), Token::Ctrl(']')),
                        (Token::Ctrl('{'), Token::Ctrl('}')),
                    ],
                    |span| (Expr::Error, span),
                ))
                // Attempt to recover anything that looks like a list but contains errors
                .recover_with(nested_delimiters(
                    Token::Ctrl('['),
                    Token::Ctrl(']'),
                    [
                        (Token::Ctrl('('), Token::Ctrl(')')),
                        (Token::Ctrl('{'), Token::Ctrl('}')),
                    ],
                    |span| (Expr::Error, span),
                ));

            // Function calls have very high precedence so we prioritise them
            let call = atom
                .then(
                    items
                        .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')')))
                        .map_with_span(|args, span| (args, span))
                        .repeated(),
                )
                .foldl(|f, args| {
                    let span = f.1.start..args.1.end;
                    (Expr::Call(Box::new(f), args), span)
                });

            // Product ops (multiply and divide) have equal precedence
            let op = just(Token::Op("*".to_string()))
                .to(BinaryOp::Mul)
                .or(just(Token::Op("/".to_string())).to(BinaryOp::Div));
            let product = call.clone().then(op.then(call).repeated()).foldl(|a, (op, b)| {
                let span = a.1.start..b.1.end;
                (Expr::Binary(Box::new(a), op, Box::new(b)), span)
            });

            // Sum ops (add and subtract) have equal precedence
            let op = just(Token::Op("+".to_string()))
                .to(BinaryOp::Add)
                .or(just(Token::Op("-".to_string())).to(BinaryOp::Sub));
            let sum = product.clone().then(op.then(product).repeated()).foldl(|a, (op, b)| {
                let span = a.1.start..b.1.end;
                (Expr::Binary(Box::new(a), op, Box::new(b)), span)
            });

            // Comparison ops (equal, not-equal) have equal precedence
            let op = just(Token::Op("==".to_string()))
                .to(BinaryOp::Eq)
                .or(just(Token::Op("!=".to_string())).to(BinaryOp::NotEq));

            sum.clone().then(op.then(sum).repeated()).foldl(|a, (op, b)| {
                let span = a.1.start..b.1.end;
                (Expr::Binary(Box::new(a), op, Box::new(b)), span)
            })
        });

        let block = expr
            .clone()
            .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')))
            // Attempt to recover anything that looks like a block but contains errors
            .recover_with(nested_delimiters(
                Token::Ctrl('{'),
                Token::Ctrl('}'),
                [
                    (Token::Ctrl('('), Token::Ctrl(')')),
                    (Token::Ctrl('['), Token::Ctrl(']')),
                ],
                |span| (Expr::Error, span),
            ));

        let if_ = recursive(|if_| {
            just(Token::If)
                .ignore_then(expr.clone())
                .then(block.clone())
                .then(just(Token::Else).ignore_then(block.clone().or(if_)).or_not())
                .map_with_span(|((cond, a), b), span| {
                    (
                        Expr::If(
                            Box::new(cond),
                            Box::new(a),
                            Box::new(match b {
                                Some(b) => b,
                                // If an `if` expression has no trailing `else` block, we magic up one that just produces null
                                None => (Expr::Value(Value::Null), span.clone()),
                            }),
                        ),
                        span,
                    )
                })
        });

        let block_expr = block.or(if_).labelled("block");

        let block_chain = block_expr.clone().then(block_expr.clone().repeated()).foldl(|a, b| {
            let span = a.1.start..b.1.end;
            (Expr::Then(Box::new(a), Box::new(b)), span)
        });

        block_chain
            // Expressions, chained by semicolons, are statements
            .or(raw_expr.clone())
            .then(just(Token::Ctrl(';')).ignore_then(expr.or_not()).repeated())
            .foldl(|a, b| {
                let span = a.1.clone(); // TODO: Not correct
                (
                    Expr::Then(
                        Box::new(a),
                        Box::new(match b {
                            Some(b) => b,
                            None => (Expr::Value(Value::Null), span.clone()),
                        }),
                    ),
                    span,
                )
            })
    })
}

#[derive(Debug)]
pub struct ImCompleteSemanticToken {
    pub start: usize,
    pub length: usize,
    pub token_type: usize,
    pub debug: Option<String>
}

pub fn parser(src: &str) -> ParserResult {
    let (tokens, errors) = lexer().parse_recovery(src);

    let (ast, tokenize_errors, semantic_tokens) = if let Some(tokens) = tokens {
        let semantic_tokens = tokens
            .iter()
            .filter_map(|(token, span)| match token {
                Token::Null => None,
                Token::Bool(_) => None,

                Token::Num(_) => Some(ImCompleteSemanticToken {
                    start: span.start,
                    length: span.len(),
                    token_type: LEGEND_TYPE
                        .into_iter()
                        .position(|t| t == &SemanticTokenType::NUMBER)
                        .unwrap(),
                    debug: Some("Number".to_string())
                }),

                Token::Str(_) => Some(ImCompleteSemanticToken {
                    start: span.start,
                    length: span.len(),
                    token_type: LEGEND_TYPE
                        .into_iter()
                        .position(|t| t == &SemanticTokenType::STRING)
                        .unwrap(),
                        debug: Some("String".to_string())
                }),

                Token::Op(_) => Some(ImCompleteSemanticToken {
                    start: span.start,
                    length: span.len(),
                    token_type: LEGEND_TYPE
                        .into_iter()
                        .position(|t| t == &SemanticTokenType::OPERATOR)
                        .unwrap(),
                        debug: Some("Operator".to_string())
                }),

                Token::Dollar => Some(ImCompleteSemanticToken {
                    start: span.start,
                    length: span.len(),
                    token_type: LEGEND_TYPE
                        .into_iter()
                        .position(|t| t == &SemanticTokenType::KEYWORD)
                        .unwrap(),
                        debug: Some("Dollar".to_string())
                }),

                Token::Ctrl(_) => None,
                Token::Ident(_) => None,

                Token::Fn => Some(ImCompleteSemanticToken {
                    start: span.start,
                    length: span.len(),
                    token_type: LEGEND_TYPE
                        .into_iter()
                        .position(|t| t == &SemanticTokenType::KEYWORD)
                        .unwrap(),
                        debug: Some("Function".to_string())
                }),

                Token::Echo => Some(ImCompleteSemanticToken {
                    start: span.start,
                    length: span.len(),
                    token_type: LEGEND_TYPE
                        .into_iter()
                        .position(|t| t == &SemanticTokenType::FUNCTION)
                        .unwrap(),
                        debug: Some("Echo".to_string())
                }),

                Token::If => Some(ImCompleteSemanticToken {
                    start: span.start,
                    length: span.len(),
                    token_type: LEGEND_TYPE
                        .into_iter()
                        .position(|t| t == &SemanticTokenType::KEYWORD)
                        .unwrap(),
                        debug: Some("If".to_string())
                }),

                Token::Else => Some(ImCompleteSemanticToken {
                    start: span.start,
                    length: span.len(),
                    token_type: LEGEND_TYPE
                        .into_iter()
                        .position(|t| t == &SemanticTokenType::KEYWORD)
                        .unwrap(),
                        debug: Some("Else".to_string())
                }),

                // Token::Var(_) => Some(ImCompleteSemanticToken {
                //     start: span.start,
                //     length: span.len(),
                //     token_type: LEGEND_TYPE
                //         .into_iter()
                //         .position(|t| t == &SemanticTokenType::VARIABLE)
                //         .unwrap(),
                // }),
            })
            .collect::<Vec<_>>();

        let len = src.chars().count();

        let (_ast, _tokenize_errors) =
            funcs_parser().parse_recovery(Stream::from_iter(len..len + 1, tokens.into_iter()));

        (
            _ast,
            _tokenize_errors,
            semantic_tokens,
        )
    } else {
        (None, Vec::new(), Vec::new())
    };

    let parse_errors = errors
        .into_iter()
        .map(|e| e.map(|c| c.to_string()))
        .chain(
            tokenize_errors
                .into_iter()
                .map(|e| e.map(|tok| tok.to_string())),
        )
        .collect::<Vec<_>>();

    ParserResult {
        ast,
        parse_errors,
        semantic_tokens,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_variable_assignment() {
        let src = "$x = 5;";
        let result = lexer().parse(src).unwrap();

        assert_eq!(result.len(), 5);
        assert_eq!(result[0].0, Token::Dollar);
        assert_eq!(result[1].0, Token::Ident("x".to_string()));
        assert_eq!(result[2].0, Token::Op("=".to_string()));
        assert_eq!(result[3].0, Token::Num("5".to_string()));
        assert_eq!(result[4].0, Token::Ctrl(';'));
    }

    #[test]
    fn test_lexer_with_comment() {
        let src = "$x = 5; // This is a comment";
        let result = lexer().parse(src).unwrap();

        assert_eq!(result.len(), 5);
        assert_eq!(result[0].0, Token::Dollar);
        assert_eq!(result[1].0, Token::Ident("x".to_string()));
        assert_eq!(result[2].0, Token::Op("=".to_string()));
        assert_eq!(result[3].0, Token::Num("5".to_string()));
        assert_eq!(result[4].0, Token::Ctrl(';'));
    }
}
