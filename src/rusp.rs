use crate::error::InterpreterError;
use crate::eval::default_env;
use crate::eval::environment::Context;
use crate::eval::eval_program;
use crate::eval::io::IOStream;
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

        //let mut context = Context::new(IOStream::new_in_memory_buffer());
        let mut context = Context::new(IOStream::new_stdout());

        let ret = eval_program(&mut default_env(), &mut context, &ast)
            // I'm not sure why this is needed. We have a proper From defined for
            // this conversion, but something is preventing this file from seeing it.
            .map_err(|re| InterpreterError::new("RuntimeError", &re.message));

        let captured_output = if let IOStream::InMemoryBuffer(vec) = context.stdout {
            vec
        } else {
            Vec::new()
        };
        println!("{}", String::from_utf8(captured_output).unwrap());

        ret
    }
}
