use std::sync::Arc;

use pyo3::pyclass;

pub trait TouchHandler {}

#[pyclass]
pub struct TouchDevice {
    backend: Arc<dyn TouchHandler + Send + Sync>,
}

impl TouchDevice {
    pub fn new(backend: Arc<dyn TouchHandler + Send + Sync>) -> Self {
        Self { backend }
    }
}

impl Clone for TouchDevice {
    fn clone(&self) -> Self {
        Self {
            backend: Arc::clone(&self.backend),
        }
    }
}

impl TouchHandler for TouchDevice {}
