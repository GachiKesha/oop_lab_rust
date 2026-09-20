use std::{cell::RefCell, rc::Rc};

use super::{AIGame, GameHandle, StandartGame, TrainingGame};

pub enum GameType {
    Standard,
    Training,
    AI,
}

pub fn create_game(
    game_type: GameType,
    player1: String,
    player2: String,
    rating: i32,
    result: bool,
) -> GameHandle {
    match game_type {
        GameType::Standard => Rc::new(RefCell::new(StandartGame::new(
            player1, player2, rating, result,
        ))),
        GameType::Training => Rc::new(RefCell::new(TrainingGame::new(
            player1, player2, rating, result,
        ))),
        GameType::AI => Rc::new(RefCell::new(AIGame::new(player1, rating, result))),
    }
}
