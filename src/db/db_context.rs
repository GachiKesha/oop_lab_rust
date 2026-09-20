use crate::{game::GameHandle, game_account::GameAccountHandle};

pub struct DbContext {
    game_accounts: Vec<GameAccountHandle>,
    games: Vec<GameHandle>,
}

impl DbContext {
    pub fn new() -> Self {
        Self {
            game_accounts: Vec::new(),
            games: Vec::new(),
        }
    }

    pub fn game_accounts(&self) -> &Vec<GameAccountHandle> {
        &self.game_accounts
    }

    pub fn game_accounts_mut(&mut self) -> &mut Vec<GameAccountHandle> {
        &mut self.game_accounts
    }

    pub fn set_game_accounts(&mut self, value: Vec<GameAccountHandle>) {
        self.game_accounts = value
    }

    pub fn games(&self) -> &Vec<GameHandle> {
        &self.games
    }

    pub fn games_mut(&mut self) -> &mut Vec<GameHandle> {
        &mut self.games
    }

    pub fn set_games(&mut self, value: Vec<GameHandle>) {
        self.games = value
    }
}
