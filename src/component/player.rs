use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct Player {}

impl Component for Player {
    type Storage = VecStorage<Self>;
}
