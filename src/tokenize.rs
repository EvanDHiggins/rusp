use crate::error::InterpreterError;

#[derive(Debug)]
pub struct NaiveTokenStream {
    curr: usize,
    tokens: Vec<Token>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    OpenParen,
    CloseParen,
    IntLiteral(i64),
    StringLiteral(String),
    Id(String),
}

pub trait TokenStream {
    fn advance(&mut self) -> Option<Token>;
    fn peek(&self) -> Option<Token>;
}

impl Token {
    fn from_string(str_tok: &str) -> Token {
        if str_tok == "(" {
            Token::OpenParen
        } else if str_tok == ")" {
            Token::CloseParen
        } else if is_string_literal(str_tok) {
            let str_len = str_tok.len();
            Token::StringLiteral(
                str_tok.chars().skip(1).take(str_len-2).collect())
        } else if is_integer(str_tok) {
            Token::IntLiteral(str_tok.parse::<i64>().unwrap())
        } else {
            Token::Id(str_tok.to_owned())
        }
    }
}

pub struct LazyTokenStream {
    input: String,
    curr: usize,
    next_token: Option<Token>,
}

pub struct TokenError {
    message: String,
}

impl From<TokenError> for InterpreterError {
    fn from(token_error: TokenError) -> InterpreterError {
        InterpreterError::new("Tokenization Error", &token_error.message)
    }
}

impl TokenError {
    fn new(message: String) -> TokenError {
        TokenError{message: message}
    }
}

impl TokenStream for LazyTokenStream {
    fn advance(&mut self) -> Option<Token> {
        Option::None
    }

    fn peek(&self) -> Option<Token> {
        Option::None
    }
}



impl LazyTokenStream {
    pub fn new(input: &str) -> Result<LazyTokenStream, TokenError> {
        LazyTokenStream{
            input: input.to_owned(),
            curr: 0,
            next_token: Option::None
        }.init()
    }

    fn init(mut self) -> Result<LazyTokenStream, TokenError> {
        if self.input.as_bytes()[self.curr] != ('(' as u8) {
            Err(TokenError::new(format!(
                    "Expected program to begin with '('. Found {} instead.",
                    self.input.as_bytes()[self.curr])))
        } else {
            self.curr += 1;
            self.next_token = Some(Token::OpenParen);
            Ok(self)
        }
    }
}

pub fn tokenize(s: &str) -> Result<LazyTokenStream, TokenError> {
    LazyTokenStream::new(s)
}

fn is_integer(s: &str) -> bool {
    s.parse::<i64>().is_ok()
}

fn is_string_literal(s: &str) -> bool {
    let begins_with_quote =
        s.chars().nth(0).map(|c| c == '"').filter(|&b| b).is_some();
    let ends_with_quote =
        s.chars().last().map(|c| c == '"').filter(|&b| b).is_some();

    begins_with_quote && ends_with_quote
}

impl TokenStream for NaiveTokenStream {
    fn advance(&mut self) -> Option<Token> {
        let tok = self.peek();
        if tok.is_some() {
            self.curr += 1;
            tok
        } else {
            Option::None
        }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.curr).map(|t| t.clone())
    }

}

impl NaiveTokenStream {
    fn new(tokens: Vec<String>) -> NaiveTokenStream {
        NaiveTokenStream{
            curr: 0,
            tokens: tokens.iter().map(|s| Token::from_string(s)).collect()
        }
    }
}

pub fn naive_tokenize(s: &str) -> NaiveTokenStream {
    NaiveTokenStream::new(s.replace('(', " ( ")
     .replace(')', " ) ")
     .split_whitespace()
     .map(|st| st.to_owned()).collect())
}
