use crate::parser::expression;
use crate::parser::expression::LoxType::LoxNumber;
use crate::parser::expression::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, LoxType, UnaryExpr};
use crate::parser::parser::Parser;
use crate::scanner::scanner::Scanner;
use crate::scanner::token::{Token, TokenType};

mod parser;
mod scanner;

fn main() {
    // Expression Pretty Printing.
    // let x = Expr::BINARY_EXPR(BinaryExpr {
    //     left: Box::new(Expr::UNARY_EXPR(UnaryExpr { operator: Token {
    //         token_type: TokenType::MINUS,
    //         lexeme: "-".to_string(),
    //         literal: None,
    //         line: 0,
    //         col: 0,
    //     }, right: Box::new(Expr::LITERAL_EXPR(LiteralExpr { value: LoxNumber(Some(123 as f64)) })) })),
    //     operator: Token {
    //         token_type: TokenType::STAR,
    //         lexeme: "*".to_string(),
    //         literal: None,
    //         line: 0,
    //         col: 0,
    //     },
    //     right: Box::new(Expr::GROUP_EXPR(GroupingExpr{ expression: Box::new(Expr::LITERAL_EXPR(LiteralExpr { value: LoxNumber(Some(45.67)) })) })),
    // });
    // expression::handle_expr(x);

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        run_prompt();
    }
}

fn run_file(file_path: &String) {
    let bytes = match std::fs::read(file_path) {
        Ok(v) => v,
        Err(e) => {
            let msg = format!(
                "An error occurred while reading the source file {0}: {1}",
                file_path, e
            );
            panic!("{}", msg);
        }
    };
    let s = match std::str::from_utf8(&bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 Sequence: {}", e),
    };
    run(s.to_string());
}

fn run_prompt() {
    loop {
        let mut line = String::new();
        println!(">>");
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {
                run(line);
            }
            Err(_) => {
                println!("An error occurred, exiting REPL.");
                break;
            }
        };
    }
}

// I wonder if this should return a Result to handle errors in the event that the function was called from
// run_prompt.
fn run(source: String) {
    let mut scanner = scanner::scanner::create_scanner(source);
    scanner.scan_tokens();

    dbg!(scanner.get_tokens());
    let mut parser = parser::parser::create_parser(scanner.get_tokens());
    let x = parser.parse();
    dbg!(&x);
    if !parser.errors.is_empty() {
        for error in parser.errors {
            println!("{}", error);
        }
        panic!("Parser errors.");
    }
    let x = parser.interpret(x);
    if !parser.errors.is_empty() {
        for error in parser.errors {
            println!("ERROR: {0}", error);
        }
    } else {
        dbg!(x);
    }
}

fn error(line: i32, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: i32, location: String, message: String) {
    let x = format!("[line {0}] Error {1} : {2}", line, location, message);
    println!("{}", x);
}
