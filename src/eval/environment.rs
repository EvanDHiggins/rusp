use super::value::Value;
use std::collections::HashMap;

pub struct Context {}

impl Context {
    pub fn new() -> Context {
        Context {}
    }
}

#[derive(Clone)]
pub struct Environment {
    value_map: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            value_map: HashMap::new(),
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
