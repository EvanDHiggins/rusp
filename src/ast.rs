use crate::tokenize::TokenStreamOld;
use crate::tokenize::Token;

#[derive(Debug)]
pub enum ASTNode {
    Terminal { token: Token },
    Expression { children: Vec<ASTNode> },
    Program { statements: Vec<ASTNode> },
}

pub fn parse(tokens: &mut TokenStreamOld) -> Result<ASTNode, String> {
    let mut statements = Vec::new();
    while tokens.peek().is_some() {
        let expr = parse_expr(tokens)?;
        statements.push(expr);
    }
    Ok(ASTNode::Program{statements: statements})
}

pub fn parse_expr(tokens: &mut TokenStreamOld) -> Result<ASTNode, String> {
    let next_tok = tokens.advance().ok_or("No more tokens available.")?;
    if next_tok == Token::OpenParen {
        let mut nodes = Vec::new();
        while tokens.peek().is_some()
            && tokens.peek().unwrap() != Token::CloseParen {
            nodes.push(parse_expr(tokens).unwrap());
        }
        tokens.advance(); // Strip off trailing ')'
        Ok(ASTNode::Expression{children: nodes})
    } else {
        Ok(ASTNode::Terminal{token: next_tok})
    }
}
