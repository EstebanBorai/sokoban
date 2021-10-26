use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct BoxSpot {}

impl Component for BoxSpot {
    type Storage = VecStorage<Self>;
}
