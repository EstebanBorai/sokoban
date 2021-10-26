mod component;
mod entity;
mod game;
mod system;

use ggez::GameError;

fn main() -> Result<(), GameError> {
    game::Game::start()
}
