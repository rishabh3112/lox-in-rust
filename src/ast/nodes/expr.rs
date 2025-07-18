use crate::{
    ast::traits::{ExprVisitor, VisitExpr},
    literal::Literal,
    token::Token,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Lit),
    Unary(Unary),
    Variable(Variable),
    Assign(Assign),
    Logical(Logical),
    Call(Call),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Token,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Logical {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Token,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lit {
    pub literal: Literal,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub token: Token,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assign {
    pub token: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    // closing paren's token, for location reporting in error
    pub paren: Token,
    pub arguments: Vec<Expr>,
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

impl<R, V: ExprVisitor<R>> VisitExpr<R, V> for Call {
    fn accept(&self, visitor: &mut V) -> R {
        V::visit_call_expr(visitor, self)
    }
}
