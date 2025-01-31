use chumsky::{
    error::Rich,
    extra::Err,
    input::{Input, Stream, ValueInput},
    prelude::{
        any, choice, just, nested_delimiters, one_of, recursive, skip_then_retry_until, via_parser,
    },
    select,
    span::SimpleSpan,
    IterParser, Parser,
};
use logos::Logos;

pub type Span = SimpleSpan;
pub type Spanned<T> = (T, Span);

#[derive(Debug, Logos, PartialEq, Clone)]
pub enum Token<'a> {
    #[regex(r"[ \t\f]+", logos::skip)]
    Whitespace,

    #[regex(r"\n+", |lex| lex.slice().len())]
    Newline(usize),

    #[regex(r"//[^\n]*", logos::skip)]
    SingleLineComment,

    #[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/", logos::skip)]
    MiltLineComment,

    #[token("\\")]
    Backslash,

    #[token("<?php")]
    OpenTag,

    #[token("?>")]
    CloseTag,

    #[token("namespace")]
    Namespace,

    #[token("final")]
    Final,

    #[token("class")]
    Class,

    #[token("abstract")]
    Abstract,

    #[token("static")]
    Static,

    #[token("function")]
    Function,

    #[token("public")]
    #[token("private")]
    #[token("protected")]
    Visibility(&'a str),

    #[regex(r"(\$[a-zA-Z_][a-zA-Z0-9_]*)", |lex| &lex.slice()[1..])]
    Variable(&'a str),

    #[token("null")]
    Null,

    #[regex(r"[0-9]+")]
    Number(&'a str),

    #[regex(r#""([^"\\]|\\.)*""#)]
    String(&'a str),

    #[token("true")]
    #[token("false")]
    Bool(&'a str),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier(&'a str),

    #[token(";")]
    Semicolon,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token(",")]
    Comma,

    #[token("return")]
    Return,

    #[token("=")]
    Assign,

    #[token("==")]
    Eq,

    #[token("!=")]
    NotEq,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token(".")]
    Concat,

    #[token("->")]
    Arrow,

    #[token("=>")]
    DoubleArrow,

    Error,
}

#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Namespace {
        path: Vec<&'a str>,
        with_brackets: bool,
        body: Vec<Statement<'a>>,
    },

    Property {
        name: &'a str,
        value: &'a str,
        visibility: &'a str,
    },

    Method {
        name: &'a str,
        is_static: bool,
        visibility: &'a str,
        args: Vec<Statement<'a>>,
        body: Vec<Expr<'a>>,
    },

    Parameter {
        name: &'a str,
        typed: Option<&'a str>,
        value: Option<Expr<'a>>,
    },

    // Return {
    //     expr: Expr<'a>,
    // },
    Class {
        is_final: bool,
        is_abstract: bool,
        extends: Option<&'a str>,
        name: &'a str,
        body: Vec<Statement<'a>>,
    },
}

#[derive(Debug, PartialEq)]
struct Program<'a> {
    kind: &'a str,
    span: Span,
    statements: Vec<Statement<'a>>,
}

