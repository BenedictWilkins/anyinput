use std::fmt::Debug;

use crate::core::handler::InputHandler;
use crate::core::result::InputResult;

/// The mouse buttons, more buttons may be added in the future.
#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle mouse button.
    Middle,
}

pub trait MouseHandler {
    /// Press a mouse button down, primitive action.
    fn press(&mut self, button: MouseButton) -> InputResult;

    /// Release a mouse button up, primitive action.
    fn release(&mut self, button: MouseButton) -> InputResult;

    /// Click a mouse button.
    fn click(&mut self, button: MouseButton) -> InputResult {
        self.press(button)?;
        self.release(button)?;
        Ok(())
    }

    /// Hold a mouse button for a duration, then release it.
    fn hold(&mut self, button: MouseButton, duration: std::time::Duration) -> InputResult {
        self.press(button)?;
        std::thread::sleep(duration);
        self.release(button)?;
        Ok(())
    }

    /// Drag a mouse button to a relative position.
    fn drag(
        &mut self,
        button: MouseButton,
        dx: i32,
        dy: i32,
        duration: std::time::Duration,
    ) -> InputResult {
        self.press(button)?;
        self.move_rel(dx, dy, duration)?;
        self.release(button)?;
        Ok(())
    }

    /// Move the mouse to an absolute position.
    fn move_abs(&mut self, x: i32, y: i32, duration: std::time::Duration) -> InputResult;

    /// Move the mouse by a relative amount.
    fn move_rel(&mut self, dx: i32, dy: i32, duration: std::time::Duration) -> InputResult;
}

// Implement the MouseHandler trait for InputHandler
impl<B: MouseHandler> MouseHandler for InputHandler<B> {
    fn press(&mut self, button: MouseButton) -> InputResult {
        self.backend.press(button)
    }

    fn release(&mut self, button: MouseButton) -> InputResult {
        self.backend.release(button)
    }

    fn click(&mut self, button: MouseButton) -> InputResult {
        self.backend.click(button)
    }

    fn hold(&mut self, button: MouseButton, duration: std::time::Duration) -> InputResult {
        self.backend.hold(button, duration)
    }

    fn drag(
        &mut self,
        button: MouseButton,
        dx: i32,
        dy: i32,
        duration: std::time::Duration,
    ) -> InputResult {
        self.backend.drag(button, dx, dy, duration)
    }

    fn move_abs(&mut self, x: i32, y: i32, duration: std::time::Duration) -> InputResult {
        self.backend.move_abs(x, y, duration)
    }

    fn move_rel(&mut self, dx: i32, dy: i32, duration: std::time::Duration) -> InputResult {
        self.backend.move_rel(dx, dy, duration)
    }
}
