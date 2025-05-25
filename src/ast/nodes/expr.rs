use crate::{
    ast::traits::{ExprVisitor, VisitExpr},
    token::{Literal, Token},
};

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Lit),
    Unary(Unary),
    Variable(Variable),
    Assign(Assign),
    Logical(Logical),
}

pub struct Binary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Token,
}

pub struct Logical {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Token,
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

pub struct Lit {
    pub literal: Literal,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Variable {
    pub token: Token,
}

pub struct Assign {
    pub token: Token,
    pub value: Box<Expr>,
}

// VisitExpr impl
impl<R, V: ExprVisitor<R>> VisitExpr<R, V> for Expr {
    fn accept(&self, visitor: &mut V) -> R {
        V::visit_expr(visitor, self)
    }
}

impl<R, V: ExprVisitor<R>> VisitExpr<R, V> for Binary {
    fn accept(&self, visitor: &mut V) -> R {
        V::visit_binary_expr(visitor, self)
    }
}

impl<R, V: ExprVisitor<R>> VisitExpr<R, V> for Logical {
    fn accept(&self, visitor: &mut V) -> R {
        V::visit_logical_expr(visitor, self)
    }
}

impl<R, V: ExprVisitor<R>> VisitExpr<R, V> for Grouping {
    fn accept(&self, visitor: &mut V) -> R {
        V::visit_grouping_expr(visitor, self)
    }
}

impl<R, V: ExprVisitor<R>> VisitExpr<R, V> for Lit {
    fn accept(&self, visitor: &mut V) -> R {
        V::visit_literal_expr(visitor, self)
    }
}

impl<R, V: ExprVisitor<R>> VisitExpr<R, V> for Unary {
    fn accept(&self, visitor: &mut V) -> R {
        V::visit_unary_expr(visitor, self)
    }
}

impl<R, V: ExprVisitor<R>> VisitExpr<R, V> for Variable {
    fn accept(&self, visitor: &mut V) -> R {
        V::visit_variable_expr(visitor, self)
    }
}

impl<R, V: ExprVisitor<R>> VisitExpr<R, V> for Assign {
    fn accept(&self, visitor: &mut V) -> R {
        V::visit_assign_expr(visitor, self)
    }
}
