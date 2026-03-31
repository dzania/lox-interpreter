use crate::{expr::Expr, token::Token};

pub enum Stmt {
    Expression {
        expr: Expr,
    },
    Print {
        expr: Expr,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
}
