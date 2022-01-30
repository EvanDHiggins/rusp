use crate::lexer::TokenStream;
use crate::lexer::Token;
use crate::lexer::TokenError;
use crate::error::InterpreterError;

#[derive(Debug)]
pub enum ASTNode {
    Terminal { token: Token },
    Expression { children: Vec<ASTNode> },
    Program { statements: Vec<ASTNode> },
}

#[derive(Debug)]
pub struct ParseError {
    message: String
}

impl From<TokenError> for ParseError {
    fn from(token_error: TokenError) -> ParseError {
        ParseError{message: token_error.message}
    }
}

impl From<ParseError> for InterpreterError {
    fn from(parse_error: ParseError) -> InterpreterError {
        InterpreterError::new("ParseError", &parse_error.message)
    }
}

impl ParseError {
    fn new(message: &str) -> ParseError {
        ParseError{message: String::from(message)}
    }
}

pub fn parse(tokens: &mut dyn TokenStream) -> Result<ASTNode, ParseError> {
    let mut statements = Vec::new();
    while tokens.peek()?.is_some() {
        let expr = parse_expr(tokens)?;
        statements.push(expr);
    }
    Ok(ASTNode::Program{statements})
}

pub fn parse_expr(tokens: &mut dyn TokenStream) -> Result<ASTNode, ParseError> {
    let next_tok = tokens.advance()?.ok_or_else(||
        ParseError::new(
            "Attempted to read next token, but there are none left."))?;
    if next_tok == Token::OpenParen {
        let mut nodes = Vec::new();
        while tokens.peek()?.is_some()
            && tokens.peek()?.unwrap() != Token::CloseParen {
            nodes.push(parse_expr(tokens).unwrap());
        }
        tokens.advance()?; // Strip off trailing ')'
        Ok(ASTNode::Expression{children: nodes})
    } else {
        Ok(ASTNode::Terminal{token: next_tok})
    }
}
