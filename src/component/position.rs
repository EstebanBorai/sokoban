use specs::{Component, VecStorage};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
