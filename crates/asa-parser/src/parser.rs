// PELAKSANAAN SEMENTARA: Pengkompil ini ditulis dalam Rust.
// Ia akan digantikan dengan pengkompil ASA yang ditulis dalam ASA
// sebaik sahaja bahasa ini matang. Lihat SUPREMACY.txt

use asa_ast::*;
use asa_lexer::{Token, TokenKind};
use std::mem::discriminant;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.pos];
        self.pos += 1;
        token
    }

    fn expect(&mut self, expected: TokenKind) -> Result<&Token, String> {
        let token = self.peek();
        if discriminant(&token.kind) == discriminant(&expected) {
            Ok(self.advance())
        } else {
            Err(format!(
                "Ralat sintaks pada {}:{} - dijangka {:?}, dijumpai {:?}",
                token.line, token.col, expected, token.kind
            ))
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut functions = Vec::new();
        while self.peek().kind != TokenKind::Eof {
            functions.push(self.parse_function()?);
        }
        Ok(Program { functions })
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        self.expect(TokenKind::Fn)?;
        let name = match self.expect(TokenKind::Ident(String::new()))?.kind.clone() {
            TokenKind::Ident(n) => n,
            _ => unreachable!(),
        };
        self.expect(TokenKind::LParen)?;
        let mut params = Vec::new();
        if self.peek().kind != TokenKind::RParen {
            loop {
                if let TokenKind::Ident(pname) = &self.peek().kind {
                    let pname = pname.clone();
                    self.advance();
                    params.push((pname, Type::Unknown));
                } else {
                    return Err("Parameter fungsi mesti berupa pengenal".to_string());
                }
                if self.peek().kind == TokenKind::Comma {
                    self.advance();
                    continue;
                } else {
                    break;
                }
            }
        }
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::LBrace)?;
        let body = self.parse_block()?;
        Ok(Function { name, params, body })
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut stmts = Vec::new();
        while self.peek().kind != TokenKind::RBrace && self.peek().kind != TokenKind::Eof {
            stmts.push(self.parse_statement()?);
        }
        self.expect(TokenKind::RBrace)?;
        Ok(stmts)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match &self.peek().kind {
            TokenKind::Biar => self.parse_let(),
            TokenKind::Jika => self.parse_if(),
            TokenKind::Untuk => self.parse_for(),
            TokenKind::Sementara => self.parse_while(),
            TokenKind::Ident(_) => {
                let line = self.peek().line;
                let col = self.peek().col;
                if let TokenKind::Ident(name) = &self.peek().kind {
                    let name = name.clone();
                    self.advance();
                    if self.peek().kind == TokenKind::Assign {
                        self.advance();
                        let val = self.parse_expression()?;
                        self.expect(TokenKind::Semicolon)?;
                        Ok(Statement::Assign { name, value: val })
                    } else if self.peek().kind == TokenKind::LParen {
                        let func = name;
                        self.advance();
                        let mut args = Vec::new();
                        if self.peek().kind != TokenKind::RParen {
                            loop {
                                args.push(self.parse_expression()?);
                                if self.peek().kind == TokenKind::Comma {
                                    self.advance();
                                    continue;
                                } else {
                                    break;
                                }
                            }
                        }
                        self.expect(TokenKind::RParen)?;
                        self.expect(TokenKind::Semicolon)?;
                        Ok(Statement::Expression(Expression::Call { func, args }))
                    } else {
                        Err(format!("Ralat sintaks pada {}:{} - pernyataan tidak sah", line, col))
                    }
                } else {
                    unreachable!()
                }
            }
            _ => {
                let expr = self.parse_expression()?;
                self.expect(TokenKind::Semicolon)?;
                Ok(Statement::Expression(expr))
            }
        }
    }

    fn parse_let(&mut self) -> Result<Statement, String> {
        self.advance();
        let name = match self.expect(TokenKind::Ident(String::new()))?.kind.clone() {
            TokenKind::Ident(n) => n,
            _ => unreachable!(),
        };
        self.expect(TokenKind::Assign)?;
        let init = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Statement::Let { name, init })
    }

    fn parse_if(&mut self) -> Result<Statement, String> {
        self.advance();
        let cond = self.parse_expression()?;
        self.expect(TokenKind::LBrace)?;
        let then_branch = self.parse_block()?;
        let mut else_branch = None;
        if self.peek().kind == TokenKind::Lain {
            self.advance();
            self.expect(TokenKind::LBrace)?;
            else_branch = Some(self.parse_block()?);
        }
        Ok(Statement::If { condition: cond, then_branch, else_branch })
    }

    fn parse_for(&mut self) -> Result<Statement, String> {
        self.advance();
        let var = match self.expect(TokenKind::Ident(String::new()))?.kind.clone() {
            TokenKind::Ident(n) => n,
            _ => unreachable!(),
        };
        if let TokenKind::Ident(keyword) = &self.peek().kind {
            if keyword == "dalam" {
                self.advance();
            } else {
                return Err(format!("Dijangka 'dalam' selepas pemboleh ubah gelung"));
            }
        } else {
            return Err(format!("Dijangka 'dalam'"));
        }
        let start = self.parse_expression()?;
        self.expect(TokenKind::DotDot)?;
        let end = self.parse_expression()?;
        self.expect(TokenKind::LBrace)?;
        let body = self.parse_block()?;
        Ok(Statement::For { var, start, end, body })
    }

    fn parse_while(&mut self) -> Result<Statement, String> {
        self.advance();
        let cond = self.parse_expression()?;
        self.expect(TokenKind::LBrace)?;
        let body = self.parse_block()?;
        Ok(Statement::While { condition: cond, body })
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_term()?;
        loop {
            let op = match &self.peek().kind {
                TokenKind::EqualEqual => BinaryOp::Eq,
                TokenKind::NotEqual => BinaryOp::Neq,
                TokenKind::Less => BinaryOp::Lt,
                TokenKind::Greater => BinaryOp::Gt,
                TokenKind::LessEqual => BinaryOp::Le,
                TokenKind::GreaterEqual => BinaryOp::Ge,
                _ => break,
            };
            self.advance();
            let right = self.parse_term()?;
            left = Expression::Binary { left: Box::new(left), op, right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_factor()?;
        loop {
            let op = match &self.peek().kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_factor()?;
            left = Expression::Binary { left: Box::new(left), op, right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match &self.peek().kind {
                TokenKind::Star => BinaryOp::Mul,
                TokenKind::Slash => BinaryOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expression::Binary { left: Box::new(left), op, right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression, String> {
        if self.peek().kind == TokenKind::Minus {
            self.advance();
            let expr = self.parse_unary()?;
            Ok(Expression::Unary { op: UnaryOp::Neg, expr: Box::new(expr) })
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match &self.peek().kind {
            TokenKind::Int(val) => {
                let v = *val;
                self.advance();
                Ok(Expression::IntLiteral(v))
            }
            TokenKind::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(Expression::StringLiteral(s))
            }
            TokenKind::Ident(name) => {
                let name = name.clone();
                self.advance();
                if self.peek().kind == TokenKind::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    if self.peek().kind != TokenKind::RParen {
                        loop {
                            args.push(self.parse_expression()?);
                            if self.peek().kind == TokenKind::Comma {
                                self.advance();
                                continue;
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(TokenKind::RParen)?;
                    Ok(Expression::Call { func: name, args })
                } else {
                    Ok(Expression::Ident(name))
                }
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            _ => Err(format!(
                "Ralat sintaks pada {}:{} - token tidak dijangka {:?}",
                self.peek().line, self.peek().col, self.peek().kind
            )),
        }
    }
}
