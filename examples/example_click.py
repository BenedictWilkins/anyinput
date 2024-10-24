from anyinput import Session
import time

session = Session()
mouse = session.get_device("mouse")
for i in range(100):
    time.sleep(0.1)
    mouse.move_rel(2, 2, 0)
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
