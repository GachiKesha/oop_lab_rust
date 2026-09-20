use super::Command;
use crate::db::DataServiceLike;
use colored::Colorize;
use std::{cell::RefCell, rc::Rc};

pub struct DeletePlayerCommand {
    data_service: Rc<RefCell<dyn DataServiceLike>>,
}

impl DeletePlayerCommand {
    pub fn new(data_service: Rc<RefCell<dyn DataServiceLike>>) -> Self {
        Self { data_service }
    }
}

impl Command for DeletePlayerCommand {
    fn execute(&mut self, command_parts: Vec<&str>) {
        if command_parts.len() != 2 {
            println!(
                "{}",
                format!("Invalid command. Usage: {} <username>", self.name()).bright_red()
            );
            return;
        }

        let mut data_service = self.data_service.borrow_mut();

        let username = command_parts[1];
        match data_service.delete_game_account(username.to_owned()) {
            Ok(_) => {
                print!("\n{} ", "Player".cyan());
                print!("{} ", username.bright_cyan());
                println!("{}\n", "deleted successfully.".cyan());
            }
            Err(message) => {
                println!("{}", message.bright_red());
            }
        };
    }

    fn show_info(&self) {
        println!(
            "\n{}",
            format!("{} <username>", self.name()).bright_yellow()
        );
        println!("{}", "#delete player".bright_green());
    }

    fn name(&self) -> &str {
        "deleteplayer"
    }
}
