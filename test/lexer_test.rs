#[cfg(test)]
mod tests {

    #[test]
    fn test_lexer() {
        // Import the lexer function and the TokenType struct
        use haki_script::lexer::lexer;
        use haki_script::lexer::TokenType;
        // Define your token types
        let token_types = vec![
            TokenType::new("KEYWORD", r"^(let)"),
            TokenType::new("NUMBER", r"^\d+"),
            TokenType::new("IDENTIFIER", r"^[a-zA-Z_]\w*"),
            TokenType::new("OPERATOR", r"^[+\-*/]"),
            TokenType::new("PUNCTUATION", r"^[,;(){}]"),
            TokenType::new("EQUAL", r"^="),
            TokenType::new("KEYWORD", r"^(let|get)"),
        ];

        // Test input
        let input = "let x = y ; get";

        // Call the lexer function
        let tokens = lexer(input, &token_types);

        // Define the expected tokens
        let expected_tokens = vec![
            ("KEYWORD", "let"),
            ("IDENTIFIER", "x"),
            ("OPERATOR", "="),
            ("IDENTIFIER", "y"),
            ("PUNCTUATION", ";"),
            ("KEYWORD", "get"),
        ];

        // Compare the tokens generated by the lexer with the expected tokens
        assert_eq!(tokens, expected_tokens);
    }
}