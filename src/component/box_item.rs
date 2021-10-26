use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct BoxItem {}

impl Component for BoxItem {
    type Storage = VecStorage<Self>;
}
