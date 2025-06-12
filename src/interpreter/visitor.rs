use crate::{
    ast::{
        nodes::{
            Assign, Binary, BlockStmt, Call, Expr, ExpressionStmt, ForStmt, FunctionStmt, Grouping,
            IfStmt, Lit, Logical, PrintStmt, ReturnStmt, Stmt, Unary, Variable,
            VariableDeclarationStmt, WhileStmt,
        },
        traits::{ExprVisitor, StmtVisitor, VisitExpr, VisitStmt},
    },
    error::LoxError,
    token::{Literal, NativeFunction, Token, TokenType},
};

use super::{callable::LoxCallable, environment::Environment};

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new(None);
        globals.define(
            Token::new(TokenType::Identifier, None, Some("clock".into()), 0.into()),
            Literal::NativeFunction(NativeFunction::Clock),
        );

        Interpreter {
            environment: globals,
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<Option<Literal>, LoxError> {
        for statement in statements {
            self.visit_statement(statement)?;
        }
        Ok(None)
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
            Literal::String(_string) => Ok(self.get_boolean_literal(true, invert)),
            Literal::Number(number) => Ok(self.get_boolean_literal(number != 0.0, invert)),
            Literal::Boolean(boolean) => Ok(self.get_boolean_literal(boolean, invert)),
            Literal::NativeFunction(_) => Ok(Literal::Boolean(false)),
            Literal::Function(_) => Ok(Literal::Boolean(false)),
            Literal::Nil => Ok(self.get_boolean_literal(false, invert)),
        }
    }
}

impl StmtVisitor<Result<Option<Literal>, LoxError>> for Interpreter {
    fn visit_statement(&mut self, stmt: &Stmt) -> Result<Option<Literal>, LoxError> {
        match stmt {
            Stmt::Print(print_stmt) => self.visit_print(print_stmt),
            Stmt::Expression(expr_stmt) => self.visit_expression(expr_stmt),
            Stmt::Variable(variable_stmt) => self.visit_variable_declaration(variable_stmt),
            Stmt::Block(block_stmt) => self.visit_block(block_stmt),
            Stmt::If(if_stmt) => self.visit_if(if_stmt),
            Stmt::While(while_stmt) => self.visit_while(while_stmt),
            Stmt::For(for_stmt) => self.visit_for(for_stmt),
            Stmt::Function(function_stmt) => self.visit_function(function_stmt),
            Stmt::Return(return_stmt) => return_stmt.accept(self),
        }
    }

    fn visit_expression(
        &mut self,
        expr_stmt: &ExpressionStmt,
    ) -> Result<Option<Literal>, LoxError> {
        self.visit_expr(&expr_stmt.expression)?;
        Ok(None)
    }

    fn visit_print(&mut self, print_stmt: &PrintStmt) -> Result<Option<Literal>, LoxError> {
        println!("{}", self.visit_expr(&print_stmt.expression)?);
        Ok(None)
    }

    fn visit_variable_declaration(
        &mut self,
        variable_stmt: &VariableDeclarationStmt,
    ) -> Result<Option<Literal>, LoxError> {
        let result = self.visit_expr(&variable_stmt.initializer)?;
        self.environment.define(variable_stmt.token.clone(), result);
        Ok(None)
    }

    fn visit_block(&mut self, block_stmt: &BlockStmt) -> Result<Option<Literal>, LoxError> {
        let mut return_value = None;
        self.environment.start_scope();
        for statement in &block_stmt.statements {
            return_value = statement.accept(self)?;
            if return_value.is_some() {
                self.environment.close_scope();
                return Ok(return_value);
            }
        }
        self.environment.close_scope();
        Ok(return_value)
    }

    fn visit_if(&mut self, if_stmt: &IfStmt) -> Result<Option<Literal>, LoxError> {
        let condition = self.visit_expr(&if_stmt.condition)?;
        if let Literal::Boolean(is_true) = self.is_truthy(condition, false)? {
            match is_true {
                true => {
                    return if_stmt.then_branch.accept(self);
                }
                false => match &if_stmt.else_branch {
                    Some(statement) => return statement.accept(self),
                    None => {}
                },
            }
            return Ok(None);
        }

        panic!("Unrecoverable error");
    }

    fn visit_while(&mut self, while_stmt: &WhileStmt) -> Result<Option<Literal>, LoxError> {
        loop {
            let condition = while_stmt.condition.accept(self)?;
            if let Literal::Boolean(is_true) = self.is_truthy(condition, false)? {
                if is_true {
                    let result = while_stmt.body.accept(self)?;
                    if result.is_some() {
                        return Ok(result);
                    }
                } else {
                    break;
                }
            }
        }

        Ok(None)
    }

    fn visit_for(&mut self, for_stmt: &ForStmt) -> Result<Option<Literal>, LoxError> {
        if let Some(initializer) = &for_stmt.initializer {
            initializer.accept(self)?;
        }

        loop {
            let condition_value = match &for_stmt.condition {
                Some(condition_expr) => {
                    let value = condition_expr.accept(self)?;
                    match self.is_truthy(value, false) {
                        Ok(Literal::Boolean(value)) => value,
                        _ => false,
                    }
                }
                None => true,
            };

            if !condition_value {
                break;
            }

            let result = for_stmt.body.accept(self)?;
            if result.is_some() {
                return Ok(result);
            }
            match &for_stmt.increment {
                Some(expr) => {
                    expr.accept(self)?;
                }
                None => {}
            }
        }

        Ok(None)
    }

    fn visit_function(
        &mut self,
        function_stmt: &FunctionStmt,
    ) -> Result<Option<Literal>, LoxError> {
        let function = function_stmt.clone();
        self.environment
            .define(*(function_stmt.name).clone(), Literal::Function(function));

        Ok(None)
    }

    fn visit_return(&mut self, return_stmt: &ReturnStmt) -> Result<Option<Literal>, LoxError> {
        Ok(Some(return_stmt.value.accept(self)?))
    }
}

impl ExprVisitor<Result<Literal, LoxError>> for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> Result<Literal, LoxError> {
        match expr {
            Expr::Binary(binary) => self.visit_binary_expr(binary),
            Expr::Grouping(grouping) => self.visit_grouping_expr(grouping),
            Expr::Literal(lit) => self.visit_literal_expr(lit),
            Expr::Unary(unary) => self.visit_unary_expr(unary),
            Expr::Variable(variable) => self.visit_variable_expr(variable),
            Expr::Assign(assign) => self.visit_assign_expr(assign),
            Expr::Logical(logical) => self.visit_logical_expr(logical),
            Expr::Call(call) => self.visit_call_expr(call),
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
        let value = assign_expr.value.accept(self)?;
        self.environment.assign(&assign_expr.token, &value)
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> Result<Literal, LoxError> {
        let left = expr.left.accept(self)?;

        if let Literal::Boolean(value) = self.is_truthy(left.clone(), false)? {
            match (expr.operator.ty.clone(), value) {
                (TokenType::Or, true) => {
                    return Ok(left);
                }
                (TokenType::And, false) => {
                    return Ok(left);
                }
                _ => {}
            }
        }

        return expr.right.accept(self);
    }

    fn visit_call_expr(&mut self, call_expr: &Call) -> Result<Literal, LoxError> {
        let callee = call_expr.callee.accept(self)?;
        let mut arguments: Vec<Literal> = vec![];
        for argument in &call_expr.arguments {
            arguments.push(argument.accept(self)?);
        }

        callee.call(self, &call_expr.paren, arguments)
    }
}
