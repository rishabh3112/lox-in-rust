use crate::{
    ast::traits::{StmtVisitor, VisitStmt},
    token::Token,
};

use super::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Variable(VariableDeclarationStmt),
    If(IfStmt),
    While(WhileStmt),
    For(ForStmt),
    Function(FunctionStmt),
    Return(ReturnStmt),
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrintStmt {
    pub expression: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarationStmt {
    pub token: Token,
    pub initializer: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStmt {
    pub initializer: Option<Box<Stmt>>,
    pub condition: Option<Expr>,
    pub increment: Option<Expr>,
    pub body: Box<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionStmt {
    pub name: Box<Token>,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStmt {
    pub token: Token,
    pub value: Expr,
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

impl<R, V: StmtVisitor<R>> VisitStmt<R, V> for ForStmt {
    fn accept(&self, visitor: &mut V) -> R {
        visitor.visit_for(self)
    }
}

impl<R, V: StmtVisitor<R>> VisitStmt<R, V> for FunctionStmt {
    fn accept(&self, visitor: &mut V) -> R {
        visitor.visit_function(self)
    }
}

impl<R, V: StmtVisitor<R>> VisitStmt<R, V> for ReturnStmt {
    fn accept(&self, visitor: &mut V) -> R {
        visitor.visit_return(self)
    }
}
