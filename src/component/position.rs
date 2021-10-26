use specs::{Component, VecStorage};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl Position {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        Position { x, y, z }
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
