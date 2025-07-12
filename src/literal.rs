use std::{
    cell::RefCell,
    fmt::{self, Display, Formatter},
    rc::Rc,
};

use crate::{ast::nodes::FunctionStmt, interpreter::environment::Environment};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NativeFunction {
    Clock,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionLiteral {
    pub node: FunctionStmt,
    pub closure: Rc<RefCell<Environment>>,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Nil,
    String(String),
    Number(f64),
    Boolean(bool),
    Function(FunctionLiteral),
    NativeFunction(NativeFunction),
}

impl Clone for Literal {
    fn clone(&self) -> Self {
        match self {
            Self::String(string) => Self::String(string.clone()),
            Self::Number(number) => Self::Number(number.clone()),
            Self::Boolean(boolean) => Self::Boolean(boolean.clone()),
            Self::Function(function) => Self::Function(function.clone()),
            Self::NativeFunction(function) => Self::NativeFunction(function.clone()),
            Self::Nil => Self::Nil,
        }
    }
}

impl Literal {
    pub fn token_print(&self) -> String {
        match self {
            Literal::String(string) => format!("{}", string),
            Literal::Number(number) => format!("{:?}", number),
            _ => format!("null"),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(string) => write!(f, "{}", string),
            Literal::Number(number) => write!(f, "{}", number),
            Literal::Boolean(boolean) => write!(f, "{}", boolean),
            Literal::NativeFunction(_function) => write!(f, "<fn native>"),
            Literal::Nil => write!(f, "nil"),
            Literal::Function(function) => {
                write!(f, "<fn {}>", function.node.name.lexeme.clone())
            }
        }
    }
}
