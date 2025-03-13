use crate::token::Token;

/*
    Abstract Syntax Tree
    --------------------
    
    Uma binary tree onde os tokens são organizados em ordem de execução de cima para baixo.
*/

#[derive(Debug)]
pub struct AST {
    pub content: Token,
    pub left: Option<Box<AST>>,
    pub right: Option<Box<AST>>
}

impl AST {
    pub fn evaluate(tree: &Option<Box<AST>>) -> f64 {
        if tree.is_none() {
            return 0.0f64;
        }

        let tree = tree.as_ref().unwrap();


        return match &tree.content {
            Token::Number(num) => num.parse().unwrap(),
            Token::Addition => AST::evaluate(&tree.left) + AST::evaluate(&tree.right),
            Token::Subtraction => AST::evaluate(&tree.left) - AST::evaluate(&tree.right),
            Token::Division => AST::evaluate(&tree.left) / AST::evaluate(&tree.right),
            Token::Multiplication => AST::evaluate(&tree.left) * AST::evaluate(&tree.right),
            Token::Invalid(_) => 0.0f64
        }
    }
}