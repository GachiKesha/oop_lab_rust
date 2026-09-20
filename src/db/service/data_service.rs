use crate::{
    db::repository::{GameAccountRepositoryLike, GameRepositoryLike},
    game::{GameHandle, GameType},
    game_account::{GameAccountHandle, GameAccountType},
};

pub trait DataServiceLike {
    // Games
    fn create_game(
        &mut self,
        game_type: GameType,
        user: String,
        opponent: String,
        rating: i32,
        result: bool,
    ) -> Result<(), String>;
    fn create_game_ai(&mut self, user: String, rating: i32, result: bool) -> Result<(), String>;
    fn get_games(&self) -> Vec<GameHandle>;
    fn delete_game(&mut self, game: GameHandle);
    fn print_games(&self);
    fn print_games_user(&self, user: String) -> Result<(), String>;
    // Accounts
    fn create_game_account(
        &mut self,
        account_type: GameAccountType,
        username: String,
        initial_rating: Option<i32>,
    ) -> Result<(), String>;
    fn get_game_account(&self, name: String) -> Result<GameAccountHandle, String>;
    fn get_game_accounts(&self) -> Vec<GameAccountHandle>;
    fn update_game_account(&mut self, user: String, new_user: String) -> Result<(), String>;
    fn delete_game_account(&mut self, user: String) -> Result<(), String>;
}

pub struct DataService {
    game_repository: Box<dyn GameRepositoryLike>,
    game_account_repository: Box<dyn GameAccountRepositoryLike>,
}

impl DataService {
    pub fn new(
        game_repository: Box<dyn GameRepositoryLike>,
        game_account_repository: Box<dyn GameAccountRepositoryLike>,
    ) -> Self {
        Self {
            game_repository,
            game_account_repository,
        }
    }
}

impl DataServiceLike for DataService {
    fn create_game(
        &mut self,
        game_type: GameType,
        user: String,
        opponent: String,
        rating: i32,
        result: bool,
    ) -> Result<(), String> {
        if user == opponent {
            return Err("Can't play with yourself.".into());
        }
        if matches!(game_type, GameType::AI) {
            self.create_game_ai(user, rating, result)?;
            self.create_game_ai(opponent, rating, result)?;
        } else {
            let player1 = self.game_account_repository.read_game_acc_by_name(user)?;
            let player2 = self
                .game_account_repository
                .read_game_acc_by_name(opponent)?;
            self.game_repository
                .create_game(game_type, player1, Some(player2), rating, result)?;
        }
        Ok(())
    }

    fn create_game_ai(&mut self, user: String, rating: i32, result: bool) -> Result<(), String> {
        let player = self.game_account_repository.read_game_acc_by_name(user)?;
        self.game_repository
            .create_game(GameType::AI, player, None, rating, result)?;
        Ok(())
    }

    fn get_games(&self) -> Vec<GameHandle> {
        self.game_repository.read_games()
    }

    fn delete_game(&mut self, game: GameHandle) {
        self.game_repository.delete_game(game);
    }

    fn print_games(&self) {
        self.game_repository.print_games();
    }

    fn print_games_user(&self, user: String) -> Result<(), String> {
        let user = self.game_account_repository.read_game_acc_by_name(user)?;
        self.game_repository.print_games_player(user)?;
        Ok(())
    }

    fn create_game_account(
        &mut self,
        account_type: GameAccountType,
        username: String,
        initial_rating: Option<i32>,
    ) -> Result<(), String> {
        self.game_account_repository
            .create_game_account(account_type, username, initial_rating)
    }

    fn get_game_account(&self, name: String) -> Result<GameAccountHandle, String> {
        self.game_account_repository.read_game_acc_by_name(name)
    }

    fn get_game_accounts(&self) -> Vec<GameAccountHandle> {
        self.game_account_repository.read_game_accounts_mut()
    }

    fn update_game_account(&mut self, user: String, new_user: String) -> Result<(), String> {
        let game_account = self
            .game_account_repository
            .read_game_acc_by_name(user.clone())?;
        self.game_account_repository
            .update_game_account(game_account, new_user.clone())?;
        let game_account = self
            .game_account_repository
            .read_game_acc_by_name(user.clone())?;
        self.game_repository
            .update_games(game_account, user, new_user)?;
        Ok(())
    }

    fn delete_game_account(&mut self, user: String) -> Result<(), String> {
        let game_account = self.game_account_repository.read_game_acc_by_name(user)?;
        self.game_account_repository
            .delete_game_account(game_account);
        Ok(())
    }
}
