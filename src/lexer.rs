use crate::error::InterpreterError;

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
    fn peek(&self) -> Result<Option<Token>, TokenError>;
}

pub fn lex(s: &str) -> Result<LazyTokenStream, TokenError> {
    LazyTokenStream::new(s)
}

pub struct LazyTokenStream {
    input: String,
    curr: usize,
    next_token: Option<Token>,
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
        TokenError{message: message}
    }
}

impl TokenStream for LazyTokenStream {
    fn advance(&mut self) -> Result<Option<Token>, TokenError> {
        if self.next_token.is_some() {
            let next = self.next_token.clone();
            self.next_token = self.consume_token_from_input()?;
            Ok(next.clone())
        } else {
            Ok(Option::None)
        }
    }

    fn peek(&self) -> Result<Option<Token>, TokenError> {
        Ok(self.next_token.clone())
    }
}

fn is_identifier_char(c: char) -> bool {
    !c.is_whitespace() && !c.is_ascii_digit() && c != '(' && c != ')'
}

impl LazyTokenStream {
    pub fn new(input: &str) -> Result<LazyTokenStream, TokenError> {
        LazyTokenStream{
            input: String::from(input),
            curr: 0,
            next_token: Option::None
        }.init()
    }

    fn consume_token_from_input(&mut self) 
        -> Result<Option<Token>, TokenError> {
        self.consume_whitespace();
        if self.curr >= self.input.len() {
            return Ok(Option::None);
        }
        let curr_char = self.curr_char();
        self.curr += 1;
        if curr_char == '(' {
            Ok(Some(Token::OpenParen))
        } else if curr_char == ')' {
            Ok(Some(Token::CloseParen))
        } else if curr_char == '"' {
            self.consume_string().map(Some)
        } else if curr_char.is_ascii_digit() {
            self.curr -= 1; // backtrack to include this digit.
            self.consume_integer().map(Some)
        } else {
            self.curr -= 1;
            self.consume_identifier().map(Some)
        }

    }

    fn consume_identifier(&mut self) -> Result<Token, TokenError> {
        let start = self.curr;
        let first_non_id_idx =
            self.input.as_bytes()[self.curr..]
                .into_iter().position(|c| !is_identifier_char(*c as char));
        if first_non_id_idx.is_none() {
            return Err(TokenError::new(
                format!(
                    "Could not find end of identifier starting \
                     at index = {}", start)));
        };
        let end_idx = start + first_non_id_idx.unwrap();
        self.curr += first_non_id_idx.unwrap();

        let identifier =
            std::str::from_utf8(&self.input.as_bytes()[start..end_idx])?;
        Ok(Token::Id(String::from(identifier)))
    }

    fn stream_active(&self) -> bool {
        self.curr < self.input.as_bytes().len()
    }

    fn consume_whitespace(&mut self){
        while self.stream_active() && self.curr_char().is_whitespace() {
            self.curr += 1;
        }
    }

    fn curr_char(&self) -> char {
        self.input.as_bytes()[self.curr] as char
    }

    fn consume_string(&mut self) -> Result<Token, TokenError> {
        let next_quote_idx =
            self.input.as_bytes()[self.curr..]
                .into_iter().position(|c| (*c as char) == '"');
        if next_quote_idx.is_none() {
            return Err(
                TokenError::new(format!(
                    "Could not find closing quote to match \
                     the quote found at index = {}", self.curr)));
        }
        let end_idx = self.curr + next_quote_idx.unwrap();
        let start = self.curr;
        self.curr = end_idx + 1;
        let literal: String =
            self.input.chars().skip(start).take(end_idx - start).collect();
        Ok(Token::StringLiteral(literal))
    }

    fn consume_integer(&mut self) -> Result<Token, TokenError> {
        let first_non_digit = self.input.as_bytes()[self.curr..]
            .into_iter().position(|c| !(*c as char).is_ascii_digit()).unwrap();

        let start = self.curr;
        let end_idx = self.curr + first_non_digit - 1;
        self.curr += first_non_digit;
        let literal =
            std::str::from_utf8(&self.input.as_bytes()[start..(end_idx + 1)])?;
        let int_literal = literal.parse::<i64>()?;
        Ok(Token::IntLiteral(int_literal))
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
