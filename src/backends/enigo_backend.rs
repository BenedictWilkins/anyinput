use std::sync::atomic::AtomicBool;
use std::sync::Mutex;
use std::time::Duration;

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
        let enigo = Enigo::new(settings.unwrap_or(&Settings::default()))
            .expect("Failed to create `enigo` backend.");
        let starting_position = enigo
            .location()
            .expect("Failed to get initial mouse location.");
        Self {
            enigo: Mutex::new(enigo),
        }
    }

    fn get_mouse_position(&self) -> (i32, i32) {
        self.enigo
            .lock()
            .unwrap()
            .location()
            .expect("failed to get mouse position.")
    }

    // fn sleep(&self, duration: std::time::Duration) -> InputResult {
    //     let start_time = std::time::Instant::now();
    //     // sleep periodically and check that the mouse state has not changed
    //     // this is how you interrupt automated actions (that have duration), its a fail safe, for now!
    //     let start_position = self.get_mouse_position();
    //     while std::time::Instant::now().duration_since(start_time) < duration {
    //         std::thread::sleep(Duration::from_millis(5));
    //         if start_position == self.get_mouse_position() {
    //             return Err(InputError::interrupt());
    //         }
    //     }
    //     return Ok(());
    // }

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
        InputError::error(err.to_string())
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
        fn enigo_press(enigo: &mut Enigo, button: enigo::Button) -> InputResult {
            enigo
                .button(button, Direction::Press)
                .or_else(|e| Err::<(), InputError>(e.into()))?;
            Ok(())
        }

        fn enigo_release(enigo: &mut Enigo, button: enigo::Button) -> InputResult {
            enigo
                .button(button, Direction::Release)
                .or_else(|e| Err::<(), InputError>(e.into()))?;
            Ok(())
        }

        fn enigo_position(enigo: &Enigo) -> Result<(i32, i32), InputError> {
            enigo.location().or_else(|e| Err(e.into()))
        }

        //println!("Holding mouse button {:?}", button);

        let mut enigo = self.enigo.lock().expect("Failed to aquire enigo backend.");
        let button = Self::map_mouse_button(button);

        enigo_press(&mut enigo, button)?;

        // sleep for the duration, but check if the mouse has moved during, if so we assume that the user has interrupted the action
        let start_time = std::time::Instant::now();
        let start_position = enigo_position(&enigo).or_else(|e| {
            enigo_release(&mut enigo, button).or_else(|ee| Err(ee))?;
            return Err(e);
        })?;

        while let Some(diff) =
            duration.checked_sub(std::time::Instant::now().duration_since(start_time))
        {
            std::thread::sleep(Duration::min(diff, Duration::from_millis(5)));
            let new_position = enigo_position(&enigo).or_else(|e| {
                enigo_release(&mut enigo, button).or_else(|ee| Err(ee))?;
                return Err(e);
            })?;
            if start_position != new_position {
                // release the mouse button, the user interrupted the action with mouse movement
                enigo_release(&mut enigo, button)?;
                return Err(InputError::interrupt());
            }
        }
        // release the mouse button
        enigo_release(&mut enigo, button)?;
        //println!("Released mouse button {:?}", button);
        Ok(())
    }

    fn move_abs(&self, x: i32, y: i32, duration: std::time::Duration) -> InputResult {
        let mut enigo = self.enigo.lock().unwrap();
        // move the mouse

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

    fn drag(
        &self,
        button: MouseButton,
        dx: i32,
        dy: i32,
        duration: std::time::Duration,
    ) -> InputResult {
        todo!()
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
