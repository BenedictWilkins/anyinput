use std::sync::Mutex;

use crate::core::result::{InputError, InputResult};
use crate::core::TouchHandler;
use crate::device::keyboard::KeyboardHandler;
use crate::device::mouse::MouseHandler;
use crate::device::{keyboard::KeyboardKey, mouse::MouseButton};
use enigo::{
    Coordinate, Direction, Enigo, InputError as EnigoInputError, Keyboard, Mouse, Settings,
};

use super::backend::Backend;

#[derive(Debug)]
pub struct EnigoBackend {
    enigo: Mutex<Enigo>,
}

impl Backend for EnigoBackend {}

impl Default for EnigoBackend {
    fn default() -> Self {
        Self::new(None)
    }
}

impl EnigoBackend {
    pub fn new(settings: Option<&Settings>) -> Self {
        Self {
            enigo: Mutex::new(
                Enigo::new(settings.unwrap_or(&Settings::default()))
                    .expect("Failed to create `enigo` backend."),
            ),
        }
    }

    fn map_key(key: KeyboardKey) -> enigo::Key {
        match key {
            KeyboardKey::Unicode(c) => enigo::Key::Unicode(c),
            KeyboardKey::Control => enigo::Key::Control,
            KeyboardKey::Shift => enigo::Key::Shift,
            KeyboardKey::Alt => enigo::Key::Alt,
            KeyboardKey::Enter => enigo::Key::Return,
            KeyboardKey::Escape => enigo::Key::Escape,
            KeyboardKey::Tab => enigo::Key::Tab,
            KeyboardKey::ArrowUp => enigo::Key::UpArrow,
            KeyboardKey::ArrowDown => enigo::Key::DownArrow,
            KeyboardKey::ArrowLeft => enigo::Key::LeftArrow,
            KeyboardKey::ArrowRight => enigo::Key::RightArrow,
            KeyboardKey::Home => enigo::Key::Home,
            KeyboardKey::End => enigo::Key::End,
            KeyboardKey::PageUp => enigo::Key::PageUp,
            KeyboardKey::PageDown => enigo::Key::PageDown,
        }
    }

    fn map_mouse_button(button: MouseButton) -> enigo::Button {
        match button {
            MouseButton::Left => enigo::Button::Left,
            MouseButton::Right => enigo::Button::Right,
            MouseButton::Middle => enigo::Button::Middle,
        }
    }
}

impl From<EnigoInputError> for InputError {
    fn from(err: EnigoInputError) -> Self {
        InputError::new(err.to_string())
    }
}

impl MouseHandler for EnigoBackend {
    fn press(&self, button: MouseButton) -> InputResult {
        let mut enigo = self.enigo.lock().unwrap();

        if let Err(e) = enigo.button(Self::map_mouse_button(button), Direction::Press) {
            return Err(e.into());
        }
        Ok(())
    }

    fn release(&self, button: MouseButton) -> InputResult {
        let mut enigo = self.enigo.lock().unwrap();
        if let Err(e) = enigo.button(Self::map_mouse_button(button), Direction::Release) {
            return Err(e.into());
        }
        Ok(())
    }

    fn click(&self, button: MouseButton) -> InputResult {
        let mut enigo = self.enigo.lock().unwrap();
        if let Err(e) = enigo.button(Self::map_mouse_button(button), Direction::Click) {
            return Err(e.into());
        }
        Ok(())
    }

    fn hold(&self, button: MouseButton, duration: std::time::Duration) -> InputResult {
        MouseHandler::press(self, button)?;
        std::thread::sleep(duration);
        MouseHandler::release(self, button)?;
        Ok(())
    }

    fn move_abs(&self, x: i32, y: i32, _duration: std::time::Duration) -> InputResult {
        let mut enigo = self.enigo.lock().unwrap();

        // TODO: Implement duration
        if let Err(e) = enigo.move_mouse(x, y, Coordinate::Abs) {
            return Err(e.into());
        }
        Ok(())
    }

    fn move_rel(&self, dx: i32, dy: i32, duration: std::time::Duration) -> InputResult {
        let mut enigo = self.enigo.lock().unwrap();
        // TODO: Implement duration
        if let Err(e) = enigo.move_mouse(dx, dy, Coordinate::Rel) {
            return Err(e.into());
        }
        Ok(())
    }
}

impl KeyboardHandler for EnigoBackend {
    fn press(&self, key: KeyboardKey) -> InputResult {
        let mut enigo = self.enigo.lock().unwrap();

        if let Err(e) = enigo.key(Self::map_key(key), Direction::Press) {
            return Err(e.into());
        }
        Ok(())
    }

    fn release(&self, key: KeyboardKey) -> InputResult {
        let mut enigo = self.enigo.lock().unwrap();

        if let Err(e) = enigo.key(Self::map_key(key), Direction::Release) {
            return Err(e.into());
        }
        Ok(())
    }

    fn tap(&self, key: KeyboardKey) -> InputResult {
        KeyboardHandler::press(self, key)?;
        KeyboardHandler::release(self, key)?;
        Ok(())
    }

    fn hold(&self, key: KeyboardKey, duration: std::time::Duration) -> InputResult {
        KeyboardHandler::press(self, key)?;
        std::thread::sleep(duration);
        KeyboardHandler::release(self, key)?;
        Ok(())
    }

    fn text(&self, text: &str, duration: std::time::Duration) -> InputResult {
        let mut enigo = self.enigo.lock().unwrap();
        if let Err(e) = enigo.text(text) {
            return Err(e.into());
        }
        Ok(())
    }
}

impl TouchHandler for EnigoBackend {}
