#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Statement(String),
    Expression(String),
    // Add more node types as needed
}

pub mod ast_builder {
    use super::ASTNode;

    pub fn build_ast(tokens: Vec<(String, String)>) -> ASTNode {
        println!("{:#?}", tokens);
        ASTNode::Program(
            tokens
                .into_iter()
                .map(|(type_name, value)| ASTNode::Statement(format!("{}: {}", type_name, value)))
                .collect(),
        )
    }
}
