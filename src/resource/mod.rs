mod gameplay;
mod input_queue;

pub use gameplay::*;
pub use input_queue::*;

use specs::World;

pub fn register_resources(world: &mut World) {
    world.insert(Gameplay::default());
    world.insert(InputQueue::default());
}
