use crate::lexer::charstream::CharStream;
use crate::lexer::charstream::StaticCharStream;
use crate::lexer::Token;
use crate::lexer::TokenError;
use crate::lexer::TokenStream;

pub struct LazyTokenStream {
    char_stream: Box<dyn CharStream>,
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
    pub fn new_from_string(s: &str) -> LazyTokenStream {
        LazyTokenStream {
            char_stream: Box::new(StaticCharStream::new(s)),
            next_token: Option::None,
        }
    }

    fn consume_token_from_input(&mut self) -> Result<Option<Token>, TokenError> {
        self.consume_whitespace();
        if self.char_stream.peek().is_none() {
            return Ok(Option::None);
        }
        let curr_char = if let Some(c) = self.char_stream.peek() {
            Ok(c)
        } else {
            Err(TokenError::new("Reached end of input.".to_string()))
        }?;
        if curr_char == '(' {
            self.char_stream.advance();
            Ok(Some(Token::OpenParen))
        } else if curr_char == ')' {
            self.char_stream.advance();
            Ok(Some(Token::CloseParen))
        } else if curr_char == '"' {
            self.consume_string().map(Some)
        } else if curr_char.is_ascii_digit() {
            self.consume_integer().map(Some)
        } else {
            self.consume_identifier().map(Some)
        }
    }

    fn consume_whitespace(&mut self) {
        while self.char_stream.peek().map_or(false, |c| c.is_whitespace()) {
            self.char_stream.advance();
        }
    }
    fn consume_string(&mut self) -> Result<Token, TokenError> {
        self.char_stream.advance();
        let literal = self.consume_while(|c| c != '"')?;
        self.char_stream.advance();
        Ok(Token::StringLiteral(literal))
    }

    fn consume_identifier(&mut self) -> Result<Token, TokenError> {
        let identifier = self.consume_while(|c| is_identifier_char(c))?;
        Ok(Token::Id(identifier))
    }

    fn consume_integer(&mut self) -> Result<Token, TokenError> {
        let literal = self.consume_while(|c| c.is_ascii_digit())?.parse::<i64>()?;
        Ok(Token::IntLiteral(literal))
    }

    // Consumes characters, c, from the input until F(c) evaluates to false.
    fn consume_while<F>(&mut self, func: F) -> Result<String, TokenError>
    where
        F: Fn(char) -> bool,
    {
        let mut chars = Vec::new();
        while self.char_stream.peek().map_or(false, &func) {
            chars.push(self.char_stream.advance().unwrap());
        }
        Ok(chars.iter().collect::<String>())
    }
}
