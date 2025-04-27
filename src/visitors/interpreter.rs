use std::mem::discriminant;

use crate::{
    ast::{
        nodes::{Binary, Expr, Grouping, Lit, Unary},
        traits::{Visit, Visitor},
    },
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
    fn is_truthy(&self, value: Literal, invert: bool) -> Result<Literal, String> {
        match value {
            Literal::String(string) => Ok(self.get_boolean_literal(string.len() > 0, invert)),
            Literal::Number(number) => Ok(self.get_boolean_literal(number != 0.0, invert)),
            Literal::Boolean(boolean) => Ok(self.get_boolean_literal(boolean, invert)),
            Literal::Nil => Ok(self.get_boolean_literal(false, invert)),
        }
    }
}

impl Visitor<Result<Literal, String>> for Interpreter {
    fn visit_expr(&self, expr: &Expr) -> Result<Literal, String> {
        match expr {
            Expr::Binary(binary) => self.visit_binary_expr(binary),
            Expr::Grouping(grouping) => self.visit_grouping_expr(grouping),
            Expr::Literal(lit) => self.visit_literal_expr(lit),
            Expr::Unary(unary) => self.visit_unary_expr(unary),
        }
    }

    fn visit_binary_expr(&self, binary_expr: &Binary) -> Result<Literal, String> {
        let left_result = binary_expr.left.accept(self)?;
        let right_result = binary_expr.right.accept(self)?;

        match binary_expr.operator.ty {
            TokenType::Comma => Ok(right_result.clone()),
            TokenType::Minus => {
                if let Literal::Number(left) = left_result {
                    if let Literal::Number(right) = right_result {
                        return Ok(Literal::Number(left - right));
                    }
                }

                Err(String::from("lol"))
            }
            TokenType::Plus => {
                if let Literal::Number(left) = left_result {
                    if let Literal::Number(right) = right_result {
                        return Ok(Literal::Number(left - right));
                    }
                }

                if let Literal::String(left) = left_result {
                    if let Literal::String(right) = right_result {
                        return Ok(Literal::String(format!("{}{}", left, right)));
                    }
                }

                Err(String::from("LOL"))
            }
            TokenType::Slash => {
                if let Literal::Number(left) = left_result {
                    if let Literal::Number(right) = right_result {
                        return Ok(Literal::Number(left / right));
                    }
                }

                Err(String::from("lol"))
            }
            TokenType::Star => {
                if let Literal::Number(left) = left_result {
                    if let Literal::Number(right) = right_result {
                        return Ok(Literal::Number(left * right));
                    }
                }

                Err(String::from("lol"))
            }
            TokenType::Bang => todo!(),
            TokenType::BangEqual => todo!(),
            TokenType::Equal => todo!(),
            TokenType::EqualEqual => todo!(),
            TokenType::Greater => todo!(),
            TokenType::GreaterEqual => todo!(),
            TokenType::Less => todo!(),
            TokenType::LessEqual => todo!(),
            _ => Err(String::from("LOL")),
        }
    }

    fn visit_grouping_expr(&self, grouping_expr: &Grouping) -> Result<Literal, String> {
        grouping_expr.expression.accept(self)
    }

    fn visit_literal_expr(&self, literal_expr: &Lit) -> Result<Literal, String> {
        Ok(literal_expr.literal.clone())
    }

    fn visit_unary_expr(&self, unary_expr: &Unary) -> Result<Literal, String> {
        let right_result = unary_expr.right.accept(self)?;

        match unary_expr.operator.ty {
            TokenType::Minus => match right_result {
                Literal::Number(number) => Ok(Literal::Number(number * -1.0)),
                _ => Err(String::from(
                    "ERROR, unary operator can only work on numbers",
                )),
            },
            TokenType::Bang => self.is_truthy(right_result, true),
            _ => Err(String::from("Expected - or !")),
        }
    }
}
