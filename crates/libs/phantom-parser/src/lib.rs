use chumsky::{
    error::Rich,
    extra::Err,
    input::{Input, Stream, ValueInput},
    prelude::{choice, just, recursive},
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
    Equals,

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
        args: Vec<&'a str>,
        body: Vec<Statement<'a>>,
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

        let args = select! {Token::Variable(name) => name}
            .separated_by(just(Token::Comma))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(just(Token::LParen), just(Token::RParen))
            .labelled("Args");

        let function = select! { Token::Visibility(visibility) => visibility }
            .labelled("Visibility")
            .then(just(Token::Static).or_not())
            .then_ignore(just(Token::Function))
            .then(select! { Token::Identifier(name) => name })
            .then(args)
            .then(
                statement
                    .clone()
                    .repeated()
                    .collect()
                    .delimited_by(just(Token::LBrace), just(Token::RBrace)).or_not(),
            )
            .map_with(
                |((((visibility, is_static), name), args), body), _| Statement::Method {
                    name,
                    is_static: is_static.is_some(),
                    visibility,
                    args,
                    body: body.unwrap_or_default(),
                },
            );

        // let expr = recursive(|expr| {
        //     // Valores básicos
        //     let value = select! {
        //         Token::Null => Expr::Value(Value::Null),
        //         Token::Bool(value) => Expr::Value(Value::Bool(value)),
        //         Token::Number(value) => Expr::Value(Value::Num(value)),
        //         Token::String(value) => Expr::Value(Value::Str(value)),
        //     };

        //     // Atribuição
        //     let assign = select! { Token::Variable(name) => name }
        //         .then_ignore(just(Token::Equals))
        //         .then(expr.clone()) // Usando `expr` recursivamente
        //         .then_ignore(just(Token::Semicolon))
        //         .map(|(var, val)| Expr::Assign(var, Box::new(val))); // Retorno formatado

        //     // Combinação de parsers
        //     value.or(assign).map_with(|ex, e| (ex, e.span()))
        // });

        // // Parser para `return`
        // let _return = just(Token::Return)
        //     .ignore_then(expr.clone()) // O que está sendo retornado
        //     .then_ignore(just(Token::Semicolon)) // Ignorando o ponto e vírgula
        //     .map(|(ret_expr, span)| Statement::Return { expr: ret_expr }); // Retorno formatado

        choice((class, function, namespace, property)).padded_by(new_line.repeated())
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

    let result = parser().parse(token_stream).into_result();

    dbg!(&result);
}
