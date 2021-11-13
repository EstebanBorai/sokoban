use specs::World;

use crate::component::Position;
use crate::entity::{create_box_item, create_box_spot, create_floor, create_player, create_wall};

pub fn load_map(world: &mut World, map: String) {
    let rows: Vec<&str> = map.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            let position = Position::new(x as u8, y as u8, 0);

            match *column {
                "." => create_floor(world, position),
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                }
                "B" => {
                    create_floor(world, position);
                    create_box_item(world, position);
                }
                "S" => {
                    create_floor(world, position);
                    create_box_spot(world, position);
                }
                "N" => (),
                character => panic!("Invalid map character: {}", character),
            }
        }
    }
}
