use chumsky::span::SimpleSpan;
use logos::Logos;

pub mod rich;

pub type Span = SimpleSpan;
pub type Spanned<T> = (T, Span);

#[derive(Debug, Logos, PartialEq, Clone)]
pub enum Token<'a> {
    #[regex(r"[ \t]+", callback = |lex| {
        let slice = lex.slice();
        if slice.len() == 1 {
            Token::Whitespace
        } else {
            Token::Tab(slice.len())
        }
    })]
    Whitespace,
    Tab(usize),

    #[regex(r"\n")]
    Newline,

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

    #[token("if")]
    If,

    #[token("else")]
    Else,

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

#[derive(Debug, PartialEq, Clone)]
pub enum Statement<'a> {
    Empty,
    Namespace {
        kind: &'a str,
        span: Span,
        path: Vec<&'a str>,
        with_brackets: bool,
        body: Vec<Statement<'a>>,
    },

    Property {
        kind: &'a str,
        span: Span,
        name: &'a str,
        value: Option<Expr<'a>>,
        visibility: &'a str,
    },

    Method {
        kind: &'a str,
        span: Span,
        name: &'a str,
        is_static: bool,
        visibility: &'a str,
        args: Vec<Statement<'a>>,
        body: Vec<Expr<'a>>,
    },

    Parameter {
        kind: &'a str,
        name: &'a str,
        typed: Option<&'a str>,
        value: Option<Expr<'a>>,
    },

    // Return {
    //     expr: Expr<'a>,
    // },
    Class {
        kind: &'a str,
        span: Span,
        is_final: bool,
        is_abstract: bool,
        extends: Option<&'a str>,
        name: &'a str,
        body: Vec<Statement<'a>>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program<'a> {
    pub kind: &'a str,
    pub span: Span,
    pub statements: Vec<Statement<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'a> {
    Local(&'a str),
    Error {
        span: Span,
        target: &'a str,
    },
    List(Vec<Self>),
    Value {
        kind: &'a str,
        type_: Type,
        value: &'a str,
    },
    Variable(&'a str),
    Property(&'a str),
    Binary {
        kind: &'a str,
        span: Span,
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
    If {
        cond: Box<Self>,
        then: Box<Self>,
        else_: Box<Option<Self>>,
    },
    Call {
        kind: &'a str,
        span: Span,
        func: Box<Self>,
        args: Box<Vec<Self>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Null,
    Bool,
    Num,
    Str,
    // List(Vec<Self>),
    // Func(&'a str),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
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
