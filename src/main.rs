mod ast;
mod lexer;
mod read_file;
mod token_types;

use ast::ast_builder::build_ast;
use lexer::lexer::tokenize;
use read_file::read_file::read_file;
use std::{env, process};
use token_types::TOKEN_TYPES;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        process::exit(1);
    }

    let file_name = &args[1];

    match read_file(file_name) {
        Ok(input) => {
            let tokens = tokenize(&input, &TOKEN_TYPES);
            let ast = build_ast(tokens);

            println!("{:#?}", ast);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    }
}
