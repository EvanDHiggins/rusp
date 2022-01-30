use std::collections::HashMap;
use crate::value::Value;
use crate::parser::ASTNode;

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
pub struct Environment {
    value_map: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment{
            value_map: HashMap::new()
        }
    }

    #[allow(clippy::borrowed_box)]
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.value_map.get(name)
    }

    pub fn insert(&mut self, name: &str, value: Value) {
        self.value_map.insert(String::from(name), value);
    }

    pub fn extend(&self, name: &str, v: Value) -> Environment {
        let mut new_env = self.clone();
        new_env.insert(name, v);
        new_env
    }
}
