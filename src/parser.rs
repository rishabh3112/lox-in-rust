use crate::{
    ast::nodes::{Binary, Expr, Literal},
    token::{
        Token,
        TokenType::{self, *},
    },
};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&self) -> Expr {
        self.literal()
    }

    fn expression(&self) -> Expr {
        self.literal()
    }

    fn literal(&self) -> Expr {
        let literal = self.peek().ty.clone();
        Expr::Literal(Literal { literal })
    }

    // helpers

    fn match_token(&mut self, ty: TokenType) -> bool {
        if self.check(ty) {
            self.advance();
            return true;
        }
        false
    }

    fn check(&self, ty: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().ty == ty
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.peek().ty == EOF
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
}
