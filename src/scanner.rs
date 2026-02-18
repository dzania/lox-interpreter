use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    line: usize,
    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "".into(), None, self.line));
        self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let tt = if self.advance_if('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(tt, None);
            }
            '=' => {
                let tt = if self.advance_if('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(tt, None);
            }
            '<' => {
                let tt = if self.advance_if('=') { TokenType::LessEqual } else { TokenType::Less };
                self.add_token(tt, None);
            }
            '>' => {
                let tt = if self.advance_if('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(tt, None);
            }
            '/' => {
                if self.advance_if('/') {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            _ => crate::error(self.line, "Unexpected character."),
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line));
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current..]
            .chars()
            .next()
            .expect("Advancing out of bounds");
        self.current += c.len_utf8();
        c
    }

    fn advance_if(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c = self.source[self.current..]
            .chars()
            .next()
            .expect("Advancing out of bounds");
        if c != expected {
            return false;
        }
        self.current += c.len_utf8();
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current..]
            .chars()
            .next()
            .expect("Peeking out of bounds")
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
