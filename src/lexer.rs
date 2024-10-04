#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Fn,
    Let,
    Identifier(String),
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            current_char: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        self.current_char = if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input.as_bytes()[self.read_position] as char)
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.current_char {
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Star,
            Some('/') => Token::Slash,
            Some('=') => Token::Equal,
            Some(';') => Token::Semicolon,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some(',') => Token::Comma,
            Some(ch) if ch.is_alphabetic() => {
                let ident = self.read_identifier();
                match ident.as_str() {
                    "fn" => Token::Fn,
                    "let" => Token::Let,
                    _ => Token::Identifier(ident),
                }
            }
            Some(ch) if ch.is_digit(10) => Token::Number(self.read_number()),
            None => Token::EOF,
            _ => panic!("Unexpected character: {}", self.current_char.unwrap()),
        };
        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[start..self.position].to_string()
    }

    fn read_number(&mut self) -> f64 {
        let start = self.position;
        while let Some(ch) = self.current_char {
            if ch.is_digit(10) || ch == '.' {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[start..self.position].parse::<f64>().unwrap()
    }
}

