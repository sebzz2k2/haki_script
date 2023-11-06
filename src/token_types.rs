pub struct TokenType {
    pub type_name: &'static str,
    pub regex_pattern: &'static str,
}

impl TokenType {
    pub const fn new(type_name: &'static str, regex_pattern: &'static str) -> Self {
        TokenType {
            type_name,
            regex_pattern,
        }
    }
}

pub const TOKEN_TYPES: [TokenType; 6] = [
    TokenType::new("KEYWORD", r"^(let)"),
    TokenType::new("NUMBER", r"^\d+"),
    TokenType::new("IDENTIFIER", r"^[a-zA-Z_]\w*"),
    TokenType::new("OPERATOR", r"^[+\-*/]"),
    TokenType::new("PUNCTUATION", r"^[,;(){}]"),
    TokenType::new("EQUAL", r"^="),
];
