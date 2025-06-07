use std::time::{SystemTime, UNIX_EPOCH};

use super::Interpreter;
use crate::{
    error::LoxError,
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
        match self {
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
