mod lexer;
mod read_file;
mod token_types;

use lexer::lexer::lexer;
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
            let tokens = lexer(&input, &TOKEN_TYPES);
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    }
}
