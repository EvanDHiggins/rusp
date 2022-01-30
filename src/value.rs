use crate::lexer::Token;
use crate::parser::ASTNode;
use crate::environment::Environment;
use std::rc::Rc;

// Trait that defines a "normal" function call. Arguments to the function are
// evaluated prior to invoking the actual function. Think things like '<',
// 'write', etc.
pub trait Callable {
    fn invoke(
        &self, env: &Environment, args: &[Value]
    ) -> Result<Value, String>;
}

// Trait that defines a function call which is passed an unevaluated AST
// instead of a list of values. The function is then free to evaluate all or
// none of the arguments. Think 'if' and 'lambda'.
pub trait LazyEvaluationCallable {
    fn invoke(
        &self, env: &Environment, args: &[ASTNode]
    ) -> Result<Value, String>;
}

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
