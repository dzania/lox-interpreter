use crate::expr::Expr;
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: u64,
}

impl Parser {
    fn new() -> Self {
        Self {
            tokens: vec![],
            current: 0,
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparision();
        while self.check_match(&[TokenType::BangEqual, TokenType::Equal]) {
            let operator = self.previous().clone();
            let right = Box::new(self.comparision());
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            };
        }
        return expr;
    }

    fn comparision(&mut self) -> Expr {
        let mut expr = self.term();
        while self.check_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.check_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.check_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.check_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary());
            return Expr::Unary { operator, right };
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.check_match(&[TokenType::False]) {
            return Expr::Literal {
                value: crate::token::Literal::Bool(false),
            };
        }
        if self.check_match(&[TokenType::True]) {
            return Expr::Literal {
                value: crate::token::Literal::Bool(true),
            };
        }
        if self.check_match(&[TokenType::Nil]) {
            return Expr::Literal {
                value: crate::token::Literal::Nil,
            };
        }
        if self.check_match(&[TokenType::Number, TokenType::String]) {
            return Expr::Literal {
                value: self.previous().literal.clone().expect("Expected literal value"),
            };
        }
        if self.check_match(&[TokenType::LeftParen]) {
            let expression = Box::new(self.expression());
            return Expr::Grouping { expression };
        }

        panic!("Expected expression");
    }

    fn check_match(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn check(&self, token_type: &TokenType) -> bool {
        match self.is_at_end() {
            true => false,
            false => &self.peek().token_type == token_type,
        }
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::Eof;
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current as usize)
            .expect("Unexpected EOF")
    }

    fn previous(&self) -> &Token {
        self.tokens
            .get(self.current as usize - 1)
            .expect("Unexpected EOF")
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
}
