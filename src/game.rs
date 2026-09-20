mod ai_game;
mod base;
mod factory;
mod game;
mod standard_game;
mod training_game;

pub use ai_game::AIGame;
pub use base::BaseGame;
pub use factory::{GameType, create_game};
pub use game::{Game, GameHandle};
pub use standard_game::StandartGame;
pub use training_game::TrainingGame;
