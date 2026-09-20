use super::Command;
use crate::db::DataServiceLike;
use colored::Colorize;
use std::{cell::RefCell, rc::Rc};

pub struct ChangeNameCommand {
    data_service: Rc<RefCell<dyn DataServiceLike>>,
}

impl ChangeNameCommand {
    pub fn new(data_service: Rc<RefCell<dyn DataServiceLike>>) -> Self {
        Self { data_service }
    }
}

impl Command for ChangeNameCommand {
    fn execute(&mut self, command_parts: Vec<&str>) {
        if command_parts.len() != 3 {
            println!(
                "{}",
                format!(
                    "Invalid command. Usage: {} <username> <new_username>",
                    self.name()
                )
                .bright_red()
            );
            return;
        }
        let mut data_service = self.data_service.borrow_mut();
        let username = command_parts[1];
        let new_username = command_parts[2];
        match data_service.update_game_account(username.to_owned(), new_username.to_owned()) {
            Ok(_) => {
                print!("\n{} ", "Changed name of player".cyan());
                print!("{} ", username.bright_cyan());
                print!("{} ", "to".cyan());
                println!("{}\n", new_username.bright_cyan());
            }
            Err(message) => {
                println!("{}", message.bright_red());
            }
        };
    }

    fn show_info(&self) {
        println!(
            "{}",
            format!("\n{} <username> <new_username>", self.name()).bright_yellow()
        );
        println!("{}", "#change name of player".bright_green());
    }

    fn name(&self) -> &str {
        "changename"
    }
}
