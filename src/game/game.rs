use std::{cell::RefCell, rc::Rc};

pub trait Game {
    fn player1(&self) -> &str;
    fn set_player1(&mut self, value: String);

    fn player2(&self) -> &str;
    fn set_player2(&mut self, value: String);

    fn rating(&self) -> i32;

    fn new_rating_player1(&self) -> i32;
    fn set_new_rating_player1(&mut self, value: i32);

    fn new_rating_player2(&self) -> i32;
    fn set_new_rating_player2(&mut self, value: i32);

    fn game_index(&self) -> i32;
    fn set_game_index(&mut self, value: i32);

    fn result(&self) -> bool;
}

pub type GameHandle = Rc<RefCell<dyn Game>>;
