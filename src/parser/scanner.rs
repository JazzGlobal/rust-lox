use crate::parser::token::{Token, TokenType};
use std::fmt::format;

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
            literal: "".to_string(),
            line: self.line,
            col: self.current_col,
        });
    }

    fn scan_token(&mut self) {
        let current_char: char = self.advance();
        match current_char {
            '(' => self.add_token(TokenType::LEFT_PAREN, current_char.to_string()),
            ')' => self.add_token(TokenType::RIGHT_PAREN, current_char.to_string()),
            '{' => self.add_token(TokenType::LEFT_BRACE, current_char.to_string()),
            '}' => self.add_token(TokenType::RIGHT_BRACE, current_char.to_string()),
            ',' => self.add_token(TokenType::COMMA, current_char.to_string()),
            '.' => self.add_token(TokenType::DOT, current_char.to_string()),
            '-' => self.add_token(TokenType::MINUS, current_char.to_string()),
            '+' => self.add_token(TokenType::PLUS, current_char.to_string()),
            ';' => self.add_token(TokenType::SEMICOLON, current_char.to_string()),
            '*' => self.add_token(TokenType::STAR, current_char.to_string()),
            '!' => match self.match_value('=') {
                true => self.add_token(TokenType::BANG_EQUAL, "!=".to_string()),
                false => self.add_token(TokenType::BANG, "!".to_string()),
            },
            '=' => match self.match_value('=') {
                true => self.add_token(TokenType::EQUAL_EQUAL, "==".to_string()),
                false => self.add_token(TokenType::EQUAL, "=".to_string()),
            },
            '<' => match self.match_value('=') {
                true => self.add_token(TokenType::LESS_EQUAL, "<=".to_string()),
                false => self.add_token(TokenType::LESS, "<".to_string()),
            },
            '>' => match self.match_value('=') {
                true => self.add_token(TokenType::GREATER_EQUAL, ">=".to_string()),
                false => self.add_token(TokenType::GREATER, ">".to_string()),
            },
            '/' => match self.match_value('/') {
                true => {
                    while self.peek(None) != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => self.add_token(TokenType::SLASH, "/".to_string()),
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
                if default.is_numeric()
                {
                    self.handle_number();
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

    fn add_token(&mut self, token_type: TokenType, literal: String) {
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
            None => { self.source.chars().nth(self.current).unwrap_or('|') }
            Some(num) =>
                {
                    if self.current + (num as usize) >= self.source.len() {
                        return '\0';
                    }
                    self.source.chars().nth(self.current + (num as usize)).unwrap_or('|')
                }
        }
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
        self.add_token(TokenType::STRING, value);
    }

    fn handle_number(&mut self) {
        while self.peek(None).is_numeric()
        {
            self.advance();
        }
        if self.peek(None) == '.' && self.peek(Some(1)).is_numeric() {
            self.advance();
            while self.peek(None).is_numeric() {
                self.advance();
            }
        }
        self.add_token(TokenType::NUMBER, self.source[self.start..self.current].to_string())
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
