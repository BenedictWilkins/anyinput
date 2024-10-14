// Define a struct that will act as the handler for backends, each device will implement this
pub struct InputHandler<B> {
    pub(crate) backend: B,
}

impl<B> InputHandler<B> {
    pub fn new(backend: B) -> Self {
        InputHandler { backend }
    }
}
