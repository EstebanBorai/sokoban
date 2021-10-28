use ggez::event::KeyCode;
use specs::{Join, ReadStorage, System, Write, WriteStorage};

use crate::component::{Player, Position};
use crate::resource::InputQueue;

pub struct InputSystem;

impl InputSystem {
    pub fn new() -> Self {
        InputSystem
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut positions, players) = data;

        for (position, _player) in (&mut positions, &players).join() {
            if let Some(key) = input_queue.keys_pressed.pop() {
                match key {
                    KeyCode::Up | KeyCode::W => position.y -= 1,
                    KeyCode::Right | KeyCode::D => position.x += 1,
                    KeyCode::Down | KeyCode::S => position.y += 1,
                    KeyCode::Left | KeyCode::A => position.x -= 1,
                    _ => (),
                }
            }
        }
    }
}
