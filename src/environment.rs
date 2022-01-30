use std::collections::HashMap;
use crate::value::Value;
use crate::ast::ASTNode;

pub trait LazyEvaluationCallable {
    fn invoke(
        &self, env: &Environment, args: &[ASTNode]
    ) -> Result<Value, String>;
}

pub trait Callable {
    fn invoke(
        &self, env: &Environment, args: &[Value]
    ) -> Result<Value, String>;
}

pub struct Environment {
    value_map: HashMap<Value, Box<dyn Callable>>,
    lazy_evaluation_map: HashMap<Value, Box<dyn LazyEvaluationCallable>>
}

impl Environment {
    pub fn new() -> Environment {
        Environment{
            value_map: HashMap::new(),
            lazy_evaluation_map: HashMap::new()
        }
    }

    #[allow(clippy::borrowed_box)]
    pub fn get(&self, name: &Value) -> Option<&Box<dyn Callable>> {
        self.value_map.get(name)
    }

    pub fn insert(&mut self, name: Value, func: Box<dyn Callable>) {
        self.value_map.insert(name, func);
    }

    #[allow(clippy::borrowed_box)]
    pub fn get_lazy_evaluated(
        &self, name: &Value) -> Option<&Box<dyn LazyEvaluationCallable>> {
        self.lazy_evaluation_map.get(name)
    }

    pub fn insert_lazy_evaluated(
        &mut self, name: Value, func: Box<dyn LazyEvaluationCallable>) {
        self.lazy_evaluation_map.insert(name, func);
    }
}
