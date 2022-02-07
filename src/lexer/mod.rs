use crate::error::InterpreterError;

mod lazy_token_stream;
use lazy_token_stream::LazyTokenStream;

mod charstream;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    OpenParen,
    CloseParen,
    IntLiteral(i64),
    StringLiteral(String),
    Id(String),
}

pub trait TokenStream {
    fn advance(&mut self) -> Result<Option<Token>, TokenError>;
    fn peek(&mut self) -> Result<Option<Token>, TokenError>;
}

pub fn lex(s: &str) -> Box<dyn TokenStream> {
    Box::new(LazyTokenStream::new_from_string(s))
}

#[derive(Debug)]
pub struct TokenError {
    pub message: String,
}

impl From<TokenError> for InterpreterError {
    fn from(token_error: TokenError) -> InterpreterError {
        InterpreterError::new("Tokenization Error", &token_error.message)
    }
}

impl From<std::num::ParseIntError> for TokenError {
    fn from(parse_error: std::num::ParseIntError) -> TokenError {
        TokenError::new(format!("{}", parse_error))
    }
}

impl From<std::str::Utf8Error> for TokenError {
    fn from(utf8_error: std::str::Utf8Error) -> TokenError {
        TokenError::new(format!("{}", utf8_error))
    }
}

impl TokenError {
    fn new(message: String) -> TokenError {
        TokenError{message}
    }
}


