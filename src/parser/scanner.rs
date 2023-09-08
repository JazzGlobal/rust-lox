use std::fmt::format;
use crate::parser::token::{Token, TokenType};

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: i32,
}

impl Scanner {
    pub fn scan_tokens(&mut self) {
        self.start = 0;
        self.current = 0;
        self.line = 1;

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
        });
    }

    fn scan_token(&mut self) {
        let current_char: char = self.advance();
        match current_char {
            '(' => {self.add_token(TokenType::LEFT_PAREN, current_char.to_string())}
            ')' => {self.add_token(TokenType::RIGHT_PAREN, current_char.to_string())}
            '{' => {self.add_token(TokenType::LEFT_BRACE, current_char.to_string())}
            '}' => {self.add_token(TokenType::RIGHT_BRACE, current_char.to_string())}
            ',' => {self.add_token(TokenType::COMMA, current_char.to_string())}
            '.' => {self.add_token(TokenType::DOT, current_char.to_string())}
            '-' => {self.add_token(TokenType::MINUS, current_char.to_string())}
            '+' => {self.add_token(TokenType::PLUS, current_char.to_string())}
            ';' => {self.add_token(TokenType::SEMICOLON, current_char.to_string())}
            '*' => {self.add_token(TokenType::STAR, current_char.to_string())}
            '!' => {
                match self.match_value('=') {
                    true => { self.add_token(TokenType::BANG_EQUAL, "!=".to_string()) },
                    false => { self.add_token(TokenType::BANG, "!".to_string()) },
                }
            }
            '=' => {
                match self.match_value('=') {
                    true => { self.add_token(TokenType::EQUAL_EQUAL, "==".to_string()) },
                    false => { self.add_token(TokenType::EQUAL, "=".to_string()) }
                }
            }
            '<' => {
                match self.match_value('=') {
                    true => { self.add_token(TokenType::LESS_EQUAL, "<=".to_string()) }
                    false => { self.add_token(TokenType::LESS, "<".to_string()) }
                }
            }
            '>' => {
                match self.match_value('=') {
                    true => { self.add_token(TokenType::GREATER_EQUAL, ">=".to_string()) }
                    false => { self.add_token(TokenType::GREATER, ">".to_string()) }
                }
            }

            default =>
                {
                    let x = format!("Unexpected character {0} on Line {1}", default, self.line);
                    println!("{}", x);
                }
        };
    }

    fn add_token(&mut self, token_type: TokenType, literal: String) {
        dbg!(&self);

        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
        });
    }

    fn is_at_end(&self) -> bool {
        return &self.current >= &self.source.len()
    }

    fn advance(&mut self) -> char {
        let result = self.source.chars().nth(self.current).unwrap_or('|');
        self.current += 1;
        result
    }

    fn match_value(&mut self, expected: char) -> bool {
        // dbg!("Inside match_value: {0}", &expected);
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap_or('|') != expected
        {
            return false
        }
        self.current += 1;
        true
    }
}

pub fn create_scanner(source: String) -> Scanner {
    Scanner {source, tokens: vec![], current: 0 , start: 0, line: 1}
}