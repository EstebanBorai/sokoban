mod component;
mod entity;
mod game;

use ggez::GameError;

fn main() -> Result<(), GameError> {
    game::Game::start()
}
