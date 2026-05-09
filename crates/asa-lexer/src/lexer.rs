// PELAKSANAAN SEMENTARA: Pengkompil ini ditulis dalam Rust.
// Ia akan digantikan dengan pengkompil ASA yang ditulis dalam ASA
// sebaik sahaja bahasa ini matang. Lihat SUPREMACY.txt

use crate::token::{Token, TokenKind};

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            source: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        match self.source.get(self.pos) {
            Some(ch) => {
                self.pos += 1;
                if *ch == '\n' {
                    self.line += 1;
                    self.col = 1;
                } else {
                    self.col += 1;
                }
                Some(*ch)
            }
            None => None,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn is_ident_char(ch: char, first: bool) -> bool {
        if first {
            ch.is_alphabetic() || ch == '_'
        } else {
            ch.is_alphanumeric() || ch == '_'
        }
    }

    fn read_identifier(&mut self, first_char: char) -> TokenKind {
        let mut ident = String::new();
        ident.push(first_char);
        while let Some(ch) = self.peek() {
            if Self::is_ident_char(ch, false) {
                ident.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        match ident.as_str() {
            "fn" => TokenKind::Fn,
            "biar" => TokenKind::Biar,
            "jika" => TokenKind::Jika,
            "lain" => TokenKind::Lain,
            "untuk" => TokenKind::Untuk,
            "sementara" => TokenKind::Sementara,
            _ => TokenKind::Ident(ident),
        }
    }

    fn read_integer(&mut self, first_digit: char) -> Result<i64, String> {
        let mut number = String::new();
        number.push(first_digit);
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                number.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        number.parse::<i64>().map_err(|_| {
            format!("Ralat lekser pada {}:{} - literal integer tidak sah '{}'",
                    self.line, self.col, number)
        })
    }

    fn read_string(&mut self) -> Result<String, String> {
        self.advance();
        let mut content = String::new();
        loop {
            match self.advance() {
                Some('"') => break,
                Some('\n') => {
                    return Err(format!(
                        "Ralat lekser pada {}:{} - rentetan tidak tertutup",
                        self.line, self.col
                    ));
                }
                Some(ch) => content.push(ch),
                None => {
                    return Err(format!(
                        "Ralat lekser pada {}:{} - akhir fail dalam rentetan",
                        self.line, self.col
                    ));
                }
            }
        }
        Ok(content)
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            let line = self.line;
            let col = self.col;

            match self.peek() {
                None => break,
                Some(ch) => {
                    if Self::is_ident_char(ch, true) {
                        self.advance();
                        let kind = self.read_identifier(ch);
                        tokens.push(Token::new(kind, line, col));
                    } else if ch.is_ascii_digit() {
                        self.advance();
                        let val = self.read_integer(ch)?;
                        tokens.push(Token::new(TokenKind::Int(val), line, col));
                    } else {
                        let kind = match ch {
                            '+' => { self.advance(); TokenKind::Plus }
                            '-' => { self.advance(); TokenKind::Minus }
                            '*' => { self.advance(); TokenKind::Star }
                            '/' => { self.advance(); TokenKind::Slash }
                            '(' => { self.advance(); TokenKind::LParen }
                            ')' => { self.advance(); TokenKind::RParen }
                            '{' => { self.advance(); TokenKind::LBrace }
                            '}' => { self.advance(); TokenKind::RBrace }
                            ',' => { self.advance(); TokenKind::Comma }
                            ';' => { self.advance(); TokenKind::Semicolon }
                            '.' => {
                                self.advance();
                                if self.peek() == Some('.') {
                                    self.advance();
                                    TokenKind::DotDot
                                } else {
                                    TokenKind::Dot
                                }
                            }
                            ':' => { self.advance(); TokenKind::Colon }
                            '=' => {
                                self.advance();
                                if self.peek() == Some('=') {
                                    self.advance();
                                    TokenKind::EqualEqual
                                } else {
                                    TokenKind::Assign
                                }
                            }
                            '<' => {
                                self.advance();
                                if self.peek() == Some('=') {
                                    self.advance();
                                    TokenKind::LessEqual
                                } else {
                                    TokenKind::Less
                                }
                            }
                            '>' => {
                                self.advance();
                                if self.peek() == Some('=') {
                                    self.advance();
                                    TokenKind::GreaterEqual
                                } else {
                                    TokenKind::Greater
                                }
                            }
                            '!' => {
                                self.advance();
                                if self.peek() == Some('=') {
                                    self.advance();
                                    TokenKind::NotEqual
                                } else {
                                    return Err(format!(
                                        "Ralat lekser pada {}:{} - aksara tidak dijangka '!'",
                                        line, col
                                    ));
                                }
                            }
                            '"' => {
                                let s = self.read_string()?;
                                TokenKind::String(s)
                            }
                            _ => {
                                return Err(format!(
                                    "Ralat lekser pada {}:{} - aksara tidak dijangka '{}'",
                                    line, col, ch
                                ));
                            }
                        };
                        tokens.push(Token::new(kind, line, col));
                    }
                }
            }
        }
        tokens.push(Token::new(TokenKind::Eof, self.line, self.col));
        Ok(tokens)
    }
}
