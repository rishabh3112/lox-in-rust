use std::time::{SystemTime, UNIX_EPOCH};

use super::Interpreter;
use crate::{
    ast::{
        nodes::{BlockStmt, FunctionStmt},
        traits::StmtVisitor,
    },
    error::LoxError,
    interpreter::environment::Environment,
    token::{Literal, NativeFunction, Token},
};

pub trait LoxCallable {
    fn call(
        &self,
        interpreter: &Interpreter,
        token: &Token,
        arguments: Vec<Literal>,
    ) -> Result<Literal, LoxError>;
    fn arity(&self) -> usize;
}

impl LoxCallable for Literal {
    fn call(
        &self,
        interpreter: &Interpreter,
        token: &Token,
        arguments: Vec<Literal>,
    ) -> Result<Literal, LoxError> {
        match self {
            Literal::Function(function) => function.call(interpreter, token, arguments),
            Literal::NativeFunction(function) => function.call(interpreter, token, arguments),
            _ => Err(LoxError::Runtime {
                token: token.clone(),
                message: "Can only call functions and classes.".into(),
            }),
        }
    }

    fn arity(&self) -> usize {
        0
    }
}

impl LoxCallable for NativeFunction {
    fn call(
        &self,
        _interpreter: &Interpreter,
        token: &Token,
        arguments: Vec<Literal>,
    ) -> Result<Literal, LoxError> {
        if arguments.len() > self.arity() {
            return Err(LoxError::Runtime {
                token: token.clone(),
                message: format!(
                    "Expected {} arguments got {}.",
                    self.arity(),
                    arguments.len()
                ),
            });
        }

        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => Ok(Literal::Number(duration.as_secs_f64().round())),
            Err(error) => Err(LoxError::Runtime {
                token: token.clone(),
                message: error.to_string(),
            }),
        }
    }

    fn arity(&self) -> usize {
        0
    }
}

impl LoxCallable for FunctionStmt {
    fn call(
        &self,
        interpreter: &Interpreter,
        _token: &Token,
        arguments: Vec<Literal>,
    ) -> Result<Literal, LoxError> {
        let mut environment = Environment::new(Some(Box::new(interpreter.environment.clone())));
        for (i, param) in self.params.iter().enumerate() {
            if let Some(value) = arguments.get(i) {
                environment.define(param.clone(), value.clone());
            }
        }

        let block = BlockStmt {
            statements: self.body.to_owned(),
        };

        if let Some(return_value) = (Interpreter { environment }).visit_block(&block)? {
            return Ok(return_value);
        }
        Ok(Literal::Nil)
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}
