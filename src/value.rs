use crate::lexer::Token;
use crate::environment::{Callable, LazyEvaluationCallable};
use std::rc::Rc;

#[derive(Clone)]
pub enum Value {
    Int(i64),
    Boolean(bool),
    Str(String),
    Function(Rc<dyn Callable>),
    LazyFunction(Rc<dyn LazyEvaluationCallable>),
    Unit,
} 

impl std::fmt::Debug for Value {
    fn fmt(
        &self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl Value {
    pub fn parse(token: &Token) -> Result<Value, String> {
        match token {
            Token::IntLiteral(v) => Ok(Value::Int(v.to_owned())),
            Token::StringLiteral(s) => Ok(Value::Str(s.to_owned())),
            _ => Err(format!("Could not convert token {:?} to Value.", token))
        }
    }

    pub fn is_callable(&self) -> bool {
        match self {
            Value::Function(_) => true,
            Value::LazyFunction(_) => true,
            _ => false
        }
    }
}
