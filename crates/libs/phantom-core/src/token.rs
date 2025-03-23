use logos::Logos;

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

impl<'a> Token<'a> {
    pub fn code(&self) -> String {
        match self {
            Token::OpenTag => "<?php".to_string().to_string(),
            Token::Namespace => "namespace".to_string(),
            Token::Newline => "\n".to_string(),
            Token::CloseTag => "?>".to_string(),
            Token::Backslash => "\\".to_string(),
            Token::Final => "final".to_string(),
            Token::Class => "class".to_string(),
            Token::Abstract => "abstract".to_string(),
            Token::Static => "static".to_string(),
            Token::Function => "function".to_string(),
            Token::If => "if".to_string(),
            Token::Else => "else".to_string(),
            Token::Visibility(v) => v.to_string(),
            Token::Variable(v) => v.to_string(),
            Token::Null => "null".to_string(),
            Token::Number(n) => n.to_string(),
            Token::String(s) => s.to_string(),
            Token::Bool(b) => b.to_string(),
            Token::Identifier(i) => i.to_string(),
            Token::Semicolon => ";".to_string(),
            Token::LBrace => "{".to_string(),
            Token::RBrace => "}".to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::LBracket => "[".to_string(),
            Token::RBracket => "]".to_string(),
            Token::Comma => ",".to_string(),
            Token::Return => "return".to_string(),
            Token::Assign => "=".to_string(),
            Token::Eq => "==".to_string(),
            Token::NotEq => "!=".to_string(),
            Token::And => "&&".to_string(),
            Token::Or => "||".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Asterisk => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::Concat => ".".to_string(),
            Token::Arrow => "->".to_string(),
            Token::DoubleArrow => "=>".to_string(),
            Token::Whitespace => " ".to_string(),
            Token::Tab(count) => " ".repeat(*count).to_string(),
            Token::SingleLineComment => "//".to_string(),
            Token::MiltLineComment => "/* */".to_string(),

            Token::Error => "error".to_string(),
        }
    }
}
