use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{run, EventHandler, KeyCode, KeyMods};
use ggez::{Context, ContextBuilder, GameError, GameResult};
use specs::{RunNow, World, WorldExt};
use std::path::PathBuf;

use crate::component::register_components;
use crate::map::load_map;
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
        input_queue.push(keycode);
    }
}
