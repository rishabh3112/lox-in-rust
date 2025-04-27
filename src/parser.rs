use crate::{
    ast::nodes::{Binary, Expr, Grouping, Lit, Unary},
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

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut left = self.comparison()?;
        while self.match_token(EqualEqual) || self.match_token(BangEqual) {
            left = Expr::Binary(Binary {
                left: Box::new(left),
                operator: self.previous().clone(),
                right: Box::new(self.comparison()?),
            })
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.term()?;
        while self.match_token(Greater)
            || self.match_token(GreaterEqual)
            || self.match_token(Less)
            || self.match_token(LessEqual)
        {
            left = Expr::Binary(Binary {
                left: Box::new(left),
                operator: self.previous().clone(),
                right: Box::new(self.term()?),
            })
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut left = self.factor()?;
        while self.match_token(Plus) || self.match_token(Minus) {
            left = Expr::Binary(Binary {
                left: Box::new(left),
                operator: self.previous().clone(),
                right: Box::new(self.factor()?),
            })
        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut left = self.unary()?;
        while self.match_token(Star) || self.match_token(Slash) {
            left = Expr::Binary(Binary {
                left: Box::new(left),
                operator: self.previous().clone(),
                right: Box::new(self.unary()?),
            })
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(Bang) || self.match_token(Minus) {
            return Ok(Expr::Unary(Unary {
                operator: self.previous().clone(),
                right: Box::new(self.unary()?),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(True)
            || self.match_token(False)
            || self.match_token(Nil)
            || self.match_token(NumberLit)
            || self.match_token(StringLit)
        {
            return Ok(Expr::Literal(Lit {
                literal: self.previous().literal.clone(),
            }));
        }

        if self.match_token(LeftParen) {
            let group = Expr::Grouping(Grouping {
                expression: Box::new(self.expression()?),
            });

            if !self.match_token(RightParen) {
                return self.error("Expect ')' after expression.");
            }

            return Ok(group);
        }

        self.error("Expect expression")
    }

    // helpers
    fn match_token(&mut self, ty: TokenType) -> bool {
        //println!("match {} with {}", ty.name(), self.peek().ty.name());
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
        self.current += 1;
        self.previous()
    }

    fn error(&mut self, message: &str) -> Result<Expr, String> {
        let err_token = self.peek();

        if self.is_at_end() {
            return Err(format!(
                "[line {}] Error at end: {}",
                err_token.line, message
            ));
        }

        Err(format!(
            "[line {}] Error at '{}': {}",
            err_token.line, err_token.lexeme, message
        ))
    }
}
