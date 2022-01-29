use crate::tokenize::Token;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Value {
    Int(i64),
    Boolean(bool),
    Str(String),
    Id(String),
    Unit,
} 

impl Value {

    pub fn parse(token: &Token) -> Result<Value, String> {
        match token {
            Token::IntLiteral(v) => Ok(Value::Int(v.to_owned())),
            Token::StringLiteral(s) => Ok(Value::Str(s.to_owned())),
            Token::Id(i) => Ok(Value::make_id(i)),
            _ => Err(format!("Could not convert token {:?} to Value.", token))
        }
    }

    pub fn to_int(&self) -> Result<i64, String> {
        match self {
            Value::Int(value) => {
                Ok(value.to_owned())
            }
            _ => Err(format!("Failed to convert {:?} to Int.", self))
        }
    }

    pub fn to_string(&self) -> Result<String, String> {
        match self {
            Value::Str(value) => {
                Ok(value.to_owned())
            }
            _ => Err(format!("Failed to convert {:?} to Str.", self))
        }
    }

    pub fn to_bool(&self) -> Result<bool, String> {
        match self {
            Value::Boolean(b) => {
                Ok(b.to_owned())
            }
            _ => Err(format!("Could not parse Bool: {:?}", self))
        }
    }

    pub fn make_id(id: &str) -> Value {
        Value::Id(id.to_owned())
    }

    pub fn make_str(val: &str) -> Value {
        Value::Str(val.to_owned())
    }
}