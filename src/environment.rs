use std::collections::HashMap;
use crate::value::Value;

pub trait Callable {
    fn invoke(&self, args: &[Value]) -> Result<Value, &'static str>;
}

pub struct Environment {
    value_map: HashMap<Value, Box<dyn Callable>>
}

impl Environment {
    pub fn new() -> Environment {
        Environment{value_map: HashMap::new()}
    }

    pub fn get(&self, name: Value) -> &Box<dyn Callable> {
        self.value_map.get(&name).unwrap()
    }

    pub fn add(&mut self, name: Value, func: Box<dyn Callable>) {
        self.value_map.insert(name, func);
    }
}
