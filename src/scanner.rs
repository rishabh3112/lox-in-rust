use crate::token::{Token, TokenType};
use std::str::Chars;

pub struct Scanner<'a> {
    source: &'a String,
    chars: Chars<'a>,
    tokens: Vec<Token>,
    current: usize,
    size: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Self {
        let size = source.len();
        //        let graphemes = source
        //            .graphemes(true)
        //            .map(|s| s.to_string())
        //            .collect::<Vec<String>>();
        Self {
            source,
            chars: source.chars(),
            tokens: vec![],
            current: 0,
            size,
        }
    }

    pub fn scan_tokens(self: &mut Self) -> &Vec<Token> {
        while let Some(char) = self.chars.next() {
            match char {
                '(' => self.add_token(TokenType::LEFT_PAREN),
                ')' => self.add_token(TokenType::RIGHT_PAREN),
                _ => {}
            }
        }
        self.tokens.push(Token {
            ty: TokenType::EOF,
            lexeme: String::new(),
            line: 0,
        });

        &self.tokens
    }

    pub fn add_token(self: &mut Self, ty: TokenType) {
        self.tokens.push(Token {
            ty,
            lexeme: String::new(),
            line: 0,
        });
    }
}
