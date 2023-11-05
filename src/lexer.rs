pub mod lexer{
    use regex::Regex;
pub struct TokenType {
    pub type_name: &'static str,
    pub regex: Regex,
}

impl TokenType {
    pub fn new(type_name: &'static str, pattern: &str) -> TokenType {
        let regex = Regex::new(pattern).expect("Invalid regex pattern");
        TokenType { type_name, regex }
    }
}

pub fn lexer(input: &str, token_types: &[TokenType]) -> Vec<(String, String)> {
    let mut tokens = Vec::new();
    let lines: Vec<&str> = input.lines().collect();

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        for word in words {
            for token_type in token_types {
                if token_type.regex.is_match(word) {
                    tokens.push((token_type.type_name.to_string(), word.to_string()));
                    break;
                }
            }
        }
    }

    tokens
}
}