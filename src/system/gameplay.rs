use specs::{Join, ReadStorage, System, Write};
use std::collections::HashMap;

use crate::component::{BoxItem, BoxSpot, Position};
use crate::resource::{Gameplay, GameplayState};

pub struct GameplaySystem {}

impl GameplaySystem {
    pub fn new() -> Self {
        GameplaySystem {}
    }
}

impl<'a> System<'a> for GameplaySystem {
    type SystemData = (
        Write<'a, Gameplay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BoxItem>,
        ReadStorage<'a, BoxSpot>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gameplay, positions, boxes, box_spots) = data;
        let boxes_by_position: HashMap<(u8, u8), &BoxItem> = (&positions, &boxes)
            .join()
            .map(|(pos, box_item)| ((pos.x, pos.y), box_item))
            .collect::<HashMap<_, _>>();

        for (_, position) in (&box_spots, &positions).join() {
            if !boxes_by_position.contains_key(&(position.x, position.y)) {
                gameplay.state = GameplayState::Playing;
                return;
            }
        }

        gameplay.state = GameplayState::Won;
    }
}
