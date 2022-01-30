use std::env;
use std::fs;

mod lexer;
mod parser;
mod eval;
mod environment;
mod value;
mod builtins;
mod error;


use lexer::lex;
use parser::parse;
use eval::eval;
use value::Value;
use std::rc::Rc;
use builtins::{
    LessThan,
    Write,
    If,
    Let,
    Lambda
};

fn default_env() -> environment::Environment {
    let mut env = environment::Environment::new();
    env.insert("<", Value::Function(Rc::new(LessThan{})));
    env.insert("write", Value::Function(Rc::new(Write{})));
    env.insert("if", Value::LazyFunction(Rc::new(If{})));
    env.insert("let", Value::LazyFunction(Rc::new(Let{})));
    env.insert("lambda", Value::LazyFunction(Rc::new(Lambda{})));
    env
}

fn main() -> Result<(), error::InterpreterError> {
    let contents = fs::read_to_string(env::args().nth(1).unwrap())?;

    let mut tokens = lex(&contents)?;
    let ast = parse(&mut tokens)?;
    eval(&default_env(), &ast)?;
    Ok(())
}
