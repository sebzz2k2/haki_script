pub mod lexer {
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

#[cfg(test)]
mod tests {
    use super::lexer::{lexer, TokenType};

    #[test]
    fn test_lexer() {
        let token_types = vec![
            TokenType::new("KEYWORD", r"^(let)"),
            TokenType::new("NUMBER", r"^\d+"),
            TokenType::new("IDENTIFIER", r"^[a-zA-Z_]\w*"),
            TokenType::new("OPERATOR", r"^[+\-*/]"),
            TokenType::new("PUNCTUATION", r"^[,;(){}]"),
            TokenType::new("EQUAL", r"^="),
        ];
        let input = "let x = y ; get";
        let tokens = lexer(input, &token_types);

        let expected_tokens = vec![
            ("KEYWORD", "let"),
            ("IDENTIFIER", "x"),
            ("EQUAL", "="),
            ("IDENTIFIER", "y"),
            ("PUNCTUATION", ";"),
            ("IDENTIFIER", "get"),
        ];
        let tokens_as_str: Vec<(&str, &str)> = tokens
            .iter()
            .map(|(t, v)| (t.as_str(), v.as_str()))
            .collect();
        assert_eq!(tokens_as_str, expected_tokens)
    }
}
