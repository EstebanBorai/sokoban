use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{run, EventHandler};
use ggez::{Context, ContextBuilder, GameError, GameResult};
use specs::{World, WorldExt};
use std::path::PathBuf;

use crate::component::{BoxItem, BoxSpot, Player, Position, Renderable, Wall};

fn register_components(world: &mut World) {
    world.register::<BoxItem>();
    world.register::<BoxSpot>();
    world.register::<Player>();
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Wall>();
}

pub struct Game {
    world: World,
}

impl Game {
    pub fn start() -> GameResult {
        let mut world = World::new();

        register_components(&mut world);

        let context_builder = ContextBuilder::new("sokoban_remake_rust", "sokoban")
            .window_setup(WindowSetup::default().title("Sokoban"))
            .window_mode(WindowMode::default().dimensions(800.0, 600.0))
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

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }
}
