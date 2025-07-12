use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::LoxError, literal::Literal, token::Token};

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn from(enclosing: &Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Rc::clone(enclosing)),
        }
    }

    pub fn define(&mut self, variable: Token, value: Literal) {
        self.values.insert(variable.lexeme, value);
    }

    pub fn get(&self, variable: &Token) -> Result<Literal, LoxError> {
        if let Some(value) = self.values.get(&variable.lexeme) {
            return Ok(value.clone());
        } else if let Some(parent) = &self.enclosing {
            return parent.borrow_mut().get(variable);
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
            return parent.borrow_mut().assign(variable, value);
        }

        Err(LoxError::Runtime {
            token: variable.clone(),
            message: format!("Undefined variable '{}'.", variable.lexeme),
        })
    }

    pub fn start_scope(&mut self) {
        *self = Environment::from(&Rc::new(RefCell::new(self.clone())))
    }

    pub fn close_scope(&mut self) {
        if let Some(enclosing) = self.enclosing.take() {
            *self = enclosing.borrow().clone();
        }
    }
}
