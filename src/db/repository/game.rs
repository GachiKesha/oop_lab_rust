use std::cell::RefCell;
use std::rc::Rc;

use crate::db::DbContext;
use crate::game::{Game, GameHandle, GameType, create_game};

use crate::game_account::GameAccountHandle;
use colored::Colorize;

pub trait GameRepositoryLike {
    fn create_game(
        &self,
        gametype: GameType,
        player1: GameAccountHandle,
        player2: Option<GameAccountHandle>,
        rating: i32,
        result: bool,
    ) -> Result<(), String>;
    fn read_games(&self) -> Vec<GameHandle>;
    fn read_game_by_id(&self, id: i32) -> Option<GameHandle>;
    fn update_games(
        &self,
        game_account: GameAccountHandle,
        player: String,
        new_player: String,
    ) -> Result<(), String>;
    fn delete_game(&self, game: GameHandle);
    fn print_games(&self);
    fn print_games_player(&self, player: GameAccountHandle) -> Result<(), String>;
}

pub struct GameRepository {
    db_context: Rc<RefCell<DbContext>>,
}

impl GameRepository {
    pub fn new(db_context: Rc<RefCell<DbContext>>) -> Self {
        Self { db_context }
    }
}

impl GameRepositoryLike for GameRepository {
    fn create_game(
        &self,
        gametype: GameType,
        player1: GameAccountHandle,
        player2: Option<GameAccountHandle>,
        rating: i32,
        result: bool,
    ) -> Result<(), String> {
        if rating < 0 {
            return Err("Rating cannot be less than one.".into());
        }
        let player2_name = match player2.clone() {
            Some(name) => name.borrow().user_name().into(),
            None => "AI".into(),
        };
        let game = create_game(
            gametype,
            player1.borrow().user_name().into(),
            player2_name,
            rating,
            result,
        );
        let mut game_borrow = game.borrow_mut();
        let game_ref = &mut *game_borrow;
        if let Some(player2) = player2 {
            if result {
                player1.borrow_mut().win_game(game_ref);
                player2.borrow_mut().lose_game(game_ref);
            } else {
                player2.borrow_mut().win_game(game_ref);
                player1.borrow_mut().lose_game(game_ref);
            }
            game_ref.set_new_rating_player1(player1.borrow().current_rating());
            game_ref.set_new_rating_player2(player2.borrow().current_rating());
        } else {
            if result {
                player1.borrow_mut().win_game(game_ref);
            } else {
                player1.borrow_mut().lose_game(game_ref);
            }
            game_ref.set_new_rating_player1(player1.borrow().current_rating());
        }
        drop(game_borrow);
        self.db_context.borrow_mut().games_mut().push(game);
        Ok(())
    }

    fn read_games(&self) -> Vec<GameHandle> {
        self.db_context.borrow().games().clone()
    }

    fn read_game_by_id(&self, id: i32) -> Option<GameHandle> {
        self.db_context
            .borrow()
            .games()
            .iter()
            .find(|game| game.borrow().game_index() == id)
            .cloned()
    }

    fn update_games(
        &self,
        game_account: GameAccountHandle,
        player: String,
        new_player: String,
    ) -> Result<(), String> {
        let game_account = game_account.borrow();
        let games = game_account.games_history();
        for &id in games {
            if let Some(game) = self.read_game_by_id(id) {
                let mut game = game.borrow_mut();
                {
                    let game: &mut dyn Game = &mut *game;
                    let player = player.clone();
                    let new_player = new_player.clone();
                    if game.player1() == player {
                        game.set_player1(new_player);
                    } else if game.player2() == player {
                        game.set_player2(new_player);
                    }
                }
            } else {
                return Err(format!("No such game with id \"{id}\""));
            };
        }
        Ok(())
    }

    fn delete_game(&self, game: GameHandle) {
        if let Some(index) = self
            .db_context
            .borrow()
            .games()
            .iter()
            .position(|g| g.borrow().game_index() == game.borrow().game_index())
        {
            self.db_context.borrow_mut().games_mut().remove(index);
        }
    }

    fn print_games(&self) {
        if self.db_context.borrow().games().len() == 0 {
            println!("{}", "No games found.".cyan());
            return;
        }

        println!("{}", "\nAll games in database:".yellow());
        println!("|------------------------------------------|");
        for game in self.db_context.borrow().games().iter() {
            let game = game.borrow();
            println!(
                "|#{}\t{} vs {}\n",
                game.game_index(),
                game.player1(),
                game.player2()
            );
            print!("|\tResult: ");

            print!(
                "{}",
                format!(
                    "{} won",
                    if game.result() {
                        game.player1()
                    } else {
                        game.player2()
                    }
                )
                .yellow()
            );

            println!("{}", format!("\tRating: {}", game.rating()).yellow());
            println!("|------------------------------------------|");
        }
        println!("\n");
    }

    fn print_games_player(&self, player: GameAccountHandle) -> Result<(), String> {
        let player = player.borrow();
        if player.games_history().len() == 0 {
            println!("{}", "No games found.".bright_cyan());
            return Ok(());
        }

        print!("\nGames history for ");
        println!("{}", player.user_name().bright_cyan());
        println!(
            "{}",
            format!("Current rating: {}", player.current_rating()).bright_cyan()
        );
        println!("|------------------------------------------|");

        for &id in player.games_history() {
            let Some(game) = self.read_game_by_id(id) else {
                return Err("No such game with id \"{id}\"".into());
            };

            let game = game.borrow();
            let (opponent, won, new_rating) = if game.player1() == player.user_name() {
                (game.player2(), game.result(), game.new_rating_player1())
            } else {
                (game.player1(), !game.result(), game.new_rating_player2())
            };

            let rating = if won { game.rating() } else { -game.rating() };

            print!("{}", format!("|#{} ", game.game_index()).bright_cyan());
            println!("{} vs {}", player.user_name(), opponent);
            if won {
                print!("{}", "|\tResult: won".bright_green());
                print!("{}", format!("\tRating: {}", rating).bright_green());
            } else {
                print!("{}", "|\tResult: lost".bright_red());
                print!("{}", format!("\tRating: {}", rating).bright_red());
            }
            println!("{}", format!(" =>{}", new_rating).bright_yellow());
            println!("|------------------------------------------|");
        }
        println!();
        Ok(())
    }
}
