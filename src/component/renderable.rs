use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct Renderable {
    pub path: String,
}

impl Component for Renderable {
    type Storage = VecStorage<Self>;
}
