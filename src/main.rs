use std::env;
use std::fs;

mod parser;
mod eval;
mod value;
mod error;
mod lexer;

use lexer::lex;
use parser::parse;
use eval::eval_program;

fn main() -> Result<(), error::InterpreterError> {
    let contents = fs::read_to_string(env::args().nth(1).unwrap())?;

    let mut tokens = lex(&contents);
    let ast = parse(&mut *tokens)?;
    eval_program(&mut eval::default_env(), &ast)?;
    Ok(())
}
