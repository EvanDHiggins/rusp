use crate::lexer::TokenStream;
use crate::lexer::Token;
use crate::lexer::TokenError;
use crate::error::InterpreterError;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Terminal { token: Token },
    Expression { children: Vec<ASTNode> },
    Identifier { name: String },
    Program { statements: Vec<ASTNode> },
    Define { id: Box<ASTNode>, defined_ast: Box<ASTNode> }
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
            "Attempted to read next token, but there are none left."))
}

fn peek_or_fail(tokens: &dyn TokenStream) -> Result<Token, ParseError> {
    tokens.peek()?.ok_or_else(||
        ParseError::new(
            "Attempted to read next token, but there are none left."))
}

fn parse_define(tokens: &mut dyn TokenStream) -> Result<ASTNode, ParseError> {
    // (define x (+ 1 2))
    // (define foo_func (lambda (x) (+ x 2)))
    read_token_or_fail(tokens)?; // Throw away "define" token.
    let defined_id = read_token_or_fail(tokens)?;
    let identifier_node = match defined_id {
        Token::Id(name) => Ok(ASTNode::Identifier{name}),
        _ => Err(ParseError::new("Expected id as first argument to 'define'."))
    }?;
    let defined_expr = parse_expr(tokens)?;
    consume_close_paren(tokens)?;
    Ok(ASTNode::Define{
        id: Box::new(identifier_node),
        defined_ast: Box::new(defined_expr)
    })
}

fn consume_close_paren(tokens: &mut dyn TokenStream) -> Result<(), ParseError> {
    let tok = read_token_or_fail(tokens)?;
    match tok {
        Token::CloseParen => Ok(()),
        _ => Err(ParseError::new(format!(
                "Expected ')', but found {:?}.", tok).as_str()))
    }
}

fn is_define(token: &Token) -> bool {
    match token {
        Token::Id(id) => id == "define",
        _ => false
    }
}

pub fn parse_expr(tokens: &mut dyn TokenStream) -> Result<ASTNode, ParseError> {
    let next_tok = read_token_or_fail(tokens)?;

    if let Token::OpenParen = next_tok {
        let first_arg = peek_or_fail(tokens)?;
        if is_define(&first_arg) {
            parse_define(tokens)
        } else {
            let mut nodes = Vec::new();
            while tokens.peek()?.is_some()
                && tokens.peek()?.unwrap() != Token::CloseParen {
                nodes.push(parse_expr(tokens)?);
            }
            tokens.advance()?; // Strip off trailing ')'
            Ok(ASTNode::Expression{children: nodes})
        }
    } else if let Token::Id(identifier) = next_tok {
        Ok(ASTNode::Identifier{name: identifier})
    } else {
        Ok(ASTNode::Terminal{token: next_tok})
    }
}
