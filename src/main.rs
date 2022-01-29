use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string(env::args().nth(1).unwrap())?;

    let mut tokens = tokenize(&contents);
    let ast = parse(&mut tokens).unwrap();
    eval(ast);
    println!("{:?}", ast);

    Ok(())
}

struct TokenStream {
    curr: usize,
    tokens: Vec<String>
}

impl TokenStream {
    fn advance(&mut self) -> String {
        let tok = self.tokens[self.curr].to_string();
        self.curr += 1;
        tok
    }

    fn peek(&self) -> String {
        self.tokens[self.curr].to_string()
    }

    fn new(tokens: Vec<String>) -> TokenStream {
        TokenStream{curr: 0, tokens: tokens}
    }
}


fn tokenize(s: &str) -> TokenStream {
    TokenStream::new(s.replace('(', " ( ")
     .replace(')', " ) ")
     .split_whitespace()
     .map(|st| st.to_owned()).collect())
}

#[derive(Debug)]
enum ASTNode {
    Terminal { value: String },
    Expression { children: Vec<ASTNode> },
}

fn parse(tokens: &mut TokenStream) -> Result<ASTNode, &'static str> {
    let next_tok = tokens.advance();
    if next_tok == "(" {
        let mut nodes = Vec::new();
        while tokens.peek() != ")" {
            nodes.push(parse(tokens).unwrap());
        }
        tokens.advance(); // Strip off trailing ')'
        Ok(ASTNode::Expression{children: nodes})
    } else {
        Ok(ASTNode::Terminal{value: next_tok})
    }
}
