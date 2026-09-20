use super::Command;
use crate::{db::DataServiceLike, game::GameType};
use colored::Colorize;
use std::{cell::RefCell, rc::Rc};

pub struct PlayGameCommand {
    data_service: Rc<RefCell<dyn DataServiceLike>>,
}

impl PlayGameCommand {
    pub fn new(data_service: Rc<RefCell<dyn DataServiceLike>>) -> Self {
        Self { data_service }
    }
}

impl Command for PlayGameCommand {
    fn execute(&mut self, command_parts: Vec<&str>) {
        let len = command_parts.len();
        if len < 2 || len > 6 {
            println!("{}", format!("Invalid command. Usage: {} <gametype> <player1> <player2> <rating> <win_of_first?(bool)>", self.name()).bright_red());
            return;
        }

        let game_type = match command_parts[1].to_ascii_lowercase().as_str() {
            "standard" => GameType::Standard,
            "ai" => GameType::AI,
            "training" => GameType::Training,
            wrong => {
                println!(
                    "{}",
                    format!("Invalid game type \"{}\"", wrong).bright_red()
                );
                return;
            }
        };

        let Ok(rating) = command_parts[4].parse::<i32>() else {
            println!("{}", "Invalid rating. Input integer value".bright_red());
            return;
        };
        let Ok(result) = command_parts[5].parse::<bool>() else {
            println!("{}", "Invalid result value. Input bool value".bright_red());
            return;
        };

        let mut data_service = self.data_service.borrow_mut();
        match data_service.create_game(
            game_type,
            command_parts[2].to_owned(),
            command_parts[3].to_owned(),
            rating,
            result,
        ) {
            Ok(_) => {
                println!("{}", "\nGame successfuly created.\n".bright_green());
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
                "\n{} <gametype> <player1> <player2> <rating> <win_of_first?(bool)>",
                self.name()
            )
            .bright_yellow()
        );
        println!(
            "{}",
            "#create game with 2 players\n#Game types: standard, training, ai".bright_green()
        );
    }

    fn name(&self) -> &str {
        "playgame"
    }
}
