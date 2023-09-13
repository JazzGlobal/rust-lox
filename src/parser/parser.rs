use crate::parser::expression::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, LoxType, UnaryExpr};
use crate::scanner::token::{Token, TokenType};
use std::ptr::eq;
use std::str::FromStr;
use crate::parser::expression::Expr::{GROUP_EXPR, LITERAL_EXPR};
use crate::parser::expression::LoxType::{LoxBoolean, LoxNil, LoxNumber, LoxString};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub errors: Vec<String>,
}

impl Parser {

    pub fn parse(&mut self) -> Expr {
        let x = self.expression();
        x
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut left: Expr = self.comparison();
        while self.peek().token_type == TokenType::BANG_EQUAL ||
              self.peek().token_type == TokenType::EQUAL_EQUAL {
            self.advance();
            let operator = self.previous();
            let right = self.comparison();
            left = Expr::BINARY_EXPR(BinaryExpr {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            })
        }
        left
    }

    fn comparison(&mut self) -> Expr {
        let mut left: Expr = self.term();
        while self.peek().token_type == TokenType::GREATER_EQUAL ||
              self.peek().token_type == TokenType::GREATER ||
              self.peek().token_type == TokenType::LESS ||
              self.peek().token_type == TokenType::LESS_EQUAL
        {
            self.advance();
            let operator = self.previous();
            let right = self.term();

            left = Expr::BINARY_EXPR(BinaryExpr {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            })
        }
        left
    }

    fn term(&mut self) -> Expr {
        let mut left = self.factor();
        while self.peek().token_type == TokenType::MINUS ||
            self.peek().token_type == TokenType::PLUS
        {
            self.advance();
            let operator = self.previous();
            let right = self.factor();
            left = Expr::BINARY_EXPR(BinaryExpr {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            })
        }
        left
    }

    fn factor(&mut self) -> Expr {
        let mut left = self.unary();

        while self.peek().token_type == TokenType::SLASH ||
              self.peek().token_type == TokenType::STAR {
            self.advance();
            let operator = self.previous();
            let right = self.unary();
            left = Expr::BINARY_EXPR(BinaryExpr {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        left
    }

    fn unary(&mut self) -> Expr {
        if self.peek().token_type == TokenType::BANG ||
            self.peek().token_type == TokenType::MINUS
        {
            self.advance();
            let operator = self.previous();
            let right = self.unary();
            return Expr::UNARY_EXPR(UnaryExpr { operator, right: Box::new(right) })
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        match self.peek().token_type {
            TokenType::FALSE =>
                {
                    self.advance();
                    return LITERAL_EXPR(LiteralExpr {value: LoxBoolean(Some(false))})
                }
            TokenType::TRUE => {
                self.advance();
                return LITERAL_EXPR(LiteralExpr {value: LoxBoolean(Some(true))})
            }
            TokenType::NIL => {
                self.advance();
                return LITERAL_EXPR(LiteralExpr {value: LoxType::LoxNil})
            }
            TokenType::NUMBER =>
                {
                    self.advance();
                    let x = self.previous().literal.unwrap();
                    let y = &str::parse::<f64>(&x).unwrap();
                    return LITERAL_EXPR(LiteralExpr { value: LoxNumber(Some(*y)) })
                }
            TokenType::STRING => {
                self.advance();
                return LITERAL_EXPR(LiteralExpr { value: LoxString(Some(self.previous().literal.unwrap_or("".to_string()))) })
            }
            TokenType::LEFT_PAREN =>
                {
                    self.advance();
                    let mut expr = self.expression();
                    self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.".to_string());
                    return GROUP_EXPR(GroupingExpr { expression: Box::new((expr)) })
                }
            _ => {}
        }
        panic!("Expect expression.");
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Token {
        if self.check(token_type)
        {
            return self.advance()
        }
        let error = format!("{0} at end. {1}", self.peek(), message);
        self.errors.push(error);
        return self.advance();
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token_type;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        let y = self.previous().to_owned();
        y
    }

    fn is_at_end(&mut self) -> bool {
        let end_of_file = self.peek().token_type == TokenType::EOF;
        end_of_file
    }

    fn peek(&mut self) -> &Token {
        let peekToken = self.tokens.get(self.current).unwrap();
        peekToken
    }

    fn previous(&mut self) -> Token {
        return self.tokens.get(self.current - 1).unwrap().to_owned();
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON { return; }

            match self.peek().token_type {
                TokenType::CLASS | TokenType::FUN | TokenType::VAR | TokenType::FOR |
                TokenType::IF | TokenType::WHILE | TokenType::PRINT | TokenType::RETURN => { return; }
                _ => { self.advance(); }
            }
        }
    }
}

pub fn create_parser(tokens: Vec<Token>) -> Parser {
    Parser { tokens, current: 0, errors: vec![]}
}