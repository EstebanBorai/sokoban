use std::fmt::Display;

pub enum GameplayState {
    Playing,
    Won,
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

impl Display for GameplayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            GameplayState::Playing => f.write_str("Playing"),
            GameplayState::Won => f.write_str("Won"),
        }
    }
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves: u32,
}
