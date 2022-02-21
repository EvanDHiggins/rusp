#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new_err(message: &str) -> RuntimeError {
        RuntimeError {
            message: message.to_owned(),
        }
    }

    pub fn new<T>(message: &str) -> Result<T, RuntimeError> {
        Err(RuntimeError::new_err(message))
    }
}

impl From<String> for RuntimeError {
    fn from(s: String) -> Self {
        RuntimeError::new_err(&s)
    }
}
