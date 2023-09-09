use crate::parser::token::{Token, TokenType};
use std::fmt::format;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: i32,
    current_col: i32,
}

impl Scanner {
    pub fn scan_tokens(&mut self) {
        self.start = 0;
        self.current = 0;
        self.line = 1;
        self.current_col = 0;

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        // TODO: Determine tokens.
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
            col: self.current_col,
        });
    }

    fn scan_token(&mut self) {
        let current_char: char = self.advance();
        match current_char {
            '(' => self.add_token(TokenType::LEFT_PAREN, Some(current_char.to_string())),
            ')' => self.add_token(TokenType::RIGHT_PAREN, Some(current_char.to_string())),
            '{' => self.add_token(TokenType::LEFT_BRACE, Some(current_char.to_string())),
            '}' => self.add_token(TokenType::RIGHT_BRACE, Some(current_char.to_string())),
            ',' => self.add_token(TokenType::COMMA, Some(current_char.to_string())),
            '.' => self.add_token(TokenType::DOT, Some(current_char.to_string())),
            '-' => self.add_token(TokenType::MINUS, Some(current_char.to_string())),
            '+' => self.add_token(TokenType::PLUS, Some(current_char.to_string())),
            ';' => self.add_token(TokenType::SEMICOLON, Some(current_char.to_string())),
            '*' => self.add_token(TokenType::STAR, Some(current_char.to_string())),
            '!' => match self.match_value('=') {
                true => self.add_token(TokenType::BANG_EQUAL, Some("!=".to_string())),
                false => self.add_token(TokenType::BANG, Some("!".to_string())),
            },
            '=' => match self.match_value('=') {
                true => self.add_token(TokenType::EQUAL_EQUAL, Some("==".to_string())),
                false => self.add_token(TokenType::EQUAL, Some("=".to_string())),
            },
            '<' => match self.match_value('=') {
                true => self.add_token(TokenType::LESS_EQUAL, Some("<=".to_string())),
                false => self.add_token(TokenType::LESS, Some("<".to_string())),
            },
            '>' => match self.match_value('=') {
                true => self.add_token(TokenType::GREATER_EQUAL, Some(">=".to_string())),
                false => self.add_token(TokenType::GREATER, Some(">".to_string())),
            },
            '/' => match self.match_value('/') {
                true => {
                    while self.peek(None) != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => self.add_token(TokenType::SLASH, Some("/".to_string())),
            },
            '\n' => {
                self.line += 1;
                self.current_col = 0;
            }
            ' ' | '\r' | '\t' => return,

            '\"' => {
                self.handle_string_literal();
            }

            default => {
                if default.is_numeric() {
                    self.handle_number();
                } else if default.is_alphabetic() || default == '_' {
                    self.handle_identifier();
                } else {
                    let x = format!(
                        "Unexpected character {0} on Line {1}, col {2}",
                        default, self.line, self.current_col
                    );
                    println!("{}", x);
                }
            }
        };
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
            col: self.current_col,
        });
    }

    fn is_at_end(&self) -> bool {
        return &self.current >= &self.source.len();
    }

    fn advance(&mut self) -> char {
        let result = self.source.chars().nth(self.current).unwrap_or('|');
        self.current += 1;
        self.current_col += 1;
        result
    }

    fn match_value(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap_or('|') != expected {
            return false;
        }
        self.current += 1;
        self.current_col += 1;
        true
    }

    fn peek(&self, by_num: Option<i32>) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return match by_num {
            None => self.source.chars().nth(self.current).unwrap_or('|'),
            Some(num) => {
                if self.current + (num as usize) >= self.source.len() {
                    return '\0';
                }
                self.source
                    .chars()
                    .nth(self.current + (num as usize))
                    .unwrap_or('|')
            }
        };
    }

    fn handle_string_literal(&mut self) {
        while self.peek(None) != '\"' && !self.is_at_end() {
            if self.peek(None) == '\n' {
                self.line += 1;
                self.current_col = 0;
            }
            self.advance();
        }

        if self.is_at_end() {
            println!(
                "Unterminated string. Line {0} Col {1}",
                self.line, self.current_col
            );
            return;
        }

        self.advance();
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::STRING, Some(value));
    }

    fn handle_number(&mut self) {
        while self.peek(None).is_numeric() {
            self.advance();
        }
        if self.peek(None) == '.' && self.peek(Some(1)).is_numeric() {
            self.advance();
            while self.peek(None).is_numeric() {
                self.advance();
            }
        }
        self.add_token(
            TokenType::NUMBER,
            Some(self.source[self.start..self.current].to_string()),
        )
    }

    fn handle_identifier(&mut self) {
        while self.peek(None).is_alphanumeric() || self.peek(None) == '_' {
            self.advance();
        }

        let code_substring = &self.source[self.start..self.current];
        // This might end up being a bug since we're pushing an TokenType::IDENTIFIER on error.
        self.add_token(
            TokenType::from_str(code_substring).unwrap_or(TokenType::IDENTIFIER),
            None,
        );
    }
}

pub fn create_scanner(source: String) -> Scanner {
    Scanner {
        source,
        tokens: vec![],
        current: 0,
        start: 0,
        line: 1,
        current_col: 0,
    }
}
