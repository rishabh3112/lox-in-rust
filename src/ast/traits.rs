use super::nodes::*;

pub trait Visit<R, V: Visitor<R>> {
    fn accept(&self, visitor: &V) -> R;
}

pub trait Visitor<R> {
    fn visit_expr(&self, expr: &Expr) -> R;
    fn visit_binary_expr(&self, binary_expr: &Binary) -> R;
    fn visit_grouping_expr(&self, grouping_expr: &Grouping) -> R;
    fn visit_literal_expr(&self, literal_expr: &Literal) -> R;
    fn visit_unary_expr(&self, unary_expr: &Unary) -> R;
}
