use crate::tokenize::TokenStream;
use crate::tokenize::Token;

#[derive(Debug)]
pub enum ASTNode {
    Terminal { token: Token },
    Expression { children: Vec<ASTNode> },
}

pub fn parse(tokens: &mut TokenStream) -> Result<ASTNode, &'static str> {
    let next_tok = tokens.advance();
    if next_tok == Token::OpenParen {
        let mut nodes = Vec::new();
        while tokens.peek() != Token::CloseParen {
            nodes.push(parse(tokens).unwrap());
        }
        tokens.advance(); // Strip off trailing ')'
        Ok(ASTNode::Expression{children: nodes})
    } else {
        Ok(ASTNode::Terminal{token: next_tok})
    }
}
