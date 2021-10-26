use specs::{Builder, World, WorldExt};

use crate::component::{BoxItem, BoxSpot, Player, Position, Renderable, Wall};

pub fn create_box_item(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: String::from("/assets/images/box_item.png"),
        })
        .with(BoxItem {})
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: String::from("/assets/images/box_spot.png"),
        })
        .with(BoxSpot {})
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable {
            path: String::from("/assets/images/floor.png"),
        })
        .build();
}

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: String::from("/assets/images/player.png"),
        })
        .with(Player {})
        .build();
}

pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: String::from("/assets/images/wall.png"),
        })
        .with(Wall {})
        .build();
}
