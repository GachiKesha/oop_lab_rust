use super::Command;
use crate::db::DataServiceLike;
use colored::Colorize;
use std::{cell::RefCell, rc::Rc};

pub struct AllGamesCommand {
    data_service: Rc<RefCell<dyn DataServiceLike>>,
}

impl AllGamesCommand {
    pub fn new(data_service: Rc<RefCell<dyn DataServiceLike>>) -> Self {
        Self { data_service }
    }
}

impl Command for AllGamesCommand {
    fn execute(&mut self, command_parts: Vec<&str>) {
        let len = command_parts.len();
        let data_service = self.data_service.borrow();
        if len == 1 {
            data_service.print_games();
        } else {
            for i in 1..len {
                match data_service.print_games_user(command_parts[i].to_owned()) {
                    Ok(_) => (),
                    Err(message) => {
                        println!("{}", message.bright_red());
                    }
                }
            }
        }
    }

    fn show_info(&self) {
        println!(
            "{}",
            format!("\n{} [<player1> <player2>...]", self.name()).bright_yellow()
        );
        println!(
            "{}",
            "#list of games played (all games if no player provided)".bright_green()
        );
    }

    fn name(&self) -> &str {
        "games"
    }
}
