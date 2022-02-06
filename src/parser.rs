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
    Defun { 
        defined_name: String,
        defined_params: Vec<String>,
        exprs: Vec<ASTNode>
    }
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

impl ASTNode {
    fn is_identifier(&self) -> bool {
        match self {
            ASTNode::Identifier{name: _} => true,
            _ => false
        }
    }

    fn id_or_fail(&self) -> Result<String, ParseError> {
        match self {
            ASTNode::Identifier{name} => Ok(name.to_string()),
            _ => Err(
                ParseError::new(
                    &format!("Expected {:?} to be an id expression.", self)))
        }
    }
}

fn read_token_or_fail(tokens: &mut dyn TokenStream) -> Result<Token, ParseError> {
    tokens.advance()?.ok_or_else(||
        ParseError::new(
            "Attempted to read the next token, but there are none left."))
}

fn peek_or_fail(tokens: &dyn TokenStream) -> Result<Token, ParseError> {
    tokens.peek()?.ok_or_else(||
        ParseError::new(
            "Attempted to view next token, but there are none left."))
}

// Expects and parses a list of Id nodes from tokens. These are generally
// expected to be a list of arguments to a defun.
fn parse_id_list_expr(
    tokens: &mut dyn TokenStream) -> Result<Vec<String>, ParseError> {
    let id_exprs = parse_expr(tokens)?;
    let unwrapped_id_nodes = match id_exprs {
        ASTNode::FunctionCall{children} => Ok(children),
        _ => Err(
            ParseError::new(
                &format!(
                    "Expected ListExpr containing only ids. Found {:?}",
                     id_exprs)))
    }?;
    if unwrapped_id_nodes.iter().any(|node| !node.is_identifier()) {
        Err(ParseError::new(&format!(
                    "Found non-identifier node in expected list of ids: {:?}",
                    unwrapped_id_nodes)))
    } else {
        let mut ids = Vec::new();
        for id_node in unwrapped_id_nodes {
            ids.push(id_node.id_or_fail()?);
        }
        Ok(ids)
    }

}

fn expect_identifier(token: &Token) -> Result<String, ParseError> {
    match token {
        Token::Id(id) => Ok(id.to_owned()),
        _ => Err(
            ParseError::new(&format!("Expected id. Found {:?}.", token)))
    }
}

fn parse_defun(tokens: &mut dyn TokenStream) -> Result<ASTNode, ParseError> {
    // (defun x (y) (+ y 2))
    // (defun some_statements (s) (write s) (write "bar"))
    read_token_or_fail(tokens)?; // Throw away "define" token.
    let defined_id = expect_identifier(
        &read_token_or_fail(tokens)?)?;
    let params = parse_id_list_expr(tokens)?;

    let mut exprs = Vec::new();
    while !next_is_close_paren(tokens)? {
        exprs.push(parse_expr(tokens)?);
    }
    consume_close_paren(tokens)?;
    Ok(ASTNode::Defun{
        defined_name: defined_id,
        defined_params: params,
        exprs
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

fn is_defun(token: &Token) -> bool {
    match token {
        Token::Id(id) => id == "defun",
        _ => false
    }
}

fn next_is_close_paren(tokens: &dyn TokenStream) -> Result<bool, ParseError> {
    let next_tok = tokens.peek()?;
    Ok(matches!(next_tok, Some(Token::CloseParen)))
}

pub fn parse_expr(tokens: &mut dyn TokenStream) -> Result<ASTNode, ParseError> {
    let next_tok = read_token_or_fail(tokens).unwrap();

    if let Token::OpenParen = next_tok {
        let first_arg = peek_or_fail(tokens)?;
        if is_defun(&first_arg) {
            parse_defun(tokens)
        } else {
            let mut nodes = Vec::new();
            while !next_is_close_paren(tokens)? {
                nodes.push(parse_expr(tokens)?);
            }
            consume_close_paren(tokens)?;
            Ok(ASTNode::FunctionCall{children: nodes})
        }
    } else if let Token::Id(identifier) = next_tok {
        Ok(ASTNode::Identifier{name: identifier})
    } else {
        Ok(ASTNode::Terminal{token: next_tok})
    }
}
