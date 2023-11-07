use crate::token_types::TokenType as Token;

// #[derive(Debug)]
// enum ASTNode {
//     Program(Vec<ASTNode>),
//     Assignment(Token, Box<ASTNode>),
//     BinaryOp(Token, Box<ASTNode>, Box<ASTNode>),
//     Identifier(Token),
//     Number(Token),

// }

pub mod ast_builder {
    use crate::token_types::Tokens;

    // use super::ASTNode;

    pub fn build_ast(tokens: Vec<(Tokens)>) {
        // println!("Token {:#?}", tokens);

        println!("Token {:#?}", tokens);

        // println!("{:#?}", tokens);
        // ASTNode::Program(
        //     tokens
        //         .into_iter()
        //         .map(|(type_name, value)| ASTNode::Statement(format!("{}: {}", type_name, value)))
        //         .collect(),
        // )
    }
}
