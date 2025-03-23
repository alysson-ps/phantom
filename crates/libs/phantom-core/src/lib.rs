use chumsky::span::SimpleSpan;
pub mod rich;
pub mod token;

pub type Span = SimpleSpan;
pub type Spanned<T> = (T, Span);
pub type Lexer<'a, T> = logos::Lexer<'a, T>;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Rule {
    LineLength,
    DisallowDebugFunctions,
    ClassMemberOrder,
    EnforceNamespace,
    SingleClassPerFile,
    TooManyConsecutiveNewlines,
}
