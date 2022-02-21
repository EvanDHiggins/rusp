use crate::rusp::RuspInterpreter;
use std::env;
use std::fs;

mod error;
mod eval;
mod lexer;
mod parser;
mod rusp;

fn main() -> Result<(), error::InterpreterError> {
    let contents = fs::read_to_string(env::args().nth(1).unwrap())?;

    // Run interpreter and throw away return value.
    RuspInterpreter::new().run(&contents).map(|_| ())
}
