use std::{cell::RefCell, rc::Rc};

use crate::game_account::{GameAccountHandle, NoobAccount, StandardAccount, WinStreakAccount};

pub enum GameAccountType {
    Standard,
    Noob,
    WinStreak,
}

pub fn create_game_account(
    game_account_type: GameAccountType,
    user_name: String,
    initial_rating: Option<i32>,
) -> GameAccountHandle {
    match game_account_type {
        GameAccountType::Standard => Rc::new(RefCell::new(StandardAccount::new(
            user_name,
            initial_rating,
        ))),
        GameAccountType::Noob => Rc::new(RefCell::new(NoobAccount::new(user_name, initial_rating))),
        GameAccountType::WinStreak => Rc::new(RefCell::new(WinStreakAccount::new(
            user_name,
            initial_rating,
        ))),
    }
}
