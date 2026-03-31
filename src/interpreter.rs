use crate::{
    expr::Expr,
    stmt::Stmt,
    token::{Literal, Token, TokenType},
};
use std::ops::Not;

// ── Runtime error ────────────────────────────────────────────────────────────

pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    fn new(token: Token, message: &str) -> Self {
        Self {
            token,
            message: message.to_string(),
        }
    }
}

// ── Runtime value ────────────────────────────────────────────────────────────

#[derive(PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Value {
    // Unary
    pub fn negate(self, token: &Token) -> Result<Value, RuntimeError> {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(RuntimeError::new(
                token.clone(),
                "Operand must be a number.",
            )),
        }
    }

    // Binary — arithmetic
    pub fn add(self, other: Value, token: &Token) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
            _ => Err(RuntimeError::new(
                token.clone(),
                "Operands must be two numbers or two strings.",
            )),
        }
    }

    pub fn sub(self, other: Value, token: &Token) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(RuntimeError::new(
                token.clone(),
                "Operands must be numbers.",
            )),
        }
    }

    pub fn mul(self, other: Value, token: &Token) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(RuntimeError::new(
                token.clone(),
                "Operands must be numbers.",
            )),
        }
    }

    pub fn div(self, other: Value, token: &Token) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
            _ => Err(RuntimeError::new(
                token.clone(),
                "Operands must be numbers.",
            )),
        }
    }

    // Binary — comparison
    pub fn greater(self, other: Value, token: &Token) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
            _ => Err(RuntimeError::new(
                token.clone(),
                "Operands must be numbers.",
            )),
        }
    }

    pub fn greater_equal(self, other: Value, token: &Token) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
            _ => Err(RuntimeError::new(
                token.clone(),
                "Operands must be numbers.",
            )),
        }
    }

    pub fn less(self, other: Value, token: &Token) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
            _ => Err(RuntimeError::new(
                token.clone(),
                "Operands must be numbers.",
            )),
        }
    }

    pub fn less_equal(self, other: Value, token: &Token) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
            _ => Err(RuntimeError::new(
                token.clone(),
                "Operands must be numbers.",
            )),
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            _ => true,
        }
    }
}

// `!value` — truthiness never fails so this is the one real ops impl
impl Not for Value {
    type Output = Value;
    fn not(self) -> Value {
        Value::Bool(!self.is_truthy())
    }
}

// ── Interpreter ──────────────────────────────────────────────────────────────

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in statements.into_iter() {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn execute(&self, statement: Stmt) -> Result<(), RuntimeError> {
        match statement {
            Stmt::Expression { expr } => {
                let _ = self.evaluate(&expr)?;
            }
            Stmt::Print { expr } => {
                let value = self.evaluate(&expr)?;
                println!("{}", self.stringify(&value));
            }
            Stmt::Var { name, initializer } => {
                todo!()
            }
        };
        Ok(())
    }

    fn stringify(&self, value: &Value) -> String {
        match value {
            Value::Nil => "nil".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => {
                let s = n.to_string();
                if s.ends_with(".0") {
                    s[..s.len() - 2].to_string()
                } else {
                    s
                }
            }
            Value::String(s) => s.clone(),
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal { value } => Ok(match value {
                Literal::Number(n) => Value::Number(*n),
                Literal::String(s) => Value::String(s.clone()),
                Literal::Bool(b) => Value::Bool(*b),
                Literal::Nil => Value::Nil,
            }),

            Expr::Grouping { expression } => self.evaluate(expression),

            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::Minus => right.negate(operator),
                    TokenType::Bang => Ok(!right),
                    _ => unreachable!(),
                }
            }

            Expr::Variable { name } => todo!(),

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::Plus => left.add(right, operator),
                    TokenType::Minus => left.sub(right, operator),
                    TokenType::Star => left.mul(right, operator),
                    TokenType::Slash => left.div(right, operator),
                    TokenType::Greater => left.greater(right, operator),
                    TokenType::GreaterEqual => left.greater_equal(right, operator),
                    TokenType::Less => left.less(right, operator),
                    TokenType::LessEqual => left.less_equal(right, operator),
                    TokenType::EqualEqual => Ok(Value::Bool(left == right)),
                    TokenType::BangEqual => Ok(Value::Bool(left != right)),
                    _ => unreachable!(),
                }
            }
        }
    }
}
