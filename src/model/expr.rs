use super::*;
use crate::parser::Position;

#[derive(Clone, Debug)]
pub struct Expr {
    expression: Expression,
    position: Option<Position>,
}

impl Expr {
    pub fn new(expression: Expression, position: Option<Position>) -> Self {
        Self {
            expression,
            position,
        }
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}
