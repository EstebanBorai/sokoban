use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{run, EventHandler, KeyCode, KeyMods};
use ggez::{Context, ContextBuilder, GameError, GameResult};
use specs::{RunNow, World, WorldExt};
use std::path::PathBuf;

use crate::component::{register_components, Position};
use crate::entity::{create_box_item, create_box_spot, create_floor, create_player, create_wall};
use crate::resource::{register_resources, InputQueue};
use crate::system::{InputSystem, RenderingSystem};

fn initialize_level(world: &mut World) {
    const MAP: &str = "
    W W W W W W W W
    W . . . . . . W
    W . . . B . . W
    W . . . . . . W
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    load_map(world, String::from(MAP));
}

fn load_map(world: &mut World, map: String) {
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

pub struct Game {
    world: World,
}

impl Game {
    pub fn start() -> GameResult {
        let mut world = World::new();

        register_components(&mut world);
        register_resources(&mut world);
        initialize_level(&mut world);

        let context_builder = ContextBuilder::new("sokoban_remake_rust", "sokoban")
            .window_setup(WindowSetup::default().title("Sokoban"))
            .window_mode(WindowMode::default().dimensions(800., 600.))
            .add_resource_path(PathBuf::from("./assets"));

        let (context, event_loop) = context_builder.build()?;

        let game = Game { world };

        run(context, event_loop, game);
    }
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        {
            let mut input_sys = InputSystem::new();
            input_sys.run_now(&self.world);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        {
            let mut rendering_sys = RenderingSystem::new(context);

            rendering_sys.run_now(&self.world);
        }

        Ok(())
    }

    fn key_down_event(&mut self, _: &mut Context, keycode: KeyCode, _: KeyMods, _: bool) {
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode);
    }
}
