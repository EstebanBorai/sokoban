use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{run, EventHandler};
use ggez::{Context, ContextBuilder, GameError, GameResult};
use specs::{RunNow, World, WorldExt};
use std::path::PathBuf;

use crate::component::{BoxItem, BoxSpot, Player, Position, Renderable, Wall};
use crate::entity::{create_box_item, create_player, create_wall};
use crate::system::RenderingSystem;

fn register_components(world: &mut World) {
    world.register::<BoxItem>();
    world.register::<BoxSpot>();
    world.register::<Player>();
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Wall>();
}

fn initialize_level(world: &mut World) {
    create_player(world, Position::new(0, 0, 0));
    create_wall(world, Position::new(1, 0, 0));
    create_box_item(world, Position::new(2, 0, 0));
}

pub struct Game {
    world: World,
}

impl Game {
    pub fn start() -> GameResult {
        let mut world = World::new();

        register_components(&mut world);
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
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        {
            let mut rendering_sys = RenderingSystem::new(context);

            rendering_sys.run_now(&self.world);
        }

        Ok(())
    }
}
