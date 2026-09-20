use super::{BaseGame, Game};

pub struct StandartGame {
    base: BaseGame,
}

impl StandartGame {
    pub fn new(player1: String, player2: String, rating: i32, result: bool) -> Self {
        let rating = Self::game_rating(rating);
        Self {
            base: BaseGame::new(player1, player2, rating, result),
        }
    }

    fn game_rating(rating: i32) -> i32 {
        rating
    }
}

impl Game for StandartGame {
    fn player1(&self) -> &str {
        self.base.player1()
    }

    fn set_player1(&mut self, value: String) {
        self.base.set_player1(value);
    }

    fn player2(&self) -> &str {
        self.base.player2()
    }

    fn set_player2(&mut self, value: String) {
        self.base.set_player2(value);
    }

    fn rating(&self) -> i32 {
        self.base.rating()
    }

    fn new_rating_player1(&self) -> i32 {
        self.base.new_rating_player1()
    }

    fn set_new_rating_player1(&mut self, value: i32) {
        self.base.set_new_rating_player1(value)
    }

    fn new_rating_player2(&self) -> i32 {
        self.base.new_rating_player2()
    }

    fn set_new_rating_player2(&mut self, value: i32) {
        self.base.set_new_rating_player2(value)
    }

    fn game_index(&self) -> i32 {
        self.base.game_index()
    }

    fn set_game_index(&mut self, value: i32) {
        self.base.set_game_index(value)
    }

    fn result(&self) -> bool {
        self.base.result()
    }
}
