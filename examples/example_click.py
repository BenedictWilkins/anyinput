from anyinput import Session
import time

session = Session()
mouse = session.get_device("mouse")
mouse.hold("left", 5)

# use anyinput::{
#     backends::enigo_backend::EnigoBackend,
#     core::{handler::InputHandler, KeyboardHandler, MouseHandler},
#     device::{keyboard, mouse},
# };

# fn main() {
#     let mut handler = InputHandler::new(EnigoBackend::default());

#     handler
#         .click(mouse::MouseButton::Left)
#         .expect("Failed to click left");

#     handler
#         .tap(keyboard::KeyboardKey::Unicode('a'))
#         .expect("Failed to tap a");
# }
