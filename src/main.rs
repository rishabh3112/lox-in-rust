mod ast;
mod error;
mod interpreter;
mod literal;
mod parser;
mod scanner;
mod token;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        &String::from("test.lox")
    };

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "failed to read file {}", filename).unwrap();
        String::new()
    });

    let mut scanner = Scanner::new(&file_contents);

    let output = scanner.run();
    let mut has_errors = output.errors.len() > 0;
    let mut error_code = 65;

    for error in output.errors {
        error.log();
    }

    let mut parser = Parser::new(&output.tokens);
    let mut interpreter = Interpreter::new();

    match parser.parse() {
        Ok(statements) => match interpreter.interpret(&statements) {
            Ok(_result) => {}
            Err(error) => {
                // runtime error
                has_errors = true;
                error.log();
                error_code = 70;
            }
        },
        Err(_errors) => {
            // compiler time error
            has_errors = true;
        }
    };

    if has_errors {
        exit(error_code)
    }
}
