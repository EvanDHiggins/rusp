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
        let mut dbs = f.debug_struct("value::Value");
        match self {
            Value::Int(i) => dbs.field("i64", i),
            Value::Boolean(b) => dbs.field("bool", b),
            Value::Str(s) => dbs.field("String", s),
            Value::Function(_) => dbs.field("Function", &"<No Name>"),
            Value::LazyFunction(_) => dbs.field("LazyFunction", &"<No Name>"),
            Value::Unit => dbs.field("Unit", &"")
        }.finish()
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
        matches!(self, Value::Function(_) | Value::LazyFunction(_))
    }
}
