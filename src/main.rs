mod lexer;
use crate::lexer::lexer::TokenType;
use lexer::lexer::lexer;

fn main() {
    let token_types = vec![
        TokenType::new("KEYWORD", r"^(let)"),
        TokenType::new("NUMBER", r"^\d+"),
        TokenType::new("IDENTIFIER", r"^[a-zA-Z_]\w*"),
        TokenType::new("OPERATOR", r"^[+\-*/]"),
        TokenType::new("PUNCTUATION", r"^[,;(){}]"),
        TokenType::new("EQUAL", r"^="),
        TokenType::new("KEYWORD", r"^(let|get)"),
    ];

    let input = "let x = y ; get";
    let tokens = lexer(input, &token_types);

    for token in tokens {
        println!("{:?}", token);
    }
}
