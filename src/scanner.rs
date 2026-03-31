use crate::token::{Literal, Token, TokenType};

pub struct ScanError {
    pub line: usize,
    pub message: String,
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    errors: Vec<ScanError>,
    line: usize,
    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            errors: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> (Vec<Token>, Vec<ScanError>) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "".into(), None, self.line));
        (self.tokens, self.errors)
    }

    fn scan_error(&mut self, message: &str) {
        self.errors.push(ScanError {
            line: self.line,
            message: message.to_string(),
        });
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
            '"' => self.string(),
            '!' => {
                let tt = if self.advance_if('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(tt, None);
            }
            '=' => {
                let tt = if self.advance_if('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(tt, None);
            }
            '<' => {
                let tt = if self.advance_if('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(tt, None);
            }
            '>' => {
                let tt = if self.advance_if('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(tt, None);
            }
            '/' => {
                if self.advance_if('/') {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else if self.advance_if('*') {
                    self.block_comment();
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            c if c.is_ascii_digit() => self.number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),
            _ => self.scan_error("Unexpected character."),
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
            .expect("Unexpected EOF");
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
            .expect("Unexpected EOF")
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1..]
            .chars()
            .next()
            .expect("Unexpected EOF")
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.scan_error("Unterminated string.");
            return;
        }
        self.advance(); // closing "
        let literal = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, Some(Literal::String(literal)));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let literal = &self.source[self.start..self.current];
        self.add_token(
            TokenType::Number,
            Some(Literal::Number(
                literal.parse::<f64>().expect("Failed to parse number"),
            )),
        );
    }

    fn block_comment(&mut self) {
        let mut depth = 1;
        while !self.is_at_end() && depth > 0 {
            if self.peek() == '/' && self.peek_next() == '*' {
                self.advance();
                self.advance();
                depth += 1;
            } else if self.peek() == '*' && self.peek_next() == '/' {
                self.advance();
                self.advance();
                depth -= 1;
            } else {
                if self.peek() == '\n' {
                    self.line += 1;
                }
                self.advance();
            }
        }
        if depth > 0 {
            self.scan_error("Unterminated block comment.");
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let token_type = TokenType::keyword_from_str(text).unwrap_or(TokenType::Identifier);
        self.add_token(token_type, None);
    }
}
