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
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut left = self.comparison();
        while self.match_token(EqualEqual) || self.match_token(BangEqual) {
            left = Expr::Binary(Binary {
                left: Box::new(left),
                operator: self.previous().clone(),
                right: Box::new(self.comparison()),
            })
        }

        left
    }

    fn comparison(&mut self) -> Expr {
        let mut left = self.term();
        while self.match_token(Greater)
            || self.match_token(GreaterEqual)
            || self.match_token(Less)
            || self.match_token(LessEqual)
        {
            left = Expr::Binary(Binary {
                left: Box::new(left),
                operator: self.previous().clone(),
                right: Box::new(self.term()),
            })
        }

        left
    }

    fn term(&mut self) -> Expr {
        let mut left = self.factor();
        while self.match_token(Plus) || self.match_token(Minus) {
            left = Expr::Binary(Binary {
                left: Box::new(left),
                operator: self.previous().clone(),
                right: Box::new(self.factor()),
            })
        }

        left
    }

    fn factor(&mut self) -> Expr {
        let mut left = self.unary();
        while self.match_token(Star) || self.match_token(Slash) {
            left = Expr::Binary(Binary {
                left: Box::new(left),
                operator: self.previous().clone(),
                right: Box::new(self.unary()),
            })
        }

        left
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(Bang) || self.match_token(Minus) {
            return Expr::Unary(Unary {
                operator: self.previous().clone(),
                right: Box::new(self.unary()),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(LeftParen) {
            let group = Expr::Grouping(Grouping {
                expression: Box::new(self.expression()),
            });

            if !self.match_token(RightParen) {
                // Handle syntax error here
            }

            return group;
        }
        let literal = self.advance().ty.clone();
        Expr::Literal(Literal { literal })
    }

    // helpers

    fn match_token(&mut self, ty: TokenType) -> bool {
        // println!("match {} with {}", ty.name(), self.peek().ty.name());
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
