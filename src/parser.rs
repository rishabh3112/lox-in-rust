use crate::{
    ast::nodes::{
        Binary, Expr, ExpressionStmt, Grouping, Lit, PrintStmt, Stmt, Unary, Variable,
        VariableDeclaration,
    },
    error::LoxError,
    token::{
        Literal, Token,
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<LoxError>> {
        let mut statements: Vec<Stmt> = vec![];
        let mut errors: Vec<LoxError> = vec![];
        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(error) => {
                    error.log();
                    errors.push(error);
                }
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        let result = if self.match_token(Var) {
            self.variable_declaration()
        } else {
            self.statement()
        };

        match result {
            Ok(declaration) => Ok(declaration),
            Err(error) => {
                self.synchronize();
                Err(error)
            }
        }
    }

    fn variable_declaration(&mut self) -> Result<Stmt, LoxError> {
        if self.match_token(Identifier) {
            let token = self.previous().clone();
            let mut initializer: Expr = Expr::Literal(Lit {
                literal: Literal::Nil,
            });

            if self.match_token(Equal) {
                initializer = self.expression()?;
            }

            if !self.match_token(SemiColon) {
                return Err(self.error("Expect ; after variable declaration."));
            }

            return Ok(Stmt::Variable(VariableDeclaration { token, initializer }));
        }

        self.statement()
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.match_token(Print) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let result = self.expression()?;
        if !self.match_token(SemiColon) {
            return Err(self.error("Expect ; after expression."));
        }

        Ok(Stmt::Print(PrintStmt { expression: result }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let result = self.expression()?;
        if !self.match_token(SemiColon) {
            return Err(self.error("Expect ; after expression."));
        }

        Ok(Stmt::Expression(ExpressionStmt { expression: result }))
    }

    pub fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
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

    fn comparison(&mut self) -> Result<Expr, LoxError> {
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

    fn term(&mut self) -> Result<Expr, LoxError> {
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

    fn factor(&mut self) -> Result<Expr, LoxError> {
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

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.match_token(Bang) || self.match_token(Minus) {
            return Ok(Expr::Unary(Unary {
                operator: self.previous().clone(),
                right: Box::new(self.unary()?),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
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
                return Err(self.error("Expect ')' after expression."));
            }

            return Ok(group);
        }

        if self.match_token(Identifier) {
            return Ok(Expr::Variable(Variable {
                token: self.previous().clone(),
            }));
        }

        Err(self.error("Expect expression"))
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

    fn error(&mut self, message: &str) -> LoxError {
        let err_token = self.peek().clone();

        LoxError::Parser {
            token: err_token,
            message: message.into(),
        }
    }

    fn synchronize(&mut self) {
        loop {
            if self.previous().ty == SemiColon {
                return;
            }

            match self.peek().ty {
                Class | For | Fun | Var | If | While | Print | Return => return,
                _ => {}
            }

            if self.is_at_end() {
                return;
            }
            self.advance();
        }
    }
}
