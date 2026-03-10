use crate::expr::Expr;
use crate::token::{Token, TokenType};
use std::fmt;

/// A parse error with the offending token and a message.
#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.token.token_type == TokenType::Eof {
            write!(
                f,
                "[line {}] Error at end: {}",
                self.token.line, self.message
            )
        } else {
            write!(
                f,
                "[line {}] Error at '{}': {}",
                self.token.line, self.token.lexeme, self.message
            )
        }
    }
}

/// Recursive descent parser for the Lox language.
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Creates a new parser with the given token list.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parses an expression (lowest precedence).
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    /// Parses `!=` and `==` expressions.
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparision()?;
        while self.check_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = Box::new(self.comparision()?);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            };
        }
        Ok(expr)
    }

    /// Parses `>`, `>=`, `<`, and `<=` expressions.
    fn comparision(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while self.check_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// Parses `+` and `-` expressions.
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.check_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// Parses `*` and `/` expressions.
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while self.check_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// Parses `!` and unary `-` expressions.
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.check_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary()?);
            return Ok(Expr::Unary { operator, right });
        }
        self.primary()
    }

    /// Parses literals, grouping, and other primary expressions.
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.check_match(&[TokenType::False]) {
            return Ok(Expr::Literal {
                value: crate::token::Literal::Bool(false),
            });
        }
        if self.check_match(&[TokenType::True]) {
            return Ok(Expr::Literal {
                value: crate::token::Literal::Bool(true),
            });
        }
        if self.check_match(&[TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: crate::token::Literal::Nil,
            });
        }
        if self.check_match(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal {
                value: self
                    .previous()
                    .literal
                    .clone()
                    .expect("Expected literal value"),
            });
        }
        if self.check_match(&[TokenType::LeftParen]) {
            let expression = Box::new(self.expression()?);
            self.consume(&TokenType::RightParen, "Expected ')' after expression")?;
            return Ok(Expr::Grouping { expression });
        }

        Err(ParseError {
            token: self.peek().clone(),
            message: "Expected expression".to_string(),
        })
    }

    /// Returns `true` and advances if the current token matches any of the given types.
    fn check_match(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Returns `true` if the current token is the given type (without consuming it).
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().token_type == token_type
    }

    /// Consumes the current token if it matches, otherwise returns a parse error.
    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(ParseError {
            token: self.peek().clone(),
            message: message.to_string(),
        })
    }

    /// Discards tokens until a statement boundary is found, to recover from a parse error.
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                break;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => break,
                _ => (),
            }
            self.advance();
        }
    }

    /// Returns `true` if all tokens have been consumed.
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// Returns the current token without consuming it.
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).expect("Unexpected EOF")
    }

    /// Returns the most recently consumed token.
    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).expect("Unexpected EOF")
    }

    /// Consumes the current token and returns the previous one.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

}
