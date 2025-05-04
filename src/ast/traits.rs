use super::nodes::*;

pub trait VisitExpr<R, V: ExprVisitor<R>> {
    fn accept(&self, visitor: &V) -> R;
}

pub trait ExprVisitor<R> {
    fn visit_expr(&self, expr: &Expr) -> R;
    fn visit_binary_expr(&self, binary_expr: &Binary) -> R;
    fn visit_grouping_expr(&self, grouping_expr: &Grouping) -> R;
    fn visit_literal_expr(&self, literal_expr: &Lit) -> R;
    fn visit_unary_expr(&self, unary_expr: &Unary) -> R;
}

pub trait VisitStmt<R, V: StmtVisitor<R>> {
    fn accept(&self, visitor: &V) -> R;
}

pub trait StmtVisitor<R> {
    fn visit_statement(&self, stmt: &Stmt) -> R;
    fn visit_expression(&self, expr_stmt: &ExpressionStmt) -> R;
    fn visit_print(&self, print_stmt: &PrintStmt) -> R;
}
