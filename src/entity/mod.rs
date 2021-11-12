use specs::{Builder, World, WorldExt};

use crate::component::{BoxItem, BoxSpot, Immovable, Movable, Player, Position, Renderable, Wall};

pub fn create_box_item(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new("/images/box_item.png"))
        .with(BoxItem {})
        .with(Movable {})
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 9, ..position })
        .with(Renderable::new("/images/box_spot.png"))
        .with(BoxSpot {})
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable::new("/images/floor.png"))
        .build();
}

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new("/images/player.png"))
        .with(Player {})
        .with(Movable {})
        .build();
}

pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new("/images/wall.png"))
        .with(Wall {})
        .with(Immovable {})
        .build();
}
