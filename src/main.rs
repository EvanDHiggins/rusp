use std::env;
use std::fs;

mod tokenize;
use tokenize::tokenize;

mod ast;
use ast::parse;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string(env::args().nth(1).unwrap())?;

    let mut tokens = tokenize(&contents);
    let ast = parse(&mut tokens).unwrap();
    println!("{:?}", ast);

    Ok(())
}
