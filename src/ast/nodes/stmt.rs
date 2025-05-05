use crate::{
    ast::traits::{StmtVisitor, VisitStmt},
    token::Token,
};

use super::Expr;

pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Variable(VariableDeclaration),
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct PrintStmt {
    pub expression: Expr,
}

pub struct VariableDeclaration {
    pub token: Token,
    pub initializer: Expr,
}

// VisitStmt impls
impl<R, V: StmtVisitor<R>> VisitStmt<R, V> for Stmt {
    fn accept(&self, visitor: &mut V) -> R {
        visitor.visit_statement(self)
    }
}

impl<R, V: StmtVisitor<R>> VisitStmt<R, V> for ExpressionStmt {
    fn accept(&self, visitor: &mut V) -> R {
        visitor.visit_expression(self)
    }
}

impl<R, V: StmtVisitor<R>> VisitStmt<R, V> for PrintStmt {
    fn accept(&self, visitor: &mut V) -> R {
        visitor.visit_print(self)
    }
}

impl<R, V: StmtVisitor<R>> VisitStmt<R, V> for VariableDeclaration {
    fn accept(&self, visitor: &mut V) -> R {
        visitor.visit_variable_declaration(self)
    }
}
