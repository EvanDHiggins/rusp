use crate::eval::environment::Environment;
use crate::eval::error::RuntimeError;
use crate::lexer::Token;
use crate::parser::ASTNode;
use std::rc::Rc;

// Trait that defines a "normal" function call. Arguments to the function are
// evaluated prior to invoking the actual function. Think things like '<',
// 'write', etc.
pub trait Callable {
    fn invoke(&self, env: &Environment, args: &[Value]) -> Result<Value, RuntimeError>;
}

// Trait that defines a function call which is passed an unevaluated AST
// instead of a list of values. The function is then free to evaluate all or
// none of the arguments. Think 'if' and 'lambda'.
pub trait LazyEvaluationCallable {
    fn invoke(&self, env: &Environment, args: &[ASTNode]) -> Result<Value, String>;
}

pub trait LazyEvaluationCallableWithMutableEnv {
    fn invoke(&self, env: &mut Environment, args: &[ASTNode]) -> Result<Value, String>;
}

#[derive(Clone)]
pub enum Value {
    Int(i64),
    Boolean(bool),
    Str(String),
    Function(fn(&Environment, &[Value]) -> Result<Value, RuntimeError>),
    LazyFunction(fn(&Environment, &[ASTNode]) -> Result<Value, RuntimeError>),
    EnvMutatingFunction(fn(&mut Environment, &[ASTNode]) -> Result<Value, RuntimeError>),
    Closure(Rc<dyn Callable>),
    List(Vec<Value>),
    Unit,
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut dbs = f.debug_struct("value::Value");
        match self {
            Value::Int(i) => dbs.field("i64", i),
            Value::Boolean(b) => dbs.field("bool", b),
            Value::Str(s) => dbs.field("String", s),
            Value::List(lst) => dbs.field("list", lst),
            Value::Function(_) => dbs.field("Function", &"<No Name>"),
            Value::Closure(_) => dbs.field("Closure", &"<No Name>"),
            Value::LazyFunction(_) => dbs.field("LazyFunction", &"<No Name>"),
            Value::EnvMutatingFunction(_) => dbs.field("EnvMutatingFunction", &"<No Name>"),
            Value::Unit => dbs.field("Unit", &""),
        }
        .finish()
    }
}

fn list_to_str(lst: &[Value]) -> Result<String, String> {
    let mut strs = Vec::new();
    for v in lst {
        strs.push(v.runtime_to_str()?);
    }
    Ok(format!("({})", strs.join(" ")))
}

impl Value {
    pub fn parse(token: &Token) -> Result<Value, RuntimeError> {
        match token {
            Token::IntLiteral(v) => Ok(Value::Int(v.to_owned())),
            Token::StringLiteral(s) => Ok(Value::Str(s.to_owned())),
            _ => RuntimeError::new(&format!("Could not convert token {:?} to Value.", token)),
        }
    }

    pub fn is_callable(&self) -> bool {
        matches!(
            self,
            Value::Closure(_)
                | Value::Function(_)
                | Value::LazyFunction(_)
                | Value::EnvMutatingFunction(_)
        )
    }

    // Converts a value to a string representation. This is expected to be
    // called at runtime, so certain types of values aren't expected to be
    // convertible to a string.
    pub fn runtime_to_str(&self) -> Result<String, String> {
        match self {
            Value::Int(i) => Ok(i.to_string()),
            Value::Boolean(b) => Ok(b.to_string()),
            Value::Str(s) => Ok(s.to_string()),
            Value::List(lst) => Ok(list_to_str(lst)?),
            _ => Err("".to_string()),
        }
    }
}
