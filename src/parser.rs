use crate::lexer::TokenStream;
use crate::lexer::Token;
use crate::lexer::TokenError;
use crate::error::InterpreterError;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Terminal { token: Token },
    FunctionCall { children: Vec<ASTNode> },
    Identifier { name: String },
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

fn read_token_or_fail(tokens: &mut dyn TokenStream) -> Result<Token, ParseError> {
    tokens.advance()?.ok_or_else(||
        ParseError::new(
            "Attempted to read the next token, but there are none left."))
}

fn consume_close_paren(tokens: &mut dyn TokenStream) -> Result<(), ParseError> {
    let tok = read_token_or_fail(tokens)?;
    match tok {
        Token::CloseParen => Ok(()),
        _ => Err(ParseError::new(format!(
                "Expected ')', but found {:?}.", tok).as_str()))
    }
}

fn next_is_close_paren(tokens: &mut dyn TokenStream) -> Result<bool, ParseError> {
    let next_tok = tokens.peek()?;
    Ok(matches!(next_tok, Some(Token::CloseParen)))
}

pub fn parse_expr(tokens: &mut dyn TokenStream) -> Result<ASTNode, ParseError> {
    let next_tok = read_token_or_fail(tokens).unwrap();

    if let Token::OpenParen = next_tok {
        let mut nodes = Vec::new();
        while !next_is_close_paren(tokens)? {
            nodes.push(parse_expr(tokens)?);
        }
        consume_close_paren(tokens)?;
        Ok(ASTNode::FunctionCall{children: nodes})
    } else if let Token::Id(identifier) = next_tok {
        Ok(ASTNode::Identifier{name: identifier})
    } else {
        Ok(ASTNode::Terminal{token: next_tok})
    }
}
