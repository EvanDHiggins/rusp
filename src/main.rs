use std::env;
use std::fs;

mod error;
mod eval;
mod lexer;
mod parser;
mod value;

use eval::eval_program;
use lexer::lex;
use parser::parse;

fn main() -> Result<(), error::InterpreterError> {
    let contents = fs::read_to_string(env::args().nth(1).unwrap())?;

    let mut tokens = lex(&contents);
    let ast = parse(&mut *tokens)?;
    eval_program(&mut eval::default_env(), &ast)?;
    Ok(())
}
