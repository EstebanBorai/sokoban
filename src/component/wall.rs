use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct Wall {}

impl Component for Wall {
    type Storage = VecStorage<Self>;
}
