//
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{Arc, Mutex},
    time::Duration,
};

use pyo3::{exceptions::PyValueError, pyclass, pymethods, PyResult};

use crate::{
    backends::backend::Backend,
    core::result::{InputResult, Interrupt},
};

/// The mouse buttons, more buttons may be added in the future.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle mouse button.
    Middle,
}

impl MouseButton {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "left" => Some(Self::Left),
            "right" => Some(Self::Right),
            "middle" => Some(Self::Middle),
            _ => None,
        }
    }
}

pub trait MouseHandler {
    /// Press a mouse button down, primitive action.
    fn press(&self, button: MouseButton) -> InputResult;

    /// Release a mouse button up, primitive action.
    fn release(&self, button: MouseButton) -> InputResult;

    /// Click a mouse button.
    fn click(&self, button: MouseButton) -> InputResult;

    /// Hold a mouse button for a duration, then release it.
    fn hold(&self, button: MouseButton, duration: std::time::Duration) -> InputResult;

    /// Drag a mouse button to a relative position.
    fn drag(
        &self,
        button: MouseButton,
        dx: i32,
        dy: i32,
        duration: std::time::Duration,
    ) -> InputResult;

    /// Move the mouse to an absolute position.
    fn move_abs(&self, x: i32, y: i32, duration: std::time::Duration) -> InputResult;

    /// Move the mouse by a relative amount.
    fn move_rel(&self, dx: i32, dy: i32, duration: std::time::Duration) -> InputResult;
}

#[pyclass]
pub struct MouseDevice {
    backend: Arc<dyn MouseHandler + Send + Sync>,
}

impl Clone for MouseDevice {
    fn clone(&self) -> Self {
        Self {
            backend: Arc::clone(&self.backend),
        }
    }
}

impl MouseDevice {
    pub fn new(backend: Arc<dyn MouseHandler + Send + Sync>) -> Self {
        Self { backend }
    }

    fn get_mouse_button(&self, button: &str) -> PyResult<MouseButton> {
        MouseButton::from_str(button).ok_or(PyValueError::new_err("Invalid mouse button"))
    }
}

#[pymethods]
impl MouseDevice {
    fn press(&self, button: &str) -> PyResult<()> {
        MouseHandler::press(self, self.get_mouse_button(button)?).map_err(|e| e.into_py())
    }

    fn release(&self, button: &str) -> PyResult<()> {
        MouseHandler::release(self, self.get_mouse_button(button)?).map_err(|e| e.into_py())
    }

    fn click(&self, button: &str) -> PyResult<()> {
        MouseHandler::click(self, self.get_mouse_button(button)?).map_err(|e| e.into_py())
    }

    fn hold(&self, button: &str, duration: f64) -> PyResult<()> {
        MouseHandler::hold(
            self,
            self.get_mouse_button(button)?,
            Duration::from_secs_f64(duration),
        )
        .map_err(|e| e.into_py())
    }

    fn drag(&self, button: &str, dx: i32, dy: i32, duration: f64) -> PyResult<()> {
        MouseHandler::drag(
            self,
            self.get_mouse_button(button)?,
            dx,
            dy,
            Duration::from_secs_f64(duration),
        )
        .map_err(|e| e.into_py())
    }

    fn move_abs(&self, x: i32, y: i32, duration: f64) -> PyResult<()> {
        MouseHandler::move_abs(self, x, y, Duration::from_secs_f64(duration))
            .map_err(|e| e.into_py())
    }

    fn move_rel(&self, dx: i32, dy: i32, duration: f64) -> PyResult<()> {
        MouseHandler::move_rel(self, dx, dy, Duration::from_secs_f64(duration))
            .map_err(|e| e.into_py())
    }
}

// Implement the MouseHandler trait for InputHandler
impl MouseHandler for MouseDevice {
    fn press(&self, button: MouseButton) -> InputResult {
        self.backend.press(button)
    }
    fn release(&self, button: MouseButton) -> InputResult {
        self.backend.release(button)
    }

    fn click(&self, button: MouseButton) -> InputResult {
        self.backend.click(button)
    }

    fn hold(&self, button: MouseButton, duration: std::time::Duration) -> InputResult {
        self.backend.hold(button, duration)
    }

    fn drag(
        &self,
        button: MouseButton,
        dx: i32,
        dy: i32,
        duration: std::time::Duration,
    ) -> InputResult {
        self.backend.drag(button, dx, dy, duration)
    }

    fn move_abs(&self, x: i32, y: i32, duration: std::time::Duration) -> InputResult {
        self.backend.move_abs(x, y, duration)
    }

    fn move_rel(&self, dx: i32, dy: i32, duration: std::time::Duration) -> InputResult {
        self.backend.move_rel(dx, dy, duration)
    }
}
