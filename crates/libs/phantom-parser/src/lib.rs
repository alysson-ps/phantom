use chumsky::{
    error::Rich,
    extra::Err,
    input::{Input, Stream, ValueInput},
    prelude::{end, just},
    select,
    span::SimpleSpan,
    Parser,
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
pub enum AstNode<'a> {
    Namespace {
        path: Vec<&'a str>,
    },
    Class {
        is_final: bool,
        name: String,
        properties: Vec<&'a str>,
    },
}

fn parser<'a, I>() -> impl Parser<'a, I, AstNode<'a>, Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    let namespace = just(Token::Namespace)
        .ignore_then(select! { Token::Identifier(name) => name  })
        .map(|name: &'a str| AstNode::Namespace { path: vec![name] });

    namespace.then_ignore(end())
}

pub fn parse(source: &str) {
    let token_iter = Token::lexer(source).spanned().map(|(tok, span)| match tok {
        // Turn the `Range<usize>` spans logos gives us into chumsky's `SimpleSpan` via `Into`, because it's easier
        // to work with
        Ok(tok) => (tok, SimpleSpan::from(span)),
        Err(()) => (Token::Error, SimpleSpan::from(span)),
    });

    let token_stream =
        Stream::from_iter(token_iter).map((0..source.len()).into(), |(t, s): (_, _)| (t, s.into()));

    let result = parser().parse(token_stream);

    dbg!(&result);
}
