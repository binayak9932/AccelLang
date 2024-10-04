#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    Binary(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Expr,
}


use crate::lexer::{Lexer, Token};
use crate::parser::{BinOp, Expr, Function};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse_function(&mut self) -> Function {
        self.expect_token(Token::Fn);
        let name = self.parse_identifier();
        self.expect_token(Token::LParen);
        let params = self.parse_params();
        self.expect_token(Token::RParen);
        self.expect_token(Token::LBrace);
        let body = self.parse_expr();
        self.expect_token(Token::RBrace);
        Function { name, params, body }
    }

    fn parse_params(&mut self) -> Vec<String> {
        let mut params = Vec::new();
        if let Token::Identifier(_) = self.current_token {
            params.push(self.parse_identifier());
            while self.current_token == Token::Comma {
                self.next_token();
                params.push(self.parse_identifier());
            }
        }
        params
    }

    fn parse_identifier(&mut self) -> String {
        if let Token::Identifier(name) = &self.current_token {
            let name = name.clone();
            self.next_token();
            name
        } else {
            panic!("Expected identifier, found {:?}", self.current_token);
        }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Expr {
        let mut node = self.parse_factor();
        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            let op = match self.current_token {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            self.next_token();
            let rhs = self.parse_factor();
            node = Expr::Binary(Box::new(node), op, Box::new(rhs));
        }
        node
    }

    fn parse_factor(&mut self) -> Expr {
        let mut node = self.parse_primary();
        while self.current_token == Token::Star || self.current_token == Token::Slash {
            let op = match self.current_token {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                _ => unreachable!(),
            };
            self.next_token();
            let rhs = self.parse_primary();
            node = Expr::Binary(Box::new(node), op, Box::new(rhs));
        }
        node
    }

    fn parse_primary(&mut self) -> Expr {
        match &self.current_token {
            Token::Number(n) => {
                let n = *n;
                self.next_token();
                Expr::Number(n)
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.next_token();
                Expr::Identifier(name)
            }
            Token::LParen => {
                self.next_token();
                let expr = self.parse_expr();
                self.expect_token(Token::RParen);
                expr
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn expect_token(&mut self, expected: Token) {
        if self.current_token == expected {
            self.next_token();
        } else {
            panic!("Expected token {:?}, found {:?}", expected, self.current_token);
        }
    }
}