#[derive(Debug, PartialEq)]
enum Expr<'a> {
    Error,
    List(Vec<Self>),
    Value(Value<'a>),
    Variable(&'a str),
    Property(&'a str),
    Number(i64),
    Binary {
        left: Box<Self>,
        op: BinaryOp,
        right: Box<Self>,
    },
    Assignment {
        target: Box<Self>,
        value: Box<Self>,
    },
    Return {
        expr: Box<Option<Self>>,
    },
    Call(Box<Self>, Vec<Self>),
}

#[derive(Clone, Debug, PartialEq)]
enum Value<'a> {
    Null,
    Bool(&'a str),
    Num(i64),
    Str(&'a str),
    List(Vec<Self>),
    Func(&'a str),
}

#[derive(Clone, Debug, PartialEq)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    And,
    Or,
    Concat,
}

fn parser<'a, I>() -> impl Parser<'a, I, Program<'a>, Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    just(Token::OpenTag)
        .ignore_then(statement().repeated().collect())
        .then_ignore(just(Token::CloseTag).or_not())
        .map_with(|statements, e| Program {
            kind: "Program",
            span: e.span(),
            statements,
        })
}

// fn expr_parser<'a, I>(
// ) -> impl Parser<'a, I, Spanned<Expr<'a>>, Err<Rich<'a, Token<'a>, Span>>> + Clone
// where
//     I: ValueInput<'a, Token = Token<'a>, Span = Span>,
// {
// }

fn statement<'a, I>() -> impl Parser<'a, I, Statement<'a>, Err<Rich<'a, Token<'a>, Span>>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    recursive(|statement| {
        // let expr_inline = recursive(|expr|{
        //     let value = select! {
        //         Token::Null => Expr::Value(Value::Null)
        //     }.labelled("Value");

        //     value.as_context()
        // });

        let new_line = select! { Token::Newline(n) => n }
            .map_with(|count, e| (count, e.span()))
            .validate(|(count, span), _, emitter| {
                if count > 2 {
                    emitter.emit(Rich::custom(span, "Too many new lines"));
                }
            });

        let base_path = select! { Token::Identifier(name) => name };
        let path = base_path.separated_by(just(Token::Backslash)).collect().labelled("Path");

        let namespace_without_brackets = just(Token::Namespace)
            .ignore_then(path.clone())
            .then_ignore(just(Token::Semicolon))
            .then(
                statement.clone().repeated().collect::<Vec<_>>().try_map(|body, span| {
                    if body.iter().any(|stmt| matches!(stmt, Statement::Namespace { .. })) {
                        Err(Rich::custom(
                            span,
                            "Nested namespaces are not allowed in this context.",
                        ))
                    } else {
                        Ok(body)
                    }
                }),
            )
            .map(|(path, body)| Statement::Namespace {
                path,
                with_brackets: false,
                body,
            });

        let namespace_with_brackets = just(Token::Namespace)
            .ignore_then(path.clone())
            .then(
                statement
                    .clone()
                    .repeated()
                    .collect::<Vec<_>>()
                    .delimited_by(just(Token::LBrace), just(Token::RBrace)),
            )
            .map(|(path, body)| Statement::Namespace {
                path,
                with_brackets: true,
                body,
            });

        let namespace = namespace_with_brackets.or(namespace_without_brackets);

        let property = select! {Token::Visibility(visibility) => visibility}
            .then(select! { Token::Variable(name) => name })
            .then_ignore(just(Token::Semicolon))
            .map(|(visibility, name)| Statement::Property {
                name,
                value: "",
                visibility,
            });

        let class = just(Token::Class)
            .ignore_then(select! { Token::Identifier(name) => name })
            .then(
                statement
                    .clone()
                    .repeated()
                    .collect::<Vec<_>>()
                    .delimited_by(just(Token::LBrace), just(Token::RBrace)),
            )
            .map(|(name, body)| Statement::Class {
                is_final: false,
                is_abstract: false,
                extends: None,
                name,
                body,
            });

        let _arg = select! {Token::Variable(name) => name}
            .then_ignore(just(Token::Assign).or_not())
            .then(expression().or_not());

        let args = _arg
            .separated_by(just(Token::Comma))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(just(Token::LParen), just(Token::RParen))
            .map(|names| {
                names
                    .into_iter()
                    .map(|(name, val)| Statement::Parameter {
                        name,
                        typed: None,
                        value: val,
                    })
                    .collect::<Vec<_>>()
            })
            .labelled("Args");

        let function = select! { Token::Visibility(visibility) => visibility }
            .labelled("Visibility")
            .then(just(Token::Static).or_not())
            .then_ignore(just(Token::Function))
            .then(select! { Token::Identifier(name) => name })
            .then(args)
            .then(
                expression()
                    .repeated()
                    .collect::<Vec<_>>()
                    .delimited_by(just(Token::LBrace), just(Token::RBrace)),
            )
            .map_with(
                |((((visibility, is_static), name), args), body), _| Statement::Method {
                    name,
                    is_static: is_static.is_some(),
                    visibility,
                    args,
                    body,
                },
            );

        choice((class, function, namespace, property)).padded_by(new_line.repeated())
    })
}

