use crate::error::InterpreterError;
use crate::eval::default_env;
use crate::eval::environment::Context;
use crate::eval::eval_program;
use crate::eval::value::Value;
use crate::lexer::lex;
use crate::parser::parse;

pub struct RuspInterpreter {}

impl RuspInterpreter {
    pub fn new() -> Self {
        RuspInterpreter {}
    }

    pub fn run(&self, input: &str) -> Result<Value, InterpreterError> {
        let mut tokens = lex(input);
        let ast = parse(&mut *tokens)?;

        // I'm not sure why this is needed. We have a proper From defined for
        // this conversion, but something is preventing this file from seeing it.
        eval_program(&mut default_env(), &Context::new(), &ast)
            .map_err(|re| InterpreterError::new("RuntimeError", &re.message))
    }
}
