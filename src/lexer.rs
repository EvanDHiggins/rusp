use crate::error::InterpreterError;
use std::collections::VecDeque;

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

pub fn lex(s: &str) -> LazyTokenStream {
    LazyTokenStream::new(s)
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

struct InteractiveCharStream {
    buffer: VecDeque<char>,
}

impl InteractiveCharStream {
    fn new() -> InteractiveCharStream {
        InteractiveCharStream {
            buffer: VecDeque::new(),
        }
    }

    fn peek(&mut self) -> char {
        'c'
    }
}

trait CharStream {
    fn advance(&mut self) -> Option<char>;
    fn peek(&mut self) -> Option<char>;
}

struct StaticCharStream {
    buffer: Vec<char>,
    curr: usize,
}

impl StaticCharStream {
    fn new(input: &str) -> StaticCharStream {
        StaticCharStream{
            buffer: input.chars().collect(),
            curr: 0
        }
    }
}

impl CharStream for StaticCharStream {
    fn advance(&mut self) -> Option<char> {
        if self.curr >= self.buffer.len() {
            None
        } else {
            let c = self.buffer[self.curr];
            self.curr += 1;
            Some(c)
        }
    }

    fn peek(&mut self) -> Option<char> {
        if self.curr >= self.buffer.len() {
            None
        } else {
            Some(self.buffer[self.curr])
        }
    }
}

pub struct LazyTokenStream {
    input: Vec<char>,
    char_stream: Box<dyn CharStream>,
    curr: usize,
    next_token: Option<Token>,
}

impl TokenStream for LazyTokenStream {
    fn advance(&mut self) -> Result<Option<Token>, TokenError> {
        if self.next_token.is_some() {
            let next = self.next_token.clone();
            self.next_token = self.consume_token_from_input()?;
            Ok(next)
        } else {
            Ok(Option::None)
        }
    }

    fn peek(&mut self) -> Result<Option<Token>, TokenError> {
        if self.next_token.is_some() {
            Ok(self.next_token.clone())
        } else {
            self.next_token = self.consume_token_from_input()?;
            Ok(self.next_token.clone())
        }
    }
}

fn is_identifier_char(c: char) -> bool {
    !c.is_whitespace() && !c.is_ascii_digit() && c != '(' && c != ')'
}

impl LazyTokenStream {
    pub fn new(input: &str) -> LazyTokenStream {
        LazyTokenStream{
            input: input.chars().collect(),
            char_stream: Box::new(StaticCharStream::new(input)),
            curr: 0,
            next_token: Option::None
        }
    }

    fn consume_token_from_input(&mut self) 
        -> Result<Option<Token>, TokenError> {
        self.consume_whitespace();
        if self.curr >= self.input.len() {
            return Ok(Option::None);
        }
        let curr_char = self.curr_char();
        if curr_char == '(' {
            self.curr += 1;
            Ok(Some(Token::OpenParen))
        } else if curr_char == ')' {
            self.curr += 1;
            Ok(Some(Token::CloseParen))
        } else if curr_char == '"' {
            self.consume_string().map(Some)
        } else if curr_char.is_ascii_digit() {
            self.consume_integer().map(Some)
        } else {
            self.consume_identifier().map(Some)
        }

    }

    fn stream_active(&self) -> bool {
        self.curr < self.input.len()
    }

    fn consume_whitespace(&mut self){
        while self.stream_active() && self.curr_char().is_whitespace() {
            self.curr += 1;
        }
    }

    fn curr_char(&self) -> char {
        self.input[self.curr]
    }

    // Consumes characters, c, from the input until F(c) evaluates to false.
    // This will advance self.curr to be one over the last character returned
    // from the input.
    fn consume_while<F>(&mut self, desc: &str, func: F) -> Result<String, TokenError>
        where F: FnMut(&char) -> bool
    {
        let first_failure = self.input[self.curr..]
            .iter().position(func);
        match first_failure {
            Option::None =>
                Err(TokenError::new(format!(
                    "Ran out of input searching for {} at index {}.",
                    desc, self.curr))),
            Option::Some(idx) => {

                let start = self.curr;
                let end = self.curr + idx;
                self.curr = end;
                Ok(self.input[start..end].iter().collect::<String>())
            }
        }
    }

    fn consume_string(&mut self) -> Result<Token, TokenError> {
        self.curr += 1;
        let literal = self.consume_while("[String]", |c| *c == '"')?;
        self.curr += 1; // Remove the '"' that we know exists.
        Ok(Token::StringLiteral(literal))
    }

    fn consume_identifier(&mut self) -> Result<Token, TokenError> {
        let identifier = self.consume_while("[Identifier]", |c| {
            !is_identifier_char(*c)
        })?;
        Ok(Token::Id(identifier))
    }

    fn consume_integer(&mut self) -> Result<Token, TokenError> {
        let literal = self.consume_while("[String Literal]", |c| {
            !c.is_ascii_digit()
        })?.parse::<i64>()?;
        Ok(Token::IntLiteral(literal))
    }
}
