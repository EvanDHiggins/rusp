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
    env.insert(Value::make_id("<"), Box::new(callables::LessThan{}));
    env.insert_lazy_evaluated(Value::make_id("if"), Box::new(callables::If{}));
    env.insert(Value::make_id("write"), Box::new(callables::Write{}));
    eval(&env, &ast);
    Ok(())
}
