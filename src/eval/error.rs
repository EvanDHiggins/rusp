#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(message: &str) -> RuntimeError {
        RuntimeError {
            message: message.to_owned(),
        }
    }
}

impl From<String> for RuntimeError {
    fn from(s: String) -> Self {
        RuntimeError::new(&s)
    }
}
