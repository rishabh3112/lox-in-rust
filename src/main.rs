mod ast;
mod parser;
mod scanner;
mod token;
mod tools;
mod visitors;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use ast::nodes::Binary;
use ast::nodes::Expr;
use ast::nodes::Literal;
use ast::traits::Visitor;
use parser::Parser;
use scanner::Scanner;
use tools::generate_ast;
use visitors::ast_printer::ASTPrinter;

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
        "print" => {
            let expr = Expr::Binary(Binary {
                left: Box::new(Expr::Literal(Literal {
                    literal: token::TokenType::NUMBER(2.0),
                })),
                operator: token::Token {
                    ty: token::TokenType::PLUS,
                    lexeme: "+".into(),
                },
                right: Box::new(Expr::Literal(Literal {
                    literal: token::TokenType::NUMBER(3.0),
                })),
            });

            let ast_printer = ASTPrinter::new();
            println!("{}", ast_printer.visit_expr(&expr))
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
            let has_errors = output.errors.len() > 0;

            for error in output.errors {
                eprintln!("{}", error);
            }

            let mut parser = Parser::new(&output.tokens);
            let expr = parser.parse();

            let ast_printer = ASTPrinter::new();
            println!("{}", ast_printer.visit_expr(&expr));

            if has_errors {
                exit(65)
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
