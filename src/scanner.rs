use crate::{
    error::LoxError,
    literal::Literal,
    token::{Token, TokenType},
};
use std::str::Chars;

pub struct Scanner<'a> {
    source: &'a String,
    chars: Chars<'a>,
    line: usize,
    errors: Vec<LoxError>,
    start: usize,
}

pub struct ScannerOutput {
    pub tokens: Vec<Token>,
    pub errors: Vec<LoxError>,
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
                tokens.push(Token::new(ty, None, None, Some(self.line)));
                break;
            }

            tokens.push(self.create_token(ty));
        }

        let output = ScannerOutput {
            tokens,
            errors: self.errors.clone(),
        };

        output
    }

    fn create_token(&mut self, ty: TokenType) -> Token {
        let raw = self.source[self.start..self.offset()].to_string();
        let lexeme = raw.clone();
        let literal = match ty {
            TokenType::StringLit => Literal::String(raw[1..raw.len() - 1].to_string()),
            TokenType::NumberLit => Literal::Number(raw.parse().unwrap()),
            TokenType::False | TokenType::True => Literal::Boolean(raw.parse().unwrap()),
            _ => Literal::Nil,
        };

        Token::new(ty, Some(literal), Some(lexeme), Some(self.line))
    }

    fn read_next_token(self: &mut Self) -> TokenType {
        while let Some(char) = self.chars.next() {
            match char {
                '(' => return TokenType::LeftParen,
                ')' => return TokenType::RightParen,
                '{' => return TokenType::LeftBrace,
                '}' => return TokenType::RightBrace,
                '-' => return TokenType::Minus,
                '+' => return TokenType::Plus,
                ',' => return TokenType::Comma,
                '.' => return TokenType::Dot,
                ';' => return TokenType::SemiColon,
                '*' => return TokenType::Star,
                '!' => {
                    if self.match_next('=') {
                        return TokenType::BangEqual;
                    }
                    return TokenType::Bang;
                }
                '=' => {
                    if self.match_next('=') {
                        return TokenType::EqualEqual;
                    }
                    return TokenType::Equal;
                }
                '>' => {
                    if self.match_next('=') {
                        return TokenType::GreaterEqual;
                    };
                    return TokenType::Greater;
                }
                '<' => {
                    if self.match_next('=') {
                        return TokenType::LessEqual;
                    };
                    return TokenType::Less;
                }
                '/' => {
                    if self.match_next('/') {
                        self.ignore_line();
                    } else {
                        return TokenType::Slash;
                    }
                }
                '"' => {
                    if let Some(ty) = self.match_string() {
                        return ty;
                    }
                }
                ' ' | '\t' | '\r' => self.start += 1,
                '\n' => {
                    self.line += 1;
                    self.start += 1;
                }
                _ => {
                    if char.is_ascii_digit() {
                        if let Some(ty) = self.match_number() {
                            return ty;
                        }
                    } else if char.is_ascii_alphabetic() || char == '_' {
                        if let Some(ty) = self.match_identifier() {
                            return ty;
                        }
                    } else {
                        self.start += 1;
                        self.errors.push(LoxError::Scanner {
                            line: self.line,
                            message: format!("Unexpected character: {}", char),
                        })
                    }
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
            self.errors.push(LoxError::Scanner {
                line: self.line,
                message: "Unterminated string.".into(),
            });
            return None;
        }

        self.chars.next();

        return Some(TokenType::StringLit);
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

        return Some(TokenType::NumberLit);
    }

    fn match_identifier(&mut self) -> Option<TokenType> {
        while let Some(next) = self.peek() {
            if next.is_ascii_alphanumeric() || next == '_' {
                self.chars.next();
                continue;
            }
            break;
        }

        let lexeme = self.source[self.start..self.offset()].to_string();
        if let Some(ty) = TokenType::get_keyword_token_type(lexeme) {
            return Some(ty);
        }
        return Some(TokenType::Identifier);
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
