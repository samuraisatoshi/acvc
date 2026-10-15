use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub context: Vec<(String, String)>,
}

impl Error {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            context: Vec::new(),
        }
    }

    pub fn ctx(mut self, key: impl Into<String>, value: impl fmt::Display) -> Self {
        self.context.push((key.into(), value.to_string()));
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        for (key, value) in &self.context {
            write!(f, " {key}={value}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
