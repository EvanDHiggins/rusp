use crate::tokenize::TokenStream;

#[derive(Debug)]
pub enum ASTNode {
    Terminal { value: String },
    Expression { children: Vec<ASTNode> },
}

pub fn parse(tokens: &mut TokenStream) -> Result<ASTNode, &'static str> {
    let next_tok = tokens.advance();
    if next_tok == "(" {
        let mut nodes = Vec::new();
        while tokens.peek() != ")" {
            nodes.push(parse(tokens).unwrap());
        }
        tokens.advance(); // Strip off trailing ')'
        Ok(ASTNode::Expression{children: nodes})
    } else {
        Ok(ASTNode::Terminal{value: next_tok})
    }
}
