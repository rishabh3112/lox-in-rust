use crate::{
    ast::nodes::{Binary, Expr, Grouping, Literal, Unary},
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
        self.factor()
    }

    fn factor(&mut self) -> Expr {
        let mut left = self.unary();
        while self.match_token(STAR) || self.match_token(SLASH) {
            left = Expr::Binary(Binary {
                left: Box::new(left),
                operator: self.previous().clone(),
                right: Box::new(self.unary()),
            })
        }

        left
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(BANG) || self.match_token(MINUS) {
            return Expr::Unary(Unary {
                operator: self.previous().clone(),
                right: Box::new(self.unary()),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(LEFT_PAREN) {
            if !self.match_token(RIGHT_PAREN) {
                // Handle syntax error here
                self.advance();
            }

            return Expr::Grouping(Grouping {
                expression: Box::new(self.expression()),
            });
        }
        let literal = self.advance().ty.clone();
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
