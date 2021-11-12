use specs::{Component, NullStorage};

#[derive(Debug, Default, Clone)]
pub struct Immovable {}

impl Component for Immovable {
    type Storage = NullStorage<Self>;
}