fn expression<'a, I>() -> impl Parser<'a, I, Expr<'a>, Err<Rich<'a, Token<'a>, Span>>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    recursive(|expr| {
        let inline_expr = recursive(|inline_expr| {
            let val = select! {
                Token::Null => Expr::Value(Value::Null),
                Token::Bool(x) => Expr::Value(Value::Bool(x)),
                Token::Number(n) => Expr::Value(Value::Num(n.parse().unwrap())),
                Token::String(s) => Expr::Value(Value::Str(s)),
            }
            .labelled("value");

            // let ident = select! { Token::Identifier(ident) => ident };

            // A list of expressions
            let items =
                expr.clone().separated_by(just(Token::Comma)).allow_trailing().collect::<Vec<_>>();

            let variable = select! {Token::Variable(name) => name}.map(|name| Expr::Variable(name));

            let _var = variable
                .then_ignore(just(Token::Assign))
                .then(inline_expr.clone())
                .then_ignore(just(Token::Semicolon))
                .map(|(var, val)| Expr::Assignment {
                    target: Box::new(var),
                    value: Box::new(val),
                });

            let property = select! {Token::Variable(name) => name}
                .then_ignore(just(Token::Arrow))
                .then(select! {Token::Identifier(name) => name})
                .then_ignore(just(Token::Assign))
                .then(inline_expr.clone())
                .map(|((_, name), val)| Expr::Assignment {
                    target: Box::new(Expr::Property(name)),
                    value: Box::new(val),
                });

            let list = items
                .clone()
                .map(Expr::List)
                .delimited_by(just(Token::LBrace), just(Token::RBrace));

            // 'Atoms' are expressions that contain no ambiguity
            let atom = val
                // .or(ident.map(Expr::Local))
                .or(_var)
                .or(property)
                .or(variable)
                .or(list)
                // In Nano Rust, `print` is just a keyword, just like Python 2, for simplicity
                // .or(just(Token::Print)
                // .ignore_then(
                //     expr.clone().delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
                // )
                // .map(|expr| Expr::Print(Box::new(expr))))
                .map_with(|expr, _span| expr)
                // Atoms can also just be normal expressions, but surrounded with parentheses
                .or(expr.clone().delimited_by(just(Token::LParen), just(Token::RParen)))
                // Attempt to recover anything that looks like a parenthesised expression but contains errors
                .recover_with(via_parser(nested_delimiters(
                    Token::LParen,
                    Token::RParen,
                    [
                        (Token::LBrace, Token::RBrace),
                        (Token::LBracket, Token::RBracket),
                    ],
                    |span| Expr::Error,
                )))
                // Attempt to recover anything that looks like a list but contains errors
                .recover_with(via_parser(nested_delimiters(
                    Token::LBrace,
                    Token::RBrace,
                    [
                        (Token::LParen, Token::RParen),
                        (Token::LBracket, Token::RBracket),
                    ],
                    |span| Expr::Error,
                )))
                .boxed();

            // Function calls have very high precedence so we prioritise them
            let call = atom.foldl_with(
                items
                    .delimited_by(just(Token::LParen), just(Token::RParen))
                    .map_with(|args, e| args)
                    .repeated(),
                |f, args, e| {
                    dbg!(&f);
                    dbg!(&args);
                    Expr::Call(Box::new(f), args)
                },
            );

            // Product ops (multiply and divide) have equal precedence
            let op =
                just(Token::Asterisk).to(BinaryOp::Mul).or(just(Token::Slash).to(BinaryOp::Div));
            let product =
                call.clone().foldl_with(op.then(call).repeated(), |a, (op, b), e| Expr::Binary {
                    left: Box::new(a),
                    op,
                    right: Box::new(b),
                });

            // Sum ops (add and subtract) have equal precedence
            let op = just(Token::Plus).to(BinaryOp::Add).or(just(Token::Minus).to(BinaryOp::Sub));
            let sum = product.clone().foldl_with(op.then(product).repeated(), |a, (op, b), e| {
                Expr::Binary {
                    left: Box::new(a),
                    op,
                    right: Box::new(b),
                }
            });

            // Comparison ops (equal, not-equal) have equal precedence
            let op = just(Token::Eq).to(BinaryOp::Eq).or(just(Token::NotEq).to(BinaryOp::NotEq));
            let compare =
                sum.clone().foldl_with(op.then(sum).repeated(), |a, (op, b), e| Expr::Binary {
                    left: Box::new(a),
                    op,
                    right: Box::new(b),
                });

            let op = just(Token::Concat).to(BinaryOp::Concat);
            let concat =
                compare.clone().foldl_with(op.then(compare).repeated(), |a, (op, b), e| {
                    Expr::Binary {
                        left: Box::new(a),
                        op,
                        right: Box::new(b),
                    }
                });

            // Logical ops (and, or) have equal precedence
            let op = just(Token::And).to(BinaryOp::And).or(just(Token::Or).to(BinaryOp::Or));
            let logic = concat.clone().foldl_with(op.then(concat).repeated(), |a, (op, b), e| {
                Expr::Binary {
                    left: Box::new(a),
                    op,
                    right: Box::new(b),
                }
            });

            logic.labelled("expression").as_context()
        });

        let new_line = select! { Token::Newline(n) => n }
            .map_with(|count, e| (count, e.span()))
            .validate(|(count, span), _, emitter| {
                if count > 2 {
                    emitter.emit(Rich::custom(span, "Too many new lines"));
                }
            });

        let _return = just(Token::Return)
            .ignore_then(inline_expr.clone().or_not())
            .then_ignore(just(Token::Semicolon))
            .map(|ret_expr| Expr::Return {
                expr: Box::new(ret_expr),
            });

        // let block = choice((
        //     _return,
        //     inline_expr.clone().then_ignore(just(Token::Semicolon)),
        // ))
        // .padded_by(new_line.repeated())
        // .delimited_by(just(Token::LBrace), just(Token::RBrace));

        let block_recovery = nested_delimiters(
            Token::LBracket,
            Token::RBracket,
            [
                (Token::LParen, Token::RParen),
                (Token::LBrace, Token::RBrace),
            ],
            |span| Expr::Error,
        );

        let block = expr
            .clone()
            .delimited_by(just(Token::LBrace), just(Token::RBrace))
            .recover_with(via_parser(nested_delimiters(
                Token::LBracket,
                Token::RBracket,
                [
                    (Token::LParen, Token::RParen),
                    (Token::LBrace, Token::RBrace),
                ],
                |span| Expr::Error,
            )));

        let block_expr = block.or(_return);

        let block_chain = block_expr.clone();

        block_chain.or(inline_expr).padded_by(new_line.repeated()).recover_with(
            skip_then_retry_until(
                block_recovery.ignored().or(any().ignored()),
                one_of([
                    Token::RBrace,
                    Token::RParen,
                    Token::RBracket,
                    Token::Semicolon,
                ])
                .ignored(),
            ),
        )
    })
}

pub fn parse(source: &str) {
    let token_iter = Token::lexer(source).spanned().map(|(tok, span)| match tok {
        // Turn the `Range<usize>` spans logos gives us into chumsky's `SimpleSpan` via `Into`, because it's easier
        // to work with
        Ok(tok) => (tok, SimpleSpan::from(span)),
        Err(()) => (Token::Error, SimpleSpan::from(span)),
    });

    let tokens: Vec<_> = token_iter.clone().collect();

    dbg!(&tokens);

    let token_stream =
        Stream::from_iter(token_iter).map((0..source.len()).into(), |(t, s): (_, _)| (t, s.into()));

    let result = parser().parse(token_stream).into_output_errors();
    // let result =
    //     expression().repeated().collect::<Vec<_>>().parse(token_stream).into_output_errors();

    dbg!(&result);
}
