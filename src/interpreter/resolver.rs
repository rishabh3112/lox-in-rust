use std::collections::HashMap;

use crate::{
    ast::{
        nodes::*,
        traits::{ExprVisitor, StmtVisitor, VisitExpr, VisitStmt},
    },
    interpreter::Interpreter,
};

pub struct Resolver {
    pub interpreter: Interpreter,
    pub scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        return Self {
            interpreter,
            scopes: vec![],
        };
    }

    pub fn begin_scope(&mut self) -> () {
        self.scopes.push(HashMap::new());
    }
    pub fn end_scope(&mut self) -> () {
        self.scopes.pop();
    }

    pub fn resolve_stmt(&mut self, statement: &Stmt) {
        statement.accept(self)
    }

    pub fn resolve_stmts(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            self.resolve_stmt(&statement)
        }
    }

    pub fn resolve_expr(&mut self, expr: &Expr) {
        expr.accept(self)
    }
}

impl StmtVisitor<()> for Resolver {
    fn visit_statement(&mut self, stmt: &Stmt) -> () {}
    fn visit_expression(&mut self, expr_stmt: &ExpressionStmt) -> () {}
    fn visit_print(&mut self, print_stmt: &PrintStmt) -> () {}
    fn visit_variable_declaration(&mut self, variable_stmt: &VariableDeclarationStmt) -> () {}
    fn visit_block(&mut self, block_stmt: &BlockStmt) -> () {
        self.begin_scope();
        self.resolve_stmts(&block_stmt.statements);
        self.end_scope();
    }
    fn visit_if(&mut self, if_stmt: &IfStmt) -> () {}
    fn visit_while(&mut self, while_stmt: &WhileStmt) -> () {}
    fn visit_for(&mut self, for_stmt: &ForStmt) -> () {}
    fn visit_function(&mut self, function_stmt: &FunctionStmt) -> () {}
    fn visit_return(&mut self, return_stmt: &ReturnStmt) -> () {}
}

impl ExprVisitor<()> for Resolver {
    fn visit_expr(&mut self, expr: &Expr) -> () {}
    fn visit_binary_expr(&mut self, binary_expr: &Binary) -> () {}
    fn visit_grouping_expr(&mut self, grouping_expr: &Grouping) -> () {}
    fn visit_literal_expr(&mut self, literal_expr: &Lit) -> () {}
    fn visit_unary_expr(&mut self, unary_expr: &Unary) -> () {}
    fn visit_variable_expr(&mut self, variable_expr: &Variable) -> () {}
    fn visit_assign_expr(&mut self, assign_expr: &Assign) -> () {}
    fn visit_logical_expr(&mut self, logical_expr: &Logical) -> () {}
    fn visit_call_expr(&mut self, call_expr: &Call) -> () {}
}
