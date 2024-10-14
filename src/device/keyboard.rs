use std::fmt::Debug;

use crate::core::handler::InputHandler;
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

pub trait KeyboardHandler {
    /// Press a key down, primitive action.
    fn press(&mut self, key: KeyboardKey) -> InputResult;

    /// Release a key up, primitive action.
    fn release(&mut self, key: KeyboardKey) -> InputResult;

    /// Tap a key.
    fn tap(&mut self, key: KeyboardKey) -> InputResult {
        self.press(key)?;
        self.release(key)?;
        Ok(())
    }

    /// Hold a key for a duration, then release it.
    fn hold(&mut self, key: KeyboardKey, duration: std::time::Duration) -> InputResult {
        self.press(key)?;
        std::thread::sleep(duration);
        self.release(key)?;
        Ok(())
    }

    /// Type text.
    fn text(&mut self, text: &str, duration: std::time::Duration) -> InputResult;
}

// implement the KeyboardHandler trait for InputHandler
impl<B: KeyboardHandler> KeyboardHandler for InputHandler<B> {
    fn press(&mut self, key: KeyboardKey) -> InputResult {
        self.backend.press(key)
    }
    fn release(&mut self, key: KeyboardKey) -> InputResult {
        self.backend.release(key)
    }

    fn tap(&mut self, key: KeyboardKey) -> InputResult {
        self.backend.tap(key)
    }

    fn hold(&mut self, key: KeyboardKey, duration: std::time::Duration) -> InputResult {
        self.backend.hold(key, duration)
    }

    fn text(&mut self, text: &str, duration: std::time::Duration) -> InputResult {
        self.backend.text(text, duration)
    }
}
