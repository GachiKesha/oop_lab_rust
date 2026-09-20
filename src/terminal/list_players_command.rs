use super::Command;
use crate::db::DataServiceLike;
use colored::Colorize;
use std::{cell::RefCell, rc::Rc};

pub struct ListPlayersCommand {
    data_service: Rc<RefCell<dyn DataServiceLike>>,
}

impl ListPlayersCommand {
    pub fn new(data_service: Rc<RefCell<dyn DataServiceLike>>) -> Self {
        Self { data_service }
    }
}

impl Command for ListPlayersCommand {
    fn execute(&mut self, command_parts: Vec<&str>) {
        let len = command_parts.len();
        let data_service = self.data_service.borrow();
        if len == 1 {
            let players = data_service.get_game_accounts();

            if players.len() > 0 {
                println!("\n{}", "List of Players:".bright_yellow());
                println!("|------------------------------------------|");
                for player in players {
                    let player = player.borrow();
                    print!("{}", player.user_name().bright_cyan());
                    println!(
                        "{}",
                        format!("\tRating: {}", player.current_rating()).bright_yellow()
                    );
                    println!(
                        "{}",
                        format!("\t\tGames played: {}", player.games_count()).bright_yellow()
                    );
                    println!("|------------------------------------------|");
                }
            } else {
                println!("{}", "No players found.".cyan());
            }
        } else if len > 1 {
            println!("|------------------------------------------|");
            for i in 1..len {
                match data_service.get_game_account(command_parts[i].to_owned()) {
                    Ok(player) => {
                        let player = player.borrow();
                        print!("{}", player.user_name().bright_cyan());
                        println!(
                            "{}",
                            format!("\tRating: {}", player.current_rating()).bright_yellow()
                        );
                        println!(
                            "{}",
                            format!("\t\tGames played: {}", player.games_count()).bright_yellow()
                        );
                        println!("|------------------------------------------|");
                    }
                    Err(message) => {
                        println!("{}", message.bright_red());
                        println!("|------------------------------------------|");
                    }
                };
            }
        } else {
            println!(
                "{}",
                format!("Invalid command. Usage: players [<player1> <player2>...]").bright_red()
            );
        }
    }

    fn show_info(&self) {
        println!(
            "{}",
            format!("\n{} [<player1> <player2>...]", self.name()).bright_yellow()
        );
        println!(
            "{}",
            "#list of players or stats of players if provided".bright_green()
        );
    }

    fn name(&self) -> &str {
        "players"
    }
}
