use std::env;
use std::fs;

mod tokenize;
mod ast;
mod eval;
mod environment;
mod value;
mod callables;


use tokenize::tokenize;
use ast::parse;
use eval::eval;
use value::Value;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string(env::args().nth(1).unwrap())?;

    let mut tokens = tokenize(&contents);
    let ast = parse(&mut tokens).unwrap();
    let mut env = environment::Environment::new();
    env.add(Value::new("<"), Box::new(callables::LessThan{}));
    env.add(Value::new("if"), Box::new(callables::If{}));
    env.add(Value::new("write"), Box::new(callables::Write{}));
    eval(&env, &ast);
    Ok(())
}
