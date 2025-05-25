use crate::{
    ast::nodes::{
        Assign, Binary, BlockStmt, Expr, ExpressionStmt, ForStmt, Grouping, IfStmt, Lit, Logical,
        PrintStmt, Stmt, Unary, Variable, VariableDeclarationStmt, WhileStmt,
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

            return Ok(Stmt::Variable(VariableDeclarationStmt {
                token,
                initializer,
            }));
        }

        self.statement()
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.match_token(For) {
            return self.for_statement();
        }

        if self.match_token(If) {
            return self.if_statement();
        }

        if self.match_token(Print) {
            return self.print_statement();
        }

        if self.match_token(While) {
            return self.while_statement();
        }

        if self.match_token(LeftBrace) {
            return Ok(Stmt::Block(BlockStmt {
                statements: self.block()?,
            }));
        }

        self.expression_statement()
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.check(RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?)
        }

        if !self.match_token(RightBrace) {
            return Err(self.error("Expect '}' after block."));
        }

        Ok(statements)
    }

    fn for_statement(&mut self) -> Result<Stmt, LoxError> {
        if !self.match_token(LeftParen) {
            return Err(self.error("Expect '(' after 'if'."));
        }

        let mut initializer: Option<Box<Stmt>> = None;
        let mut condition: Option<Expr> = None;
        let mut increment: Option<Expr> = None;

        if self.match_token(SemiColon) {
        } else if self.match_token(Var) {
            initializer = Some(Box::new(self.variable_declaration()?));
        } else {
            initializer = Some(Box::new(self.expression_statement()?));
        }

        if !self.match_token(SemiColon) {
            condition = Some(self.expression()?);

            if !self.match_token(SemiColon) {
                return Err(self.error("Expect ';' after loop condition."));
            }
        }

        if !self.match_token(RightParen) {
            increment = Some(self.expression()?);

            if !self.match_token(RightParen) {
                return Err(self.error("Expect ')' after for clauses."));
            }
        }

        Ok(Stmt::For(ForStmt {
            initializer,
            condition,
            body: Box::new(self.statement()?),
            increment,
        }))
    }

    fn if_statement(&mut self) -> Result<Stmt, LoxError> {
        if !self.match_token(LeftParen) {
            return Err(self.error("Expect '(' after 'if'."));
        }

        let condition = self.expression()?;

        if !self.match_token(RightParen) {
            return Err(self.error("Expect ')' after 'if'."));
        }

        let then_branch = Box::new(self.statement()?);

        if self.match_token(Else) {
            return Ok(Stmt::If(IfStmt {
                condition,
                then_branch,
                else_branch: Some(Box::new(self.statement()?)),
            }));
        }

        return Ok(Stmt::If(IfStmt {
            condition,
            then_branch,
            else_branch: None,
        }));
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let result = self.expression()?;
        if !self.match_token(SemiColon) {
            return Err(self.error("Expect ; after expression."));
        }

        Ok(Stmt::Print(PrintStmt { expression: result }))
    }

    fn while_statement(&mut self) -> Result<Stmt, LoxError> {
        if !self.match_token(LeftParen) {
            return Err(self.error("Expect '(' after 'if'."));
        }

        let condition = self.expression()?;

        if !self.match_token(RightParen) {
            return Err(self.error("Expect ')' after 'if'."));
        }

        return Ok(Stmt::While(WhileStmt {
            condition,
            body: Box::new(self.statement()?),
        }));
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let result = self.expression()?;
        if !self.match_token(SemiColon) {
            return Err(self.error("Expect ; after expression."));
        }

        Ok(Stmt::Expression(ExpressionStmt { expression: result }))
    }

    pub fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let left = self.or();

        if self.match_token(Equal) {
            let value = self.assignment()?;

            if let Ok(expr) = &left {
                if let Expr::Variable(variable) = expr {
                    return Ok(Expr::Assign(Assign {
                        token: variable.token.clone(),
                        value: Box::new(value),
                    }));
                }
            }

            return Err(self.error("Invalid assignment target."));
        }

        left
    }

    fn or(&mut self) -> Result<Expr, LoxError> {
        let mut left = self.and();

        while self.match_token(Or) {
            let operator = self.previous().clone();
            let right = self.and()?;
            left = Ok(Expr::Logical(Logical {
                operator,
                left: Box::new(left?),
                right: Box::new(right),
            }));
        }

        left
    }

    fn and(&mut self) -> Result<Expr, LoxError> {
        let mut left = self.equality();

        while self.match_token(And) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            left = Ok(Expr::Logical(Logical {
                operator,
                left: Box::new(left?),
                right: Box::new(right),
            }));
        }

        left
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
