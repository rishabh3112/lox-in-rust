mod ast;
mod error;
mod parser;
mod scanner;
mod token;
mod tools;
mod visitors;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use ast::traits::ExprVisitor;
use parser::Parser;
use scanner::Scanner;
use tools::generate_ast;
use visitors::ast_printer::ASTPrinter;
use visitors::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut scanner = Scanner::new(&file_contents);

            let output = scanner.run();
            let has_errors = output.errors.len() > 0;

            for error in output.errors {
                eprintln!("{}", error);
            }

            for token in output.tokens {
                println!("{}", token)
            }

            if has_errors {
                exit(65)
            }
        }
        "generate" => match filename.as_str() {
            "ast" => generate_ast(),
            _ => {}
        },
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut scanner = Scanner::new(&file_contents);

            let output = scanner.run();
            let mut has_errors = output.errors.len() > 0;

            for error in output.errors {
                eprintln!("{}", error);
            }

            let mut parser = Parser::new(&output.tokens);
            let ast_printer = ASTPrinter::new();

            match parser.expression() {
                Ok(expr) => println!("{}", ast_printer.visit_expr(&expr)),
                Err(error) => {
                    eprintln!("{}", error);
                    has_errors = true;
                }
            }

            if has_errors {
                exit(65)
            }
        }
        "evaluate" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut scanner = Scanner::new(&file_contents);

            let output = scanner.run();
            let mut has_errors = output.errors.len() > 0;
            let mut error_code = 65;

            for error in output.errors {
                // compile time error - during scanning
                eprintln!("{}", error);
            }

            let mut parser = Parser::new(&output.tokens);
            let interpreter = Interpreter {};

            match parser.parse() {
                Ok(statements) => match interpreter.interpret(&statements) {
                    Ok(result) => {}
                    Err(error) => {
                        // runtime error
                        has_errors = true;
                        eprintln!("{}", error);
                        error_code = 70;
                    }
                },
                Err(error) => {
                    // compiler time error
                    has_errors = true;
                    eprintln!("{}", error);
                }
            };

            if has_errors {
                exit(error_code)
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
