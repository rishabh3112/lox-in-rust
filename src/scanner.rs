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
                    if let Some(ty) = self.match_string() {
                        return ty;
                    }
                }
                ' ' | '\t' => self.start += 1,
                '\n' => {
                    self.line += 1;
                    self.start += 1;
                }
                _ => {
                    if char.is_ascii_digit() {
                        if let Some(ty) = self.match_number() {
                            return ty;
                        }
                    } else {
                    }
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
    fn match_string(&mut self) -> Option<TokenType> {
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

        if self.offset() == self.source.len() {
            self.errors
                .push(format!("[line {}] Error: Unterminated string.", self.line));
            return None;
        }

        self.chars.next();

        return Some(TokenType::STRING(literal));
    }

    fn match_number(&mut self) -> Option<TokenType> {
        while let Some(next) = self.peek() {
            if next.is_ascii_digit() {
                self.chars.next();
                continue;
            }
            break;
        }

        if self.peek().is_some_and(|next| next == '.')
            && self.peek_next().is_some_and(|next| next.is_ascii_digit())
        {
            self.chars.next();
            while let Some(next) = self.peek() {
                if next.is_ascii_digit() {
                    self.chars.next();
                    continue;
                }
                break;
            }
        }

        let number_string = self.source[self.start..self.offset()].to_string();

        return Some(TokenType::NUMBER(number_string.parse().unwrap()));
    }

    // helpers
    fn peek(&mut self) -> Option<char> {
        self.chars.clone().next()
    }

    fn peek_next(&mut self) -> Option<char> {
        let mut copy_chars = self.chars.clone();
        if copy_chars.next().is_none() {
            return None;
        }
        copy_chars.next()
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
