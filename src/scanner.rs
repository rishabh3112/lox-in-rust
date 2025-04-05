use crate::token::{Token, TokenType};
use std::{error, str::Chars};

pub struct Scanner<'a> {
    source: &'a String,
    chars: Chars<'a>,
    line: usize,
    errors: bool,
    start: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source,
            chars: source.chars(),
            line: 1,
            errors: false,
            start: 0,
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, Vec<Token>> {
        let mut tokens: Vec<Token> = vec![];
        loop {
            self.start = self.offset();
            let ty = self.read_next_token();

            if ty == TokenType::EOF {
                tokens.push(Token::new(ty, None));
                break;
            }

            let lexeme = self.source[self.start..self.offset()].to_string();
            tokens.push(Token::new(ty, Some(lexeme)));
        }

        if self.errors {
            Err(tokens)
        } else {
            Ok(tokens)
        }
    }

    fn read_next_token(self: &mut Self) -> TokenType {
        while let Some(char) = self.chars.next() {
            match char {
                '(' => return TokenType::LEFT_PAREN,
                ')' => return TokenType::RIGHT_PAREN,
                '{' => return TokenType::LEFT_BRACE,
                '}' => return TokenType::RIGHT_BRACE,
                '-' => return TokenType::MINUS,
                '+' => return TokenType::PLUS,
                ',' => return TokenType::COMMA,
                '.' => return TokenType::DOT,
                ';' => return TokenType::SEMICOLON,
                '/' => return TokenType::COMMA,
                '*' => return TokenType::STAR,
                '\n' => {
                    self.line += 1;
                    self.start += 1;
                }
                _ => {
                    self.start += 1;
                    eprintln!("[line {}] Error: Unexpected character: {}", self.line, char);
                    self.errors = true;
                }
            }
        }

        return TokenType::EOF;
    }

    // helpers
    fn offset(&mut self) -> usize {
        return self.source.len() - self.chars.as_str().len();
    }
}
