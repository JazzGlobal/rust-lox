use crate::parser::expression::Expr::{BINARY_EXPR, GROUP_EXPR, LITERAL_EXPR, UNARY_EXPR};
use crate::parser::expression::LoxType::{LoxBoolean, LoxNil, LoxNumber, LoxString};
use crate::parser::expression::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, LoxType, UnaryExpr};
use crate::scanner::token::{Token, TokenType};
use std::any::type_name;
use std::fmt::{Display, Formatter};
use std::ptr::eq;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParserError {
    pub error_message: String,
    pub line: i32,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ERROR on Line {}: {}", self.line, self.error_message)
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub errors: Vec<ParserError>,
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
        while self.peek().token_type == TokenType::BANG_EQUAL
            || self.peek().token_type == TokenType::EQUAL_EQUAL
        {
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
        while self.peek().token_type == TokenType::GREATER_EQUAL
            || self.peek().token_type == TokenType::GREATER
            || self.peek().token_type == TokenType::LESS
            || self.peek().token_type == TokenType::LESS_EQUAL
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
        while self.peek().token_type == TokenType::MINUS
            || self.peek().token_type == TokenType::PLUS
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

        while self.peek().token_type == TokenType::SLASH
            || self.peek().token_type == TokenType::STAR
        {
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
        if self.peek().token_type == TokenType::BANG || self.peek().token_type == TokenType::MINUS {
            self.advance();
            let operator = self.previous();
            let right = self.unary();
            return Expr::UNARY_EXPR(UnaryExpr {
                operator,
                right: Box::new(right),
            });
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        match self.peek().token_type {
            TokenType::FALSE => {
                return LITERAL_EXPR(LiteralExpr {
                    value: LoxBoolean(Some(false)),
                })
            }
            TokenType::TRUE => {
                self.advance();
                return LITERAL_EXPR(LiteralExpr {
                    value: LoxBoolean(Some(true)),
                });
            }
            TokenType::NIL => {
                self.advance();
                return LITERAL_EXPR(LiteralExpr {
                    value: LoxType::LoxNil,
                });
            }
            TokenType::NUMBER => {
                self.advance();
                let x = self.previous().literal.unwrap();
                let y = &str::parse::<f64>(&x).unwrap();
                return LITERAL_EXPR(LiteralExpr {
                    value: LoxNumber(Some(*y)),
                });
            }
            TokenType::STRING => {
                self.advance();
                return LITERAL_EXPR(LiteralExpr {
                    value: LoxString(Some(self.previous().literal.unwrap_or("".to_string()))),
                });
            }
            TokenType::LEFT_PAREN => {
                self.advance();
                let mut expr = self.expression();
                self.consume(
                    TokenType::RIGHT_PAREN,
                    "Expect ')' after expression.".to_string(),
                );
                return GROUP_EXPR(GroupingExpr {
                    expression: Box::new((expr)),
                });
            }
            _ => {}
        }
        panic!("Expect expression.");
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Token {
        if self.check(token_type) {
            return self.advance();
        }
        let error = ParserError {
            error_message: format!("{0} at end. {1}", self.peek(), message),
            line: self.current as i32,
        };
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
            if self.previous().token_type == TokenType::SEMICOLON {
                dbg!("return because we hit a SEMICOLON");
                return;
            }

            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    // TODO: This will probably have to be included in the parser struct so we can append errors as we find them.
    pub fn interpret(&mut self, expr: Expr) -> LoxType {
        return match expr {
            BINARY_EXPR(binary) => {
                let left = self.interpret(*binary.left);
                let right = self.interpret(*binary.right);

                // Support equality checks between two different types.
                if !variant_eq(&left, &right) {
                    return match binary.operator.token_type {
                        TokenType::BANG_EQUAL => LoxBoolean(Some(true)),
                        TokenType::EQUAL_EQUAL => LoxBoolean(Some(false)),
                        _ => {
                            let error = ParserError {
                                error_message: format!(
                                    "{} not supported for types {} and {}.",
                                    binary.operator.literal.unwrap(),
                                    left,
                                    right
                                ),
                                line: self.current as i32,
                            };
                            self.errors.push(error);
                            LoxNil
                        }
                    };
                }

                return match left {
                    LoxNumber(left) => {
                        if let LoxNumber(num) = right {
                            let x = left.unwrap(); // left
                            let y = num.unwrap(); // right
                            return match binary.operator.token_type {
                                TokenType::MINUS => LoxNumber(Some(x - y)),
                                TokenType::STAR => LoxNumber(Some(x * y)),
                                TokenType::SLASH => LoxNumber(Some(x / y)),
                                TokenType::PLUS => LoxNumber((Some(x + y))),
                                TokenType::GREATER => LoxBoolean(Some(x > y)),
                                TokenType::GREATER_EQUAL => LoxBoolean(Some(x >= y)),
                                TokenType::LESS => LoxBoolean(Some(x < y)),
                                TokenType::LESS_EQUAL => LoxBoolean(Some(x <= y)),
                                TokenType::EQUAL_EQUAL => LoxBoolean(Some(x == y)),
                                TokenType::BANG_EQUAL => LoxBoolean(Some(x != y)),
                                _ => LoxNil,
                            };
                        }
                        LoxNil
                    }
                    LoxString(left) => {
                        if let LoxString(text) = right {
                            let x = left.unwrap();
                            let y = text.unwrap();
                            return match binary.operator.token_type {
                                TokenType::PLUS => LoxString((Some(format!("{0}{1}", x, y)))),
                                _ => LoxNil,
                            };
                        }
                        LoxNil
                    }
                    _ => LoxNil,
                };
            }
            UNARY_EXPR(unary) => {
                let right = self.interpret(*unary.right);
                match unary.operator.token_type {
                    TokenType::MINUS => {
                        match right {
                            LoxNumber(rust_num) => {
                                let default = 0 as f64;
                                LoxNumber(Some(-rust_num.unwrap_or(default)))
                            }
                            _ => LoxNil, // TODO: Should we error out here? Should probably tell the programmer what they did wrong.
                        }
                    }
                    TokenType::BANG => match !is_truthy(right) {
                        true => LoxBoolean(Some(true)),
                        false => LoxBoolean(Some(false)),
                    },
                    _ => LoxNil,
                }
            }
            LITERAL_EXPR(literal) => literal.value,
            GROUP_EXPR(grouping) => self.interpret(*grouping.expression),
        };
    }
}

pub fn create_parser(tokens: Vec<Token>) -> Parser {
    Parser {
        tokens,
        current: 0,
        errors: vec![],
    }
}

fn variant_eq(a: &LoxType, b: &LoxType) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn evaluate(expr: Expr) -> LoxType {
    accept(expr)
}

fn accept(expr: Expr) -> LoxType {
    LoxNil
}

/// LoxNil and LoxBool(false) are always false.
/// Everything else is always true.
fn is_truthy(lox_type: LoxType) -> bool {
    return match lox_type {
        LoxNil => false,
        LoxBoolean(rust_bool) => rust_bool.unwrap_or(false),
        _ => true,
    };
}
