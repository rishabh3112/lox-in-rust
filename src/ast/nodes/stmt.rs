use crate::{
    ast::traits::{StmtVisitor, VisitStmt},
    token::Token,
};

use super::Expr;

pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Variable(VariableDeclarationStmt),
    If(IfStmt),
    While(WhileStmt),
}

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct PrintStmt {
    pub expression: Expr,
}

pub struct VariableDeclarationStmt {
    pub token: Token,
    pub initializer: Expr,
}

pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

// VisitStmt impls

impl<R, V: StmtVisitor<R>> VisitStmt<R, V> for BlockStmt {
    fn accept(&self, visitor: &mut V) -> R {
        visitor.visit_block(self)
    }
}

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

impl<R, V: StmtVisitor<R>> VisitStmt<R, V> for VariableDeclarationStmt {
    fn accept(&self, visitor: &mut V) -> R {
        visitor.visit_variable_declaration(self)
    }
}

impl<R, V: StmtVisitor<R>> VisitStmt<R, V> for IfStmt {
    fn accept(&self, visitor: &mut V) -> R {
        visitor.visit_if(self)
    }
}

impl<R, V: StmtVisitor<R>> VisitStmt<R, V> for WhileStmt {
    fn accept(&self, visitor: &mut V) -> R {
        visitor.visit_while(self)
    }
}
