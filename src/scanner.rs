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
                '*' => return TokenType::STAR,
                '!' => {
                    if self.match_next('=') {
                        return TokenType::BANG_EQUAL;
                    }
                    return TokenType::BANG;
                }
                '=' => {
                    if self.match_next('=') {
                        return TokenType::EQUAL_EQUAL;
                    }
                    return TokenType::EQUAL;
                }
                '>' => {
                    if self.match_next('=') {
                        return TokenType::GREATER_EQUAL;
                    };
                    return TokenType::GREATER;
                }
                '<' => {
                    if self.match_next('=') {
                        return TokenType::LESS_EQUAL;
                    };
                    return TokenType::LESS;
                }
                '/' => {
                    if self.match_next('/') {
                        self.ignore_line();
                    } else {
                        return TokenType::SLASH;
                    }
                }
                '"' => {
                    if let Some(ty) = self.get_string_literal() {
                        return ty;
                    }
                }
                ' ' | '\t' => self.start += 1,
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

    // handlers
    fn get_string_literal(&mut self) -> Option<TokenType> {
        let mut literal: String = String::new();

        while let Some(next) = self.peek() {
            if next == '"' {
                break;
            }

            self.chars.next();

            if next == '\n' {
                self.line += 1
            }

            literal.push(next);
        }

        if self.chars.as_str().len() == 0 {
            self.errors
                .push(format!("[line {}] Error: Unterminated string", self.line));
            return None;
        }

        return Some(TokenType::STRING(literal));
    }

    // helpers
    fn peek(&mut self) -> Option<char> {
        self.chars.clone().next()
    }

    fn match_next(&mut self, ch: char) -> bool {
        if let Some(next) = self.peek() {
            if ch == next {
                self.chars.next();
                return true;
            }
        }
        false
    }

    fn offset(&mut self) -> usize {
        return self.source.len() - self.chars.as_str().len();
    }

    fn ignore_line(&mut self) {
        while let Some(next) = self.chars.next() {
            if next == '\n' {
                self.start = self.offset();
                self.line += 1;
                break;
            }
        }
    }
}
