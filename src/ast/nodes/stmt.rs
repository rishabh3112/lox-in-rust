use crate::ast::traits::{StmtVisitor, VisitStmt};

use super::Expr;

pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct PrintStmt {
    pub expression: Expr,
}

// VisitStmt impls

impl<R, V: StmtVisitor<R>> VisitStmt<R, V> for Stmt {
    fn accept(&self, visitor: &V) -> R {
        visitor.visit_statement(self)
    }
}

impl<R, V: StmtVisitor<R>> VisitStmt<R, V> for ExpressionStmt {
    fn accept(&self, visitor: &V) -> R {
        visitor.visit_expression(self)
    }
}

impl<R, V: StmtVisitor<R>> VisitStmt<R, V> for PrintStmt {
    fn accept(&self, visitor: &V) -> R {
        visitor.visit_print(self)
    }
}
