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
use eval::eval_program;
use value::Value;

fn default_env() -> environment::Environment {
    let mut env = environment::Environment::new();
    env.insert("<", Value::Function(builtins::less_than));
    env.insert("write", Value::Function(builtins::write_impl));
    env.insert("if", Value::LazyFunction(builtins::if_impl));
    env.insert("let", Value::LazyFunction(builtins::let_impl));
    env.insert("lambda", Value::LazyFunction(builtins::lambda));
    env.insert("+", Value::Function(builtins::plus));
    env.insert("-", Value::Function(builtins::minus));
    env.insert("str", Value::Function(builtins::to_str));
    env.insert("list", Value::LazyFunction(builtins::list));
    env
}

fn main() -> Result<(), error::InterpreterError> {
    let contents = fs::read_to_string(env::args().nth(1).unwrap())?;

    let mut tokens = lex(&contents)?;
    let ast = parse(&mut tokens)?;
    eval_program(&mut default_env(), &ast)?;
    Ok(())
}
