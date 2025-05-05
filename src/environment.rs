use std::collections::HashMap;

use crate::{
    error::LoxError,
    token::{Literal, Token},
};

pub struct Environment<'a> {
    values: &'a mut HashMap<String, Literal>,
}

impl<'a> Environment<'a> {
    pub fn new(values: &'a mut HashMap<String, Literal>) -> Self {
        Self { values }
    }

    pub fn define(&mut self, variable: Token, value: Literal) {
        self.values.insert(variable.lexeme, value);
    }

    pub fn get(&mut self, variable: &Token) -> Result<Literal, LoxError> {
        if let Some(value) = self.values.get(&variable.lexeme) {
            return Ok(value.clone());
        }

        Err(LoxError::Runtime {
            token: variable.clone(),
            message: format!("Undefined variable '{}'.", variable.lexeme),
        })
    }
}
