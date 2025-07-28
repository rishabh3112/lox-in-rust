use std::collections::HashMap;

use crate::{
    ast::{
        nodes::*,
        traits::{ExprVisitor, StmtVisitor, VisitExpr, VisitStmt},
    },
    error::LoxError,
    interpreter::Interpreter,
    token::Token,
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

    pub fn resolve_stmt(&mut self, statement: &Stmt) -> Option<LoxError> {
        statement.accept(self)
    }

    pub fn resolve_stmts(&mut self, statements: &Vec<Stmt>) -> Option<LoxError> {
        for statement in statements {
            self.resolve_stmt(&statement)?;
        }
        None
    }

    pub fn resolve_expr(&mut self, expr: &Expr) -> Option<LoxError> {
        expr.accept(self)
    }

    fn define(&mut self, token: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(token.lexeme.to_owned(), false);
        }
    }

    fn declare(&mut self, token: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(token.lexeme.to_owned(), true);
        }
    }

    fn resolve_local(&self, expr: &Expr, token: &Token) {
        for (index, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&token.lexeme) {
                self.interpreter
                    .resolve(expr, self.scopes.len() - 1 - index);
                return;
            }
        }
    }

    fn resolve_function_stmt(&mut self, function_stmt: &FunctionStmt) {
        self.begin_scope();
        for token in &function_stmt.params {
            self.declare(token);
            self.define(token);
        }
        self.resolve_stmts(&function_stmt.body);
        self.end_scope();
    }
}

impl StmtVisitor<Option<LoxError>> for Resolver {
    fn visit_statement(&mut self, _stmt: &Stmt) -> Option<LoxError> {
        None
    }

    fn visit_expression(&mut self, expr_stmt: &ExpressionStmt) -> Option<LoxError> {
        self.resolve_expr(&expr_stmt.expression)
    }

    fn visit_print(&mut self, print_stmt: &PrintStmt) -> Option<LoxError> {
        self.resolve_expr(&print_stmt.expression)
    }

    fn visit_variable_declaration(
        &mut self,
        variable_stmt: &VariableDeclarationStmt,
    ) -> Option<LoxError> {
        let name = &variable_stmt.token;
        self.define(name);
        self.resolve_expr(&variable_stmt.initializer)?;
        self.declare(name);
        None
    }

    fn visit_block(&mut self, block_stmt: &BlockStmt) -> Option<LoxError> {
        self.begin_scope();
        let result = self.resolve_stmts(&block_stmt.statements);
        self.end_scope();
        result
    }

    fn visit_if(&mut self, if_stmt: &IfStmt) -> Option<LoxError> {
        self.resolve_expr(&if_stmt.condition);
        self.resolve_stmt(if_stmt.then_branch.as_ref());
        if let Some(else_branch) = if_stmt.else_branch.as_ref() {
            self.resolve_stmt(else_branch.as_ref());
        }
        None
    }

    fn visit_while(&mut self, _while_stmt: &WhileStmt) -> Option<LoxError> {
        self.resolve_expr(&_while_stmt.condition);
        self.resolve_stmt(&_while_stmt.body);
        None
    }

    fn visit_for(&mut self, for_stmt: &ForStmt) -> Option<LoxError> {
        if let Some(initializer) = &for_stmt.initializer {
            self.resolve_stmt(&initializer);
        }
        if let Some(condition) = &for_stmt.condition {
            self.resolve_expr(&condition);
        }
        if let Some(increment) = &for_stmt.increment {
            self.resolve_expr(&increment);
        }
        self.resolve_stmt(&for_stmt.body);
        None
    }

    fn visit_function(&mut self, function_stmt: &FunctionStmt) -> Option<LoxError> {
        self.define(&function_stmt.name);
        self.declare(&function_stmt.name);

        self.resolve_function_stmt(function_stmt);
        None
    }

    fn visit_return(&mut self, return_stmt: &ReturnStmt) -> Option<LoxError> {
        self.resolve_expr(&return_stmt.value);
        None
    }
}

impl ExprVisitor<Option<LoxError>> for Resolver {
    fn visit_expr(&mut self, _expr: &Expr) -> Option<LoxError> {
        None
    }

    fn visit_binary_expr(&mut self, binary_expr: &Binary) -> Option<LoxError> {
        self.resolve_expr(&binary_expr.left)?;
        self.resolve_expr(&binary_expr.right)?;
        None
    }

    fn visit_grouping_expr(&mut self, grouping_expr: &Grouping) -> Option<LoxError> {
        self.resolve_expr(&grouping_expr.expression)
    }

    fn visit_literal_expr(&mut self, _literal_expr: &Lit) -> Option<LoxError> {
        None
    }

    fn visit_unary_expr(&mut self, unary_expr: &Unary) -> Option<LoxError> {
        self.resolve_expr(&unary_expr.right)
    }

    fn visit_variable_expr(&mut self, variable_expr: &Variable) -> Option<LoxError> {
        let name = &variable_expr.token.lexeme;
        if let Some(scope) = self.scopes.last() {
            if !scope.get(name).unwrap_or(&true) {
                return Some(LoxError::Parser {
                    token: variable_expr.token.clone(),
                    message: String::from("Can't read local variable in it's own initializer"),
                });
            }
        }

        self.resolve_local(
            &Expr::Variable(variable_expr.to_owned()),
            &variable_expr.token,
        );
        None
    }

    fn visit_assign_expr(&mut self, assign_expr: &Assign) -> Option<LoxError> {
        self.resolve_expr(&assign_expr.value);
        self.resolve_local(&Expr::Assign(assign_expr.to_owned()), &assign_expr.token);
        None
    }

    fn visit_logical_expr(&mut self, logical_expr: &Logical) -> Option<LoxError> {
        self.resolve_expr(&logical_expr.left)?;
        self.resolve_expr(&logical_expr.right)?;
        None
    }

    fn visit_call_expr(&mut self, call_expr: &Call) -> Option<LoxError> {
        self.resolve_expr(&call_expr.callee)?;
        for arg in &call_expr.arguments {
            self.resolve_expr(arg)?;
        }
        None
    }
}
