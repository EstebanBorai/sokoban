mod component;
mod entity;
mod game;
mod resource;
mod system;

use ggez::GameError;

fn main() -> Result<(), GameError> {
    game::Game::start()
}
