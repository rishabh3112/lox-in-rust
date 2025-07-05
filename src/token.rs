use std::fmt::{self, Debug, Display, Formatter};

use crate::literal::Literal;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Token {
    pub fn new(
        ty: TokenType,
        literal: Option<Literal>,
        lexeme: Option<String>,
        line: Option<usize>,
    ) -> Self {
        Self {
            ty,
            lexeme: lexeme.unwrap_or(String::new()),
            literal: literal.unwrap_or(Literal::Nil),
            line: line.unwrap_or(1),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.ty.name(),
            self.lexeme,
            self.literal.token_print()
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    StringLit,
    NumberLit,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

impl TokenType {
    pub fn name(&self) -> String {
        match self {
            TokenType::LeftParen => String::from("LEFT_PAREN"),
            TokenType::RightParen => String::from("RIGHT_PAREN"),
            TokenType::LeftBrace => String::from("LEFT_BRACE"),
            TokenType::RightBrace => String::from("RIGHT_BRACE"),
            TokenType::Comma => String::from("COMMA"),
            TokenType::Dot => String::from("DOT"),
            TokenType::Minus => String::from("MINUS"),
            TokenType::Plus => String::from("PLUS"),
            TokenType::SemiColon => String::from("SEMICOLON"),
            TokenType::Slash => String::from("SLASH"),
            TokenType::Star => String::from("STAR"),
            TokenType::Bang => String::from("BANG"),
            TokenType::BangEqual => String::from("BANG_EQUAL"),
            TokenType::Equal => String::from("EQUAL"),
            TokenType::EqualEqual => String::from("EQUAL_EQUAL"),
            TokenType::Greater => String::from("GREATER"),
            TokenType::GreaterEqual => String::from("GREATER_EQUAL"),
            TokenType::Less => String::from("LESS"),
            TokenType::LessEqual => String::from("LESS_EQUAL"),
            TokenType::Identifier => String::from("IDENTIFIER"),
            TokenType::StringLit => String::from("STRING"),
            TokenType::NumberLit => String::from("NUMBER"),
            TokenType::And => String::from("AND"),
            TokenType::Class => String::from("CLASS"),
            TokenType::Else => String::from("ELSE"),
            TokenType::False => String::from("FALSE"),
            TokenType::Fun => String::from("FUN"),
            TokenType::For => String::from("FOR"),
            TokenType::If => String::from("IF"),
            TokenType::Nil => String::from("NIL"),
            TokenType::Or => String::from("OR"),
            TokenType::Print => String::from("PRINT"),
            TokenType::Return => String::from("RETURN"),
            TokenType::Super => String::from("SUPER"),
            TokenType::This => String::from("THIS"),
            TokenType::True => String::from("TRUE"),
            TokenType::Var => String::from("VAR"),
            TokenType::While => String::from("WHILE"),
            TokenType::EOF => String::from("EOF"),
        }
    }

    pub fn get_keyword_token_type(lexeme: String) -> Option<TokenType> {
        match lexeme.as_str() {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "fun" => Some(TokenType::Fun),
            "for" => Some(TokenType::For),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}
