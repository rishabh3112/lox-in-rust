use crate::token::{Token, TokenType};

use super::traits::{Visit, Visitor};

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl<R, V: Visitor<R>> Visit<R, V> for Expr {
    fn accept(&self, visitor: &V) -> R {
        V::visit_expr(visitor, self)
    }
}

pub struct Binary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Token,
}

impl<R, V: Visitor<R>> Visit<R, V> for Binary {
    fn accept(&self, visitor: &V) -> R {
        V::visit_binary_expr(visitor, self)
    }
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

impl<R, V: Visitor<R>> Visit<R, V> for Grouping {
    fn accept(&self, visitor: &V) -> R {
        V::visit_grouping_expr(visitor, self)
    }
}

pub struct Literal {
    pub literal: TokenType,
}

impl<R, V: Visitor<R>> Visit<R, V> for Literal {
    fn accept(&self, visitor: &V) -> R {
        V::visit_literal_expr(visitor, self)
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl<R, V: Visitor<R>> Visit<R, V> for Unary {
    fn accept(&self, visitor: &V) -> R {
        V::visit_unary_expr(visitor, self)
    }
}
