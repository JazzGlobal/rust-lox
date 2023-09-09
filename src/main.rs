use crate::scanner::scanner::Scanner;
use crate::scanner::token::{Token, TokenType};

mod scanner;

fn main() {
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

    dbg!(scanner);
}

fn error(line: i32, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: i32, location: String, message: String) {
    let x = format!("[line {0}] Error {1} : {2}", line, location, message);
    println!("{}", x);
}
