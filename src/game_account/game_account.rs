use std::{cell::RefCell, rc::Rc};

use crate::game::Game;

pub trait GameAccount {
    fn user_name(&self) -> &str;
    fn set_user_name(&mut self, value: String);

    fn current_rating(&self) -> i32;
    fn set_current_rating(&mut self, value: i32);

    fn games_count(&self) -> usize;

    fn game_rating(&mut self, result: bool, rating: i32);

    fn games_history(&self) -> &[i32];
    fn games_history_mut(&mut self) -> &mut Vec<i32>;

    fn win_game(&mut self, game: &dyn Game) {
        self.game_rating(true, game.rating());
        self.games_history_mut().push(game.game_index());
    }

    fn lose_game(&mut self, game: &dyn Game) {
        self.game_rating(false, game.rating());
        self.games_history_mut().push(game.game_index());
    }
}

pub type GameAccountHandle = Rc<RefCell<dyn GameAccount>>;
