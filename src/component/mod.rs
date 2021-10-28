mod box_item;
mod box_spot;
mod player;
mod position;
mod renderable;
mod wall;

pub use box_item::*;
pub use box_spot::*;
pub use player::*;
pub use position::*;
pub use renderable::*;
pub use wall::*;

use specs::{World, WorldExt};

pub fn register_components(world: &mut World) {
    world.register::<BoxItem>();
    world.register::<BoxSpot>();
    world.register::<Player>();
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Wall>();
}
