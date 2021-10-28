mod input_queue;

pub use input_queue::*;

use specs::World;

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
}
