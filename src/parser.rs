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

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        let literal = self.peek().ty.clone();
        if self.match_token(LEFT_PAREN) {
            let expr = self.expression();
            if !self.match_token(RIGHT_PAREN) {
                // Handle syntax error here
                self.advance();
            }

            return expr;
        }
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
