use std::fmt::{Debug, Display};

pub type InputResult = Result<(), InputError>;

pub struct InputError {
    error: String,
}

impl Debug for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl InputError {
    pub fn new(error: String) -> Self {
        InputError { error }
    }
}

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}
