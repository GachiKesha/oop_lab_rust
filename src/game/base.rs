use std::sync::atomic::AtomicI32;

use super::Game;

static SEED: AtomicI32 = AtomicI32::new(0);

pub struct BaseGame {
    player1: String,
    player2: String,
    rating: i32,
    new_rating_player1: i32,
    new_rating_player2: i32,
    game_index: i32,
    result: bool,
}

impl BaseGame {
    pub fn new(player1: String, player2: String, rating: i32, result: bool) -> Self {
        Self {
            player1,
            player2,
            rating,
            new_rating_player1: 0,
            new_rating_player2: 0,
            game_index: SEED.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1,
            result,
        }
    }
}

impl Game for BaseGame {
    fn player1(&self) -> &str {
        &self.player1
    }

    fn set_player1(&mut self, value: String) {
        self.player1 = value
    }

    fn player2(&self) -> &str {
        &self.player2
    }

    fn set_player2(&mut self, value: String) {
        self.player2 = value
    }

    fn rating(&self) -> i32 {
        self.rating
    }

    fn new_rating_player1(&self) -> i32 {
        self.new_rating_player1
    }

    fn set_new_rating_player1(&mut self, value: i32) {
        self.new_rating_player1 = value;
    }

    fn new_rating_player2(&self) -> i32 {
        self.new_rating_player2
    }

    fn set_new_rating_player2(&mut self, value: i32) {
        self.new_rating_player2 = value;
    }

    fn game_index(&self) -> i32 {
        self.game_index
    }

    fn set_game_index(&mut self, value: i32) {
        self.game_index = value;
    }

    fn result(&self) -> bool {
        self.result
    }
}
