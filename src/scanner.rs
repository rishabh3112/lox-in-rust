use crate::token::{Token, TokenType};
use std::str::Chars;

pub struct Scanner<'a> {
    source: &'a String,
    chars: Chars<'a>,
    line: usize,
    errors: Vec<String>,
    start: usize,
}

pub struct ScannerOutput {
    pub tokens: Vec<Token>,
    pub errors: Vec<String>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source,
            chars: source.chars(),
            line: 1,
            errors: vec![],
            start: 0,
        }
    }

    pub fn run(self: &mut Self) -> ScannerOutput {
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

        let output = ScannerOutput {
            tokens,
            errors: self.errors.clone(),
        };

        output
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
                '!' => {
                    if let Some(next) = self.peek() {
                        match next {
                            '=' => {
                                self.chars.next();
                                return TokenType::BANG_EQUAL;
                            }
                            _ => {}
                        }
                    }
                    return TokenType::BANG;
                }
                '=' => {
                    if let Some(next) = self.peek() {
                        match next {
                            '=' => {
                                self.chars.next();
                                return TokenType::EQUAL_EQUAL;
                            }
                            _ => {}
                        }
                    }
                    return TokenType::EQUAL;
                }
                ' ' | '\t' => {}
                '\n' => {
                    self.line += 1;
                    self.start += 1;
                }
                _ => {
                    self.start += 1;
                    self.errors.push(format!(
                        "[line {}] Error: Unexpected character: {}",
                        self.line, char
                    ));
                }
            }
        }

        return TokenType::EOF;
    }

    // helpers
    fn peek(&mut self) -> Option<char> {
        self.chars.clone().next()
    }

    fn offset(&mut self) -> usize {
        return self.source.len() - self.chars.as_str().len();
    }
}
