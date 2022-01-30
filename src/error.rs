use std::fmt;

pub struct InterpreterError {
    kind: String,
    message: String,
}

impl InterpreterError {
    pub fn new(kind: &str, message: &str) -> InterpreterError {
        InterpreterError {
            kind: String::from(kind),
            message: String::from(message),
        }
    }
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Encountered {}.\nMessage: {}", self.kind, self.message)
    }
}

impl fmt::Debug for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Encountered {}.\nMessage: {}", self.kind, self.message)
    }
}

impl From<String> for InterpreterError {
    fn from(s: String) -> InterpreterError {
        InterpreterError{
            kind: "Generic Error".to_owned(),
            message: s
        }
    }
}

impl From<std::io::Error> for InterpreterError {
    fn from(err: std::io::Error) -> InterpreterError {
        InterpreterError{
            kind: String::from("io"),
            message: err.to_string()
        }
    }
}

