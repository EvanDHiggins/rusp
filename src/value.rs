#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Value {
    value: String,
}

impl Value {
    pub fn new(v: &str) -> Value {
        Value{value: v.to_owned()}
    }

    pub fn to_int(&self) -> Result<i64, &'static str> {
        Ok(self.value.parse::<i64>().unwrap())
    }

    pub fn to_string(&self) -> String {
        self.value.to_owned()
    }

    pub fn to_bool(&self) -> Result<bool, String> {
        match self.value.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(format!("Could not parse Bool: {}", self.value))
        }
    }
}
