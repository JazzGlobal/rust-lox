use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
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
    STRING,
    NUMBER,

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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            TokenType::LEFT_PAREN => {
                write!(f, "LEFT_PAREN")
            }
            TokenType::RIGHT_PAREN => {
                write!(f, "RIGHT_PAREN")
            }
            TokenType::LEFT_BRACE => {
                write!(f, "LEFT_BRACE")
            }
            TokenType::RIGHT_BRACE => {
                write!(f, "RIGHT_PAREN")
            }
            TokenType::COMMA => {
                write!(f, "COMMA")
            }
            TokenType::DOT => {
                write!(f, "DOT")
            }
            TokenType::MINUS => {
                write!(f, "MINUS")
            }
            TokenType::PLUS => {
                write!(f, "PLUS")
            }
            TokenType::SEMICOLON => {
                write!(f, "SEMICOLON")
            }
            TokenType::SLASH => {
                write!(f, "SLASH")
            }
            TokenType::STAR => {
                write!(f, "STAR")
            }
            TokenType::BANG => {
                write!(f, "BANG")
            }
            TokenType::BANG_EQUAL => {
                write!(f, "BANG_EQUAL")
            }
            TokenType::EQUAL => {
                write!(f, "EQUAL")
            }
            TokenType::EQUAL_EQUAL => {
                write!(f, "EQUAL_EQUAL")
            }
            TokenType::GREATER => {
                write!(f, "GREATER")
            }
            TokenType::GREATER_EQUAL => {
                write!(f, "GREATER_EQUAL")
            }
            TokenType::LESS => {
                write!(f, "LESS")
            }
            TokenType::LESS_EQUAL => {
                write!(f, "LESS_EQUAL")
            }
            TokenType::IDENTIFIER => {
                write!(f, "IDENTIFIER")
            }
            TokenType::STRING => {
                write!(f, "STRING")
            }
            TokenType::NUMBER => {
                write!(f, "NUMBER")
            }
            TokenType::AND => {
                write!(f, "AND")
            }
            TokenType::CLASS => {
                write!(f, "CLASS")
            }
            TokenType::ELSE => {
                write!(f, "ELSE")
            }
            TokenType::FALSE => {
                write!(f, "FALSE")
            }
            TokenType::FUN => {
                write!(f, "FUN")
            }
            TokenType::FOR => {
                write!(f, "FOR")
            }
            TokenType::IF => {
                write!(f, "IF")
            }
            TokenType::NIL => {
                write!(f, "NIL")
            }
            TokenType::OR => {
                write!(f, "OR")
            }
            TokenType::PRINT => {
                write!(f, "PRINT")
            }
            TokenType::RETURN => {
                write!(f, "RETURN")
            }
            TokenType::SUPER => {
                write!(f, "SUPER")
            }
            TokenType::THIS => {
                write!(f, "THIS")
            }
            TokenType::TRUE => {
                write!(f, "TRUE")
            }
            TokenType::VAR => {
                write!(f, "VAR")
            }
            TokenType::WHILE => {
                write!(f, "WHILE")
            }
            TokenType::EOF => {
                write!(f, "EOF")
            }
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: i32,
    pub col: i32,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "{0} {1} {2} {3}",
                &self.token_type, &self.lexeme, &self.literal, &self.col
            )
        )
    }
}
