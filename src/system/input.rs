use ggez::event::KeyCode;
use specs::world::Index;
use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};
use std::collections::HashMap;

use crate::component::{Immovable, Movable, Player, Position};
use crate::resource::InputQueue;

const MAP_WIDTH: u8 = 8;
const MAP_HEIGHT: u8 = 9;

pub struct InputSystem;

impl InputSystem {
    pub fn new() -> Self {
        InputSystem
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, entities, mut positions, players, movables, immovables) = data;
        let mut to_move = Vec::new();

        for (position, _player) in (&positions, &players).join() {
            if let Some(key) = input_queue.pop() {
                // get all the movables and immovables
                let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                let immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                // Iterate through current position to the end of the map
                let (start, end, is_x) = match key {
                    KeyCode::Up | KeyCode::W => (position.y, 0, false),
                    KeyCode::Right | KeyCode::D => (position.x, MAP_WIDTH, true),
                    KeyCode::Down | KeyCode::S => (position.y, MAP_HEIGHT, false),
                    KeyCode::Left | KeyCode::A => (position.x, 0, true),
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    match mov.get(&pos) {
                        Some(id) => to_move.push((key, id.clone())),
                        None => match immov.get(&pos) {
                            Some(_id) => to_move.clear(),
                            None => break,
                        },
                    }
                }
            }
        }

        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));

            if let Some(position) = position {
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
