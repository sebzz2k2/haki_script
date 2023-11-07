pub mod lexer {
    use crate::token_types::TokenType;
    use crate::token_types::Tokens;
    use regex::Regex;

    pub fn tokenize(input: &str, token_types: &[TokenType]) -> Vec<(Tokens)> {
        let mut tokens = Vec::new();
        let lines: Vec<&str> = input.lines().collect();

        let re = Regex::new(r"(.*[^\s;])\;$").unwrap();

        for line in lines {
            let line = if line.ends_with(";") {
                line.to_string()
            } else {
                format!("{} ;", line)
            };

            let line = if re.is_match(&line) {
                re.replace(&line, "$1 ;").to_string()
            } else {
                line.to_string()
            };
            let words: Vec<&str> = line.split_whitespace().collect();
            for word in words {
                for token_type in token_types {
                    if Regex::new(&token_type.regex_pattern)
                        .unwrap()
                        .is_match(word)
                    {
                        tokens.push(Tokens {
                            token_type: token_type.type_name.to_string(),
                            value: word.to_string(),
                        });
                        // tokens.push((token_type.type_name.to_string(), word.to_string()));
                        break;
                    }
                }
            }
        }

        tokens
    }
}

// #[cfg(test)]
// mod tests {
//     use super::lexer::tokenize;
//     use crate::token_types::TokenType;

//     #[test]
//     fn test_lexer() {
//         let token_types = vec![
//             TokenType::new("KEYWORD", r"^(let)"),
//             TokenType::new("NUMBER", r"^\d+"),
//             TokenType::new("IDENTIFIER", r"^[a-zA-Z_]\w*"),
//             TokenType::new("OPERATOR", r"^[+\-*/]"),
//             TokenType::new("PUNCTUATION", r"^[,;(){}]"),
//             TokenType::new("EQUAL", r"^="),
//         ];
//         let input = "let x = y ; get";
//         let tokens = tokenize(input, &token_types);

//         let expected_tokens = vec![
//             ("KEYWORD", "let"),
//             ("IDENTIFIER", "x"),
//             ("EQUAL", "="),
//             ("IDENTIFIER", "y"),
//             ("PUNCTUATION", ";"),
//             ("IDENTIFIER", "get"),
//         ];
//         let tokens_as_str: Vec<(&str, &str)> = tokens
//             .iter()
//             .map(|(t, v)| (t.as_str(), v.as_str()))
//             .collect();
//         assert_eq!(tokens_as_str, expected_tokens)
//     }
// }
