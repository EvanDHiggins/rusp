#[derive(Debug)]
pub struct TokenStreamOld {
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

impl TokenStreamOld {
    pub fn advance(&mut self) -> Option<Token> {
        let tok = self.peek();
        if tok.is_some() {
            self.curr += 1;
            tok
        } else {
            Option::None
        }
    }

    pub fn peek(&self) -> Option<Token> {
        self.tokens.get(self.curr).map(|t| t.clone())
    }

    fn new(tokens: Vec<String>) -> TokenStreamOld {
        TokenStreamOld{
            curr: 0,
            tokens: tokens.iter().map(|s| Token::from_string(s)).collect()
        }
    }
}

pub fn tokenize(s: &str) -> TokenStreamOld {
    TokenStreamOld::new(s.replace('(', " ( ")
     .replace(')', " ) ")
     .split_whitespace()
     .map(|st| st.to_owned()).collect())
}
