use std::{cell::RefCell, rc::Rc};

use crate::{
    db::DbContext,
    game_account::{GameAccountHandle, GameAccountType, create_game_account},
};

pub trait GameAccountRepositoryLike {
    fn create_game_account(
        &self,
        account_type: GameAccountType,
        username: String,
        initial_rating: Option<i32>,
    ) -> Result<(), String>;
    fn read_game_accounts(&self) -> Vec<GameAccountHandle>;
    fn read_game_accounts_mut(&self) -> Vec<GameAccountHandle>;
    fn read_game_acc_by_name(&self, name: String) -> Result<GameAccountHandle, String>;
    fn update_game_account(
        &self,
        game_account: GameAccountHandle,
        new_name: String,
    ) -> Result<(), String>;
    fn delete_game_account(&self, game_account: GameAccountHandle);
}

pub struct GameAccountRepository {
    db_context: Rc<RefCell<DbContext>>,
}

impl GameAccountRepository {
    pub fn new(db_context: Rc<RefCell<DbContext>>) -> Self {
        Self { db_context }
    }
}

impl GameAccountRepositoryLike for GameAccountRepository {
    fn create_game_account(
        &self,
        account_type: GameAccountType,
        username: String,
        initial_rating: Option<i32>,
    ) -> Result<(), String> {
        if !self
            .db_context
            .borrow()
            .game_accounts()
            .iter()
            .any(|acc| acc.borrow().user_name() == username)
        {
            self.db_context
                .borrow_mut()
                .game_accounts_mut()
                .push(create_game_account(account_type, username, initial_rating));
            Ok(())
        } else {
            Err(format!("There is already player with name \"{username}\""))
        }
    }

    fn read_game_accounts(&self) -> Vec<GameAccountHandle> {
        self.db_context.borrow().game_accounts().clone()
    }

    fn read_game_accounts_mut(&self) -> Vec<GameAccountHandle> {
        self.db_context.borrow_mut().game_accounts_mut().clone()
    }

    fn read_game_acc_by_name(&self, name: String) -> Result<GameAccountHandle, String> {
        self.db_context
            .borrow()
            .game_accounts()
            .iter()
            .find(|acc| acc.borrow().user_name() == name)
            .cloned()
            .ok_or_else(|| format!("No such player \"{name}\""))
    }

    fn update_game_account(
        &self,
        game_account: GameAccountHandle,
        new_name: String,
    ) -> Result<(), String> {
        if !self
            .db_context
            .borrow()
            .game_accounts()
            .iter()
            .any(|acc| acc.borrow().user_name() == new_name)
        {
            game_account.borrow_mut().set_user_name(new_name);
            Ok(())
        } else {
            Err(format!("There is already player with name \"{new_name}\""))
        }
    }

    fn delete_game_account(&self, game_account: GameAccountHandle) {
        if let Some(index) = self
            .db_context
            .borrow()
            .game_accounts()
            .iter()
            .position(|acc| acc.borrow().user_name() == game_account.borrow().user_name())
        {
            self.db_context
                .borrow_mut()
                .game_accounts_mut()
                .remove(index);
        }
    }
}
