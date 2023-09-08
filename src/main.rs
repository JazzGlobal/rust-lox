use crate::scanner::Scanner;

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
    let mut line = String::new();
    println!(">>");
    // We'll likely need some error handling here.
    std::io::stdin().read_line(&mut line).unwrap();
    run(line);
}

// I wonder if this should return a Result to handle errors in the event that the function was called from
// run_prompt.
fn run(source: String) {
    let scanner = Scanner { source };
    let tokens = scanner.scan_tokens();
}
