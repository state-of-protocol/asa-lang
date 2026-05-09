// PELAKSANAAN SEMENTARA: Pengkompil ini ditulis dalam Rust.
// Ia akan digantikan dengan pengkompil ASA yang ditulis dalam ASA
// sebaik sahaja bahasa ini matang. Lihat SUPREMACY.txt

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Keywords
    Fn,
    Biar,
    Jika,
    Lain,
    Untuk,
    Sementara,

    // Literals
    Ident(String),
    Int(i64),
    String(String),

    // Operators & Delimiters
    Plus, Minus, Star, Slash, Assign,
    LParen, RParen, LBrace, RBrace,
    Less, Greater, LessEqual, GreaterEqual,
    EqualEqual, NotEqual,
    DotDot, Comma, Semicolon, Dot, Colon,

    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Fn => write!(f, "fn"),
            TokenKind::Biar => write!(f, "biar"),
            TokenKind::Jika => write!(f, "jika"),
            TokenKind::Lain => write!(f, "lain"),
            TokenKind::Untuk => write!(f, "untuk"),
            TokenKind::Sementara => write!(f, "sementara"),
            TokenKind::Ident(s) => write!(f, "identifier({})", s),
            TokenKind::Int(n) => write!(f, "integer({})", n),
            TokenKind::String(s) => write!(f, "string({})", s),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::Less => write!(f, "<"),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::LessEqual => write!(f, "<="),
            TokenKind::GreaterEqual => write!(f, ">="),
            TokenKind::EqualEqual => write!(f, "=="),
            TokenKind::NotEqual => write!(f, "!="),
            TokenKind::DotDot => write!(f, ".."),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Eof => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, col: usize) -> Self {
        Token { kind, line, col }
    }
}
