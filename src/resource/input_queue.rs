use ggez::event::KeyCode;

#[derive(Default)]
pub struct InputQueue {
    keys_pressed: Vec<KeyCode>,
}

impl InputQueue {
    pub fn push(&mut self, key: KeyCode) {
        self.keys_pressed.push(key);
    }

    pub fn pop(&mut self) -> Option<KeyCode> {
        self.keys_pressed.pop()
    }
}
