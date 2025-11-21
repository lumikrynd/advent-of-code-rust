use std::error::Error;

pub struct AocError {
    error_message: String,
}

impl AocError {
    pub fn new(message: &str) -> Self {
        AocError {
            error_message: message.to_string(),
        }
    }

    pub fn from_err(msg: &str, err: impl Error) -> Self {
        let msg = format!("{}\n{:?}", msg, err);
        AocError::new(&msg)
    }
}

impl Error for AocError {}

impl std::fmt::Debug for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl std::fmt::Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_message)
    }
}
