use specs::{Component, NullStorage};

#[derive(Debug, Default, Clone)]
pub struct Movable {}

impl Component for Movable {
    type Storage = NullStorage<Self>;
}
