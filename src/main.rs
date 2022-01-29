use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string(env::args().nth(1).unwrap())?;

    let tokens = tokenize(&contents);
    println!("{:?}", tokens);

    eval(tokens)
}

fn tokenize(s: &str) -> Vec<String> {
    s.replace('(', " ( ")
     .replace(')', " ) ")
     .split_whitespace()
     .map(|st| st.to_owned()).collect()
}

fn eval(tokens: Vec<String>) -> std::io::Result<()> {
    Ok(())
}
