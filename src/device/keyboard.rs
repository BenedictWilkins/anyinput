use std::{fmt::Debug, sync::Arc};

use pyo3::pyclass;

use crate::core::result::InputResult;
#[derive(Debug, Clone, Copy)]
pub enum KeyboardKey {
    Unicode(char),
    Control,
    Shift,
    Alt,
    Enter,
    Escape,
    Tab,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    PageUp,
    PageDown,
    // TODO we might add others!
}

// defines the keyboard handler trait, this should be implemented by the backend!
pub trait KeyboardHandler {
    /// Press a key down, primitive action.
    fn press(&self, key: KeyboardKey) -> InputResult;

    /// Release a key up, primitive action.
    fn release(&self, key: KeyboardKey) -> InputResult;

    /// Tap a key.
    fn tap(&self, key: KeyboardKey) -> InputResult {
        self.press(key)?;
        self.release(key)?;
        Ok(())
    }

    /// Hold a key for a duration, then release it.
    fn hold(&self, key: KeyboardKey, duration: std::time::Duration) -> InputResult {
        self.press(key)?;
        std::thread::sleep(duration);
        self.release(key)?;
        Ok(())
    }

    /// Type text.
    fn text(&self, text: &str, duration: std::time::Duration) -> InputResult;
}

#[pyclass]
pub struct KeyboardDevice {
    backend: Arc<dyn KeyboardHandler + Send + Sync>,
}

impl KeyboardDevice {
    pub fn new(backend: Arc<dyn KeyboardHandler + Send + Sync>) -> Self {
        Self { backend }
    }
}

impl Clone for KeyboardDevice {
    fn clone(&self) -> Self {
        Self {
            backend: Arc::clone(&self.backend),
        }
    }
}

impl KeyboardHandler for KeyboardDevice {
    fn press(&self, key: KeyboardKey) -> InputResult {
        self.backend.press(key)
    }
    fn release(&self, key: KeyboardKey) -> InputResult {
        self.backend.release(key)
    }

    fn tap(&self, key: KeyboardKey) -> InputResult {
        self.backend.tap(key)
    }

    fn hold(&self, key: KeyboardKey, duration: std::time::Duration) -> InputResult {
        self.backend.hold(key, duration)
    }

    fn text(&self, text: &str, duration: std::time::Duration) -> InputResult {
        self.backend.text(text, duration)
    }
}
