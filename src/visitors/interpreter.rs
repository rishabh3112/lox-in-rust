use crate::{
    ast::{
        nodes::{Binary, Expr, Grouping, Lit, Unary},
        traits::{Visit, Visitor},
    },
    error::LoxError,
    token::{Literal, TokenType},
};

pub struct Interpreter {}

impl Interpreter {
    fn get_boolean_literal(&self, value: bool, invert: bool) -> Literal {
        if invert {
            Literal::Boolean(!value)
        } else {
            Literal::Boolean(value)
        }
    }

    fn is_truthy(&self, value: Literal, invert: bool) -> Result<Literal, LoxError> {
        match value {
            Literal::String(string) => Ok(self.get_boolean_literal(string.len() > 0, invert)),
            Literal::Number(number) => Ok(self.get_boolean_literal(number != 0.0, invert)),
            Literal::Boolean(boolean) => Ok(self.get_boolean_literal(boolean, invert)),
            Literal::Nil => Ok(self.get_boolean_literal(false, invert)),
        }
    }

    fn are_equal(&self, x: Literal, y: Literal, invert: bool) -> Result<Literal, LoxError> {
        let x_lit = self.is_truthy(x, false)?;
        let y_lit = self.is_truthy(y, false)?;

        match (x_lit, y_lit) {
            (Literal::Boolean(left), Literal::Boolean(right)) => {
                Ok(self.get_boolean_literal(left == right, invert))
            }
            (_, _) => unreachable!(),
        }
    }
}

impl Visitor<Result<Literal, LoxError>> for Interpreter {
    fn visit_expr(&self, expr: &Expr) -> Result<Literal, LoxError> {
        match expr {
            Expr::Binary(binary) => self.visit_binary_expr(binary),
            Expr::Grouping(grouping) => self.visit_grouping_expr(grouping),
            Expr::Literal(lit) => self.visit_literal_expr(lit),
            Expr::Unary(unary) => self.visit_unary_expr(unary),
        }
    }

    fn visit_binary_expr(&self, binary_expr: &Binary) -> Result<Literal, LoxError> {
        let left_result = binary_expr.left.accept(self)?;
        let right_result = binary_expr.right.accept(self)?;

        match binary_expr.operator.ty {
            TokenType::Comma => Ok(right_result.clone()),
            TokenType::Minus => match (left_result, right_result) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Number(left - right))
                }

                (_, _) => Err(LoxError::Runtime {
                    token: binary_expr.operator.clone(),
                    message: "Operands must be numbers.".into(),
                }),
            },
            TokenType::Plus => match (left_result, right_result) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Number(left + right))
                }
                (Literal::String(left), Literal::String(right)) => {
                    Ok(Literal::String(format!("{}{}", left, right)))
                }
                (_, _) => Err(LoxError::Runtime {
                    token: binary_expr.operator.clone(),
                    message: "Operands must be two numbers or two strings.".into(),
                }),
            },
            TokenType::Slash => match (left_result, right_result) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Number(left / right))
                }

                (_, _) => Err(LoxError::Runtime {
                    token: binary_expr.operator.clone(),
                    message: "Operands must be numbers.".into(),
                }),
            },
            TokenType::Star => match (left_result, right_result) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Number(left * right))
                }

                (_, _) => Err(LoxError::Runtime {
                    token: binary_expr.operator.clone(),
                    message: "Operands must be numbers.".into(),
                }),
            },
            TokenType::BangEqual => self.are_equal(left_result, right_result, false),
            TokenType::EqualEqual => self.are_equal(left_result, right_result, true),
            TokenType::Greater => match (left_result, right_result) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Boolean(left > right))
                }

                (_, _) => Err(LoxError::Runtime {
                    token: binary_expr.operator.clone(),
                    message: "Operands must be numbers.".into(),
                }),
            },
            TokenType::GreaterEqual => match (left_result, right_result) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Boolean(left >= right))
                }

                (_, _) => Err(LoxError::Runtime {
                    token: binary_expr.operator.clone(),
                    message: "Operands must be numbers.".into(),
                }),
            },
            TokenType::Less => match (left_result, right_result) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Boolean(left < right))
                }

                (_, _) => Err(LoxError::Runtime {
                    token: binary_expr.operator.clone(),
                    message: "Operands must be numbers.".into(),
                }),
            },
            TokenType::LessEqual => match (left_result, right_result) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Boolean(left <= right))
                }

                (_, _) => Err(LoxError::Runtime {
                    token: binary_expr.operator.clone(),
                    message: "Operands must be numbers.".into(),
                }),
            },
            _ => Err(LoxError::Runtime {
                token: binary_expr.operator.clone(),
                message: "Invalid binary expression found".into(),
            }),
        }
    }

    fn visit_grouping_expr(&self, grouping_expr: &Grouping) -> Result<Literal, LoxError> {
        grouping_expr.expression.accept(self)
    }

    fn visit_literal_expr(&self, literal_expr: &Lit) -> Result<Literal, LoxError> {
        Ok(literal_expr.literal.clone())
    }

    fn visit_unary_expr(&self, unary_expr: &Unary) -> Result<Literal, LoxError> {
        let right_result = unary_expr.right.accept(self)?;

        match unary_expr.operator.ty {
            TokenType::Minus => match right_result {
                Literal::Number(number) => Ok(Literal::Number(number * -1.0)),
                _ => Err(LoxError::Runtime {
                    token: unary_expr.operator.clone(),
                    message: "Operand must be a number.".into(),
                }),
            },
            TokenType::Bang => self.is_truthy(right_result, true),
            _ => Err(LoxError::Runtime {
                token: unary_expr.operator.clone(),
                message: "Invalid unary expression found".into(),
            }),
        }
    }
}
