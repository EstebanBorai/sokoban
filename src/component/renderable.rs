use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct Renderable {
    pub path: String,
}

impl Renderable {
    pub fn new(path: &str) -> Self {
        Renderable {
            path: String::from(path),
        }
    }
}

impl Component for Renderable {
    type Storage = VecStorage<Self>;
}
