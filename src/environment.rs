use std::collections::HashMap;
use crate::value::Value;
use crate::ast::ASTNode;

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

pub struct Environment {
    value_map: HashMap<String, Value>,
    lazy_evaluation_map: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Environment {
        Environment{
            value_map: HashMap::new(),
            lazy_evaluation_map: HashMap::new()
        }
    }

    #[allow(clippy::borrowed_box)]
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.value_map.get(name)
    }

    pub fn insert(&mut self, name: &str, func: Value) {
        self.value_map.insert(String::from(name), func);
    }

    #[allow(clippy::borrowed_box)]
    pub fn get_lazy_evaluated(
        &self, name: &str) -> Option<&Value> {
        self.lazy_evaluation_map.get(name)
    }

    pub fn insert_lazy_evaluated(
        &mut self, name: &str, func: Value) {
        self.lazy_evaluation_map.insert(String::from(name), func);
    }
}
