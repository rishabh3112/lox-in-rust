use crate::{
    ast::{
        nodes::{Binary, Expr, Grouping, Lit, Unary},
        traits::{ExprVisitor, VisitExpr},
    },
    token::Literal,
};

pub struct ASTPrinter {}

impl ASTPrinter {
    pub fn new() -> Self {
        ASTPrinter {}
    }
}

impl ExprVisitor<String> for ASTPrinter {
    fn visit_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(binary) => self.visit_binary_expr(binary),
            Expr::Grouping(grouping) => self.visit_grouping_expr(grouping),
            Expr::Literal(literal) => self.visit_literal_expr(literal),
            Expr::Unary(unary) => self.visit_unary_expr(unary),
            Expr::Variable(_variable) => todo!(),
        }
    }

    fn visit_binary_expr(&mut self, binary_expr: &Binary) -> String {
        format!(
            "({} {} {})",
            binary_expr.operator.lexeme,
            binary_expr.left.accept(self),
            binary_expr.right.accept(self)
        )
    }

    fn visit_grouping_expr(&mut self, grouping_expr: &Grouping) -> String {
        format!("(group {})", grouping_expr.expression.accept(self))
    }

    fn visit_literal_expr(&mut self, literal_expr: &Lit) -> String {
        match &literal_expr.literal {
            Literal::String(string) => format!("{}", string),
            Literal::Number(number) => format!("{:?}", number),
            Literal::Boolean(boolean) => format!("{}", boolean),
            Literal::Nil => format!("nil"),
        }
    }

    fn visit_unary_expr(&mut self, unary_expr: &Unary) -> String {
        format!(
            "({} {})",
            unary_expr.operator.lexeme,
            unary_expr.right.accept(self)
        )
    }

    fn visit_variable_expr(&mut self, _variable_expr: &crate::ast::nodes::Variable) -> String {
        todo!()
    }
}
