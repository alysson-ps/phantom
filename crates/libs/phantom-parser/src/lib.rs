use chumsky::{
    error::Rich,
    extra::Err,
    input::{Input, Stream, ValueInput},
    prelude::{end, just},
    recursive, select,
    span::SimpleSpan,
    IterParser, Parser,
};
use logos::Logos;
use serde_json::json;

pub type Span = SimpleSpan;
pub type Spanned<T> = (T, Span);

#[derive(Debug, Logos, PartialEq, Clone)]
pub enum Token<'a> {
    #[regex(r"[ \t\f]+", logos::skip)]
    Whitespace,

    #[regex(r"\n+", |lex| lex.slice().len())]
    Newline(usize),

    #[regex(r"//[^\n]*")]
    SingleLineComment,

    #[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/")]
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

    #[regex(r"(\$[a-zA-Z_][a-zA-Z0-9_]*)")]
    Variable,

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

    #[token("=")]
    Equals,

    Error,
}

#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Namespace {
        path: Vec<&'a str>,
    },
    Class {
        is_final: bool,
        name: String,
        properties: Vec<&'a str>,
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

fn statement<'a, I>() -> impl Parser<'a, I, Statement<'a>, Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    let new_line = select! { Token::Newline(n) => n }
        .map_with(|count, e| (count, e.span()))
        .validate(|(count, span), _, emmiter| {
            if count > 2 {
                emmiter.emit(Rich::custom(span, "Too many new lines"));
            }
        });

    let base_path = select! { Token::Identifier(name) => name };
    let path = base_path.separated_by(just(Token::Backslash)).collect().labelled("Path");

    let namespace = just(Token::Namespace)
        .ignore_then(path)
        .then_ignore(just(Token::Semicolon))
        .map(|path| Statement::Namespace { path });

    namespace.padded_by(new_line)
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
