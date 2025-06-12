use super::nodes::*;

pub trait VisitExpr<R, V: ExprVisitor<R>> {
    fn accept(&self, visitor: &mut V) -> R;
}

pub trait ExprVisitor<R> {
    fn visit_expr(&mut self, expr: &Expr) -> R;
    fn visit_binary_expr(&mut self, binary_expr: &Binary) -> R;
    fn visit_grouping_expr(&mut self, grouping_expr: &Grouping) -> R;
    fn visit_literal_expr(&mut self, literal_expr: &Lit) -> R;
    fn visit_unary_expr(&mut self, unary_expr: &Unary) -> R;
    fn visit_variable_expr(&mut self, variable_expr: &Variable) -> R;
    fn visit_assign_expr(&mut self, assign_expr: &Assign) -> R;
    fn visit_logical_expr(&mut self, logical_expr: &Logical) -> R;
    fn visit_call_expr(&mut self, call_expr: &Call) -> R;
}

pub trait VisitStmt<R, V: StmtVisitor<R>> {
    fn accept(&self, visitor: &mut V) -> R;
}

pub trait StmtVisitor<R> {
    fn visit_statement(&mut self, stmt: &Stmt) -> R;
    fn visit_expression(&mut self, expr_stmt: &ExpressionStmt) -> R;
    fn visit_print(&mut self, print_stmt: &PrintStmt) -> R;
    fn visit_variable_declaration(&mut self, variable_stmt: &VariableDeclarationStmt) -> R;
    fn visit_block(&mut self, block_stmt: &BlockStmt) -> R;
    fn visit_if(&mut self, if_stmt: &IfStmt) -> R;
    fn visit_while(&mut self, while_stmt: &WhileStmt) -> R;
    fn visit_for(&mut self, for_stmt: &ForStmt) -> R;
    fn visit_function(&mut self, function_stmt: &FunctionStmt) -> R;
    fn visit_return(&mut self, return_stmt: &ReturnStmt) -> R;
}
