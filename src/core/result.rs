use std::fmt::{Debug, Display};

use pyo3::{
    exceptions::{PyException, PyValueError},
    pyclass, pymethods, PyErr,
};

pub type InputResult = Result<(), InputError>;

pub struct Interrupt;

impl Debug for Interrupt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User interrupted operation.")
    }
}

impl Display for Interrupt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User interrupted operation.")
    }
}

#[pyclass(extends=PyException)]
struct ActionInterrupt {}

impl Debug for ActionInterrupt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Action interrupted by the user.")
    }
}

impl Display for ActionInterrupt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Action interrupted by the user.")
    }
}

#[pymethods]
impl ActionInterrupt {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    fn __str__(&self) -> &str {
        "Action interrupted by the user."
    }
}

pub enum InputError {
    Interrupt,
    Error(String),
}

impl InputError {
    pub fn error(error: String) -> Self {
        InputError::Error(error)
    }

    pub fn interrupt() -> Self {
        InputError::Interrupt
    }

    pub fn into_py(&self) -> PyErr {
        match self {
            InputError::Interrupt => PyErr::new::<ActionInterrupt, _>(()),
            InputError::Error(error) => PyValueError::new_err(error.to_string()),
        }
    }
}

impl Debug for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::Interrupt => write!(f, "Action interrupted by the user."),
            InputError::Error(error) => write!(f, "{}", error),
        }
    }
}

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::Interrupt => write!(f, "Action interrupted by the user."),
            InputError::Error(error) => write!(f, "{}", error),
        }
    }
}
