use std::collections::HashMap;

use crate::{error::LoxError, literal::Literal, token::Token};

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, variable: Token, value: Literal) {
        self.values.insert(variable.lexeme, value);
    }

    pub fn get(&mut self, variable: &Token) -> Result<Literal, LoxError> {
        if let Some(value) = self.values.get(&variable.lexeme) {
            return Ok(value.clone());
        } else if let Some(parent) = &mut self.enclosing {
            return parent.get(variable);
        }

        Err(LoxError::Runtime {
            token: variable.clone(),
            message: format!("Undefined variable '{}'.", variable.lexeme),
        })
    }

    pub fn assign(&mut self, variable: &Token, value: &Literal) -> Result<Literal, LoxError> {
        if let Some(_) = self.values.get(&variable.lexeme) {
            self.values.insert(variable.lexeme.clone(), value.clone());
            return Ok(value.to_owned());
        } else if let Some(parent) = &mut self.enclosing {
            return parent.assign(variable, value);
        }

        Err(LoxError::Runtime {
            token: variable.clone(),
            message: format!("Undefined variable '{}'.", variable.lexeme),
        })
    }

    pub fn start_scope(&mut self) {
        *self = Environment::new(Some(Box::new(self.clone())))
    }

    pub fn close_scope(&mut self) {
        let parent = self.enclosing.as_ref().unwrap();
        *self = *parent.clone();
    }
}
