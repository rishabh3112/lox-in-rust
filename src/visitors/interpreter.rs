use crate::{
    ast::{
        nodes::{
            Assign, Binary, Expr, ExpressionStmt, Grouping, Lit, PrintStmt, Stmt, Unary, Variable,
            VariableDeclaration,
        },
        traits::{ExprVisitor, StmtVisitor, VisitExpr},
    },
    environment::Environment,
    error::LoxError,
    token::{Literal, TokenType},
};

pub struct Interpreter<'a> {
    environment: &'a mut Environment<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(environment: &'a mut Environment<'a>) -> Self {
        Interpreter { environment }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            self.visit_statement(statement)?;
        }
        Ok(())
    }

    fn are_equal(&mut self, x: Literal, y: Literal, invert: bool) -> Result<Literal, LoxError> {
        Ok(self.get_boolean_literal(x == y, invert))
    }

    fn get_boolean_literal(&mut self, value: bool, invert: bool) -> Literal {
        if invert {
            return Literal::Boolean(!value);
        }

        Literal::Boolean(value)
    }

    fn is_truthy(&mut self, value: Literal, invert: bool) -> Result<Literal, LoxError> {
        match value {
            Literal::String(string) => Ok(self.get_boolean_literal(string.len() > 0, invert)),
            Literal::Number(number) => Ok(self.get_boolean_literal(number != 0.0, invert)),
            Literal::Boolean(boolean) => Ok(self.get_boolean_literal(boolean, invert)),
            Literal::Nil => Ok(self.get_boolean_literal(false, invert)),
        }
    }
}

impl<'a> StmtVisitor<Result<(), LoxError>> for Interpreter<'a> {
    fn visit_statement(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        match stmt {
            Stmt::Print(print_stmt) => self.visit_print(print_stmt),
            Stmt::Expression(expr_stmt) => self.visit_expression(expr_stmt),
            Stmt::Variable(variable_stmt) => self.visit_variable_declaration(variable_stmt),
        }
    }

    fn visit_expression(&mut self, expr_stmt: &ExpressionStmt) -> Result<(), LoxError> {
        self.visit_expr(&expr_stmt.expression)?;
        Ok(())
    }

    fn visit_print(&mut self, print_stmt: &PrintStmt) -> Result<(), LoxError> {
        println!("{}", self.visit_expr(&print_stmt.expression)?);
        Ok(())
    }

    fn visit_variable_declaration(
        &mut self,
        variable_stmt: &VariableDeclaration,
    ) -> Result<(), LoxError> {
        let result = self.visit_expr(&variable_stmt.initializer)?;
        self.environment.define(variable_stmt.token.clone(), result);
        Ok(())
    }
}

impl<'a> ExprVisitor<Result<Literal, LoxError>> for Interpreter<'a> {
    fn visit_expr(&mut self, expr: &Expr) -> Result<Literal, LoxError> {
        match expr {
            Expr::Binary(binary) => self.visit_binary_expr(binary),
            Expr::Grouping(grouping) => self.visit_grouping_expr(grouping),
            Expr::Literal(lit) => self.visit_literal_expr(lit),
            Expr::Unary(unary) => self.visit_unary_expr(unary),
            Expr::Variable(variable) => self.visit_variable_expr(variable),
            Expr::Assign(assign) => self.visit_assign_expr(assign),
        }
    }

    fn visit_binary_expr(&mut self, binary_expr: &Binary) -> Result<Literal, LoxError> {
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
            TokenType::BangEqual => self.are_equal(left_result, right_result, true),
            TokenType::EqualEqual => self.are_equal(left_result, right_result, false),
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

    fn visit_grouping_expr(&mut self, grouping_expr: &Grouping) -> Result<Literal, LoxError> {
        grouping_expr.expression.accept(self)
    }

    fn visit_literal_expr(&mut self, literal_expr: &Lit) -> Result<Literal, LoxError> {
        Ok(literal_expr.literal.clone())
    }

    fn visit_unary_expr(&mut self, unary_expr: &Unary) -> Result<Literal, LoxError> {
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

    fn visit_variable_expr(&mut self, variable_expr: &Variable) -> Result<Literal, LoxError> {
        self.environment.get(&variable_expr.token)
    }

    fn visit_assign_expr(&mut self, assign_expr: &Assign) -> Result<Literal, LoxError> {
        match self.environment.get(&assign_expr.token) {
            Ok(_) => {
                let token = assign_expr.token.clone();
                let val = assign_expr.value.accept(self)?;

                self.environment.define(token, val.clone());
                Ok(val)
            }
            Err(error) => Err(error),
        }
    }
}
