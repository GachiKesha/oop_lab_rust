use super::Command;
use crate::{db::DataServiceLike, game_account::GameAccountType};
use colored::Colorize;
use std::{cell::RefCell, rc::Rc};

pub struct CreatePlayerCommand {
    data_service: Rc<RefCell<dyn DataServiceLike>>,
}

impl CreatePlayerCommand {
    pub fn new(data_service: Rc<RefCell<dyn DataServiceLike>>) -> Self {
        Self { data_service }
    }
}

impl Command for CreatePlayerCommand {
    fn execute(&mut self, command_parts: Vec<&str>) {
        let len = command_parts.len();
        if len != 3 && len != 4 {
            println!(
                "{}",
                format!(
                    "Invalid command. Usage: {} <accountType> <username> [<initialRating>]",
                    self.name()
                )
                .bright_red()
            );
            return;
        }

        let account_type = match command_parts[1].to_ascii_lowercase().as_str() {
            "standard" => GameAccountType::Standard,
            "noob" => GameAccountType::Noob,
            "winstreak" => GameAccountType::WinStreak,
            wrong => {
                println!(
                    "{}",
                    format!("Invalid account type \"{}\"", wrong).bright_red()
                );
                return;
            }
        };
        let username = command_parts[2];
        let mut data_service = self.data_service.borrow_mut();
        let initial_rating = if len == 3 {
            None
        } else {
            command_parts[3].parse::<i32>().ok()
        };

        match data_service.create_game_account(account_type, username.to_owned(), initial_rating) {
            Ok(_) => {
                print!("\n{} ", "Player".bright_green());
                print!("{} ", username.bright_cyan());
                println!("{}\n", "created successfully.".bright_green());
            }
            Err(message) => {
                println!("{}", message.bright_red());
            }
        };
    }

    fn show_info(&self) {
        println!(
            "{}",
            format!(
                "\n{} <accountType> <username> [<initialRating>]",
                self.name()
            )
            .bright_yellow()
        );
        println!(
            "{}",
            "#create player\n#Account types: standard, noob, winstreak".bright_green()
        );
    }

    fn name(&self) -> &str {
        "createplayer"
    }
}
