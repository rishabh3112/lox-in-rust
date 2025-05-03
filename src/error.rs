use std::fmt::Display;

use crate::token::{Token, TokenType};

#[derive(Clone, Debug)]
pub enum LoxError {
    Scanner { line: usize, message: String },
    Parser { token: Token, message: String },
    Runtime { token: Token, message: String },
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxError::Scanner { line, message } => {
                write!(f, "[line {}] Error: {}", line, message)
            }
            LoxError::Parser { token, message } => report(f, token, message),
            LoxError::Runtime { token, message } => report(f, token, message),
        }
    }
}

fn report(f: &mut std::fmt::Formatter<'_>, token: &Token, message: &String) -> std::fmt::Result {
    if token.ty == TokenType::EOF {
        write!(f, "[line {}] Error at end: {}", token.line, message)
    } else {
        write!(
            f,
            "[line {}] Error at {}: {}",
            token.line, token.lexeme, message
        )
    }
}

impl LoxError {
    fn print(&self) {
        eprintln!("{}", self)
    }
}
