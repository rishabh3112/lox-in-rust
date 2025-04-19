use std::fmt::{self, Display, Formatter};

pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
}

impl Token {
    pub fn new(ty: TokenType, lexeme: Option<String>) -> Self {
        Self {
            ty,
            lexeme: lexeme.unwrap_or(String::new()),
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
            self.ty.literal()
        )
    }
}

#[derive(PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING(String),
    NUMBER(f32),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ Token: type = {}, literal = {} ]",
            self.name(),
            self.literal()
        )
    }
}

impl TokenType {
    fn name(&self) -> String {
        match self {
            TokenType::LEFT_PAREN => String::from("LEFT_PAREN"),
            TokenType::RIGHT_PAREN => String::from("RIGHT_PAREN"),
            TokenType::LEFT_BRACE => String::from("LEFT_BRACE"),
            TokenType::RIGHT_BRACE => String::from("RIGHT_BRACE"),
            TokenType::COMMA => String::from("COMMA"),
            TokenType::DOT => String::from("DOT"),
            TokenType::MINUS => String::from("MINUS"),
            TokenType::PLUS => String::from("PLUS"),
            TokenType::SEMICOLON => String::from("SEMICOLON"),
            TokenType::SLASH => String::from("SLASH"),
            TokenType::STAR => String::from("STAR"),
            TokenType::BANG => String::from("BANG"),
            TokenType::BANG_EQUAL => String::from("BANG_EQUAL"),
            TokenType::EQUAL => String::from("EQUAL"),
            TokenType::EQUAL_EQUAL => String::from("EQUAL_EQUAL"),
            TokenType::GREATER => String::from("GREATER"),
            TokenType::GREATER_EQUAL => String::from("GREATER_EQUAL"),
            TokenType::LESS => String::from("LESS"),
            TokenType::LESS_EQUAL => String::from("LESS_EQUAL"),
            TokenType::IDENTIFIER => String::from("IDENTIFIER"),
            TokenType::STRING(_) => String::from("STRING"),
            TokenType::NUMBER(_) => String::from("NUMBER"),
            TokenType::AND => String::from("AND"),
            TokenType::CLASS => String::from("CLASS"),
            TokenType::ELSE => String::from("ELSE"),
            TokenType::FALSE => String::from("FALSE"),
            TokenType::FUN => String::from("FUN"),
            TokenType::FOR => String::from("FOR"),
            TokenType::IF => String::from("IF"),
            TokenType::NIL => String::from("NIL"),
            TokenType::OR => String::from("OR"),
            TokenType::PRINT => String::from("PRINT"),
            TokenType::RETURN => String::from("RETURN"),
            TokenType::SUPER => String::from("SUPER"),
            TokenType::THIS => String::from("THIS"),
            TokenType::TRUE => String::from("TRUE"),
            TokenType::VAR => String::from("VAR"),
            TokenType::WHILE => String::from("WHILE"),
            TokenType::EOF => String::from("EOF"),
        }
    }

    pub fn get_keyword_token_type(lexeme: String) -> Option<TokenType> {
        match lexeme.as_str() {
            "and" => Some(TokenType::AND),
            "class" => Some(TokenType::CLASS),

            "else" => Some(TokenType::ELSE),
            "false" => Some(TokenType::FALSE),
            "fun" => Some(TokenType::FUN),
            "for" => Some(TokenType::FOR),
            "if" => Some(TokenType::IF),
            "nil" => Some(TokenType::NIL),
            "or" => Some(TokenType::OR),
            "print" => Some(TokenType::PRINT),
            "return" => Some(TokenType::RETURN),
            "super" => Some(TokenType::SUPER),
            "this" => Some(TokenType::THIS),
            "true" => Some(TokenType::TRUE),
            "var" => Some(TokenType::VAR),
            "while" => Some(TokenType::WHILE),
            _ => None,
        }
    }

    fn literal(&self) -> String {
        match self {
            TokenType::STRING(str) => str.clone(),
            TokenType::NUMBER(num) => format!("{:?}", num),
            _ => String::from("null"),
        }
    }
}
