pub struct TokenStream {
    curr: usize,
    tokens: Vec<String>
}

impl TokenStream {
    pub fn advance(&mut self) -> String {
        let tok = self.tokens[self.curr].to_string();
        self.curr += 1;
        tok
    }

    pub fn peek(&self) -> String {
        self.tokens[self.curr].to_string()
    }

    pub fn new(tokens: Vec<String>) -> TokenStream {
        TokenStream{curr: 0, tokens: tokens}
    }
}


pub fn tokenize(s: &str) -> TokenStream {
    TokenStream::new(s.replace('(', " ( ")
     .replace(')', " ) ")
     .split_whitespace()
     .map(|st| st.to_owned()).collect())
}
