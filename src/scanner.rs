use crate::token::{Token, TokenType};
use std::str::Chars;

pub struct Scanner<'a> {
    source: &'a String,
    chars: Chars<'a>,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source,
            chars: source.chars(),
            line: 0,
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        loop {
            let start = self.offset();
            let ty = self.read_next_token();
            if ty == TokenType::EOF {
                tokens.push(Token::new(ty, None));
                break;
            }
            let lexeme = self.source[start..self.offset()].to_string();
            tokens.push(Token::new(ty, Some(lexeme)));
        }

        tokens
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
                '\n' => self.line += 1,
                _ => {
                    println!("[line {}] Error: Unexpected character: {}", self.line, char);
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
