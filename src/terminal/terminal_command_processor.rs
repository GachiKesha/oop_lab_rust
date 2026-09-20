use std::{cell::RefCell, rc::Rc};

use crate::db::DataServiceLike;

use super::{
    AllGamesCommand, AutoTestCommand, ChangeNameCommand, Command, CreatePlayerCommand,
    DeletePlayerCommand, ListPlayersCommand, PlayGameCommand,
};
use colored::Colorize;

pub struct TerminalCommandProcessor {
    commands: Vec<Box<dyn Command>>,
}

impl TerminalCommandProcessor {
    pub fn new(data_service: Rc<RefCell<dyn DataServiceLike>>) -> Self {
        Self {
            commands: vec![
                Box::new(AllGamesCommand::new(data_service.clone())),
                Box::new(AutoTestCommand::new(data_service.clone())),
                Box::new(ChangeNameCommand::new(data_service.clone())),
                Box::new(CreatePlayerCommand::new(data_service.clone())),
                Box::new(DeletePlayerCommand::new(data_service.clone())),
                Box::new(ListPlayersCommand::new(data_service.clone())),
                Box::new(PlayGameCommand::new(data_service.clone())),
            ],
        }
    }

    pub fn process_commands(&mut self) {
        let stdin = std::io::stdin();
        let mut command = String::new();
        loop {
            command.clear();
            println!(
                "{}",
                "Enter a command: (type \"help\" for help)".bright_cyan()
            );
            if let Err(err) = stdin.read_line(&mut command) {
                eprintln!("{}", err.to_string());
                continue;
            };

            if command.is_empty() {
                println!("{}", "Invalid command. Please try again.".bright_red());
                continue;
            }

            let command_parts: Vec<&str> = command.split_whitespace().collect();

            let result = command_parts[0].to_lowercase();
            if result == "exit" {
                println!("{}", "\nOK.".bright_green());
                return;
            }
            if result == "help" {
                self.help_command(command_parts);
                continue;
            }

            let mut done = false;
            for com in self.commands.iter_mut() {
                if com.name() == result {
                    com.execute(command_parts);
                    done = true;
                    break;
                }
            }

            if !done {
                println!("{}", "Invalid command. Please try again.".bright_red());
                continue;
            }
        }
    }

    pub fn help_command(&self, command_parts: Vec<&str>) {
        let len = command_parts.len();
        if len == 1 {
            println!("{}", "List of commands:".bright_cyan());
            for com in &self.commands {
                com.show_info();
            }
            println!("{}", "\nhelp".bright_yellow());
            println!("{}", "#list of commands".bright_green());
            println!("{}", "\nexit".bright_yellow());
            println!("{}", "#exit app".bright_green());
            return;
        }
        if len == 2 {
            let mut done = false;
            for com in &self.commands {
                if com.name() == command_parts[1] {
                    com.show_info();
                    done = true;
                    break;
                }
            }
            if !done {
                println!("{}", "Invalid command. Please try again.".bright_red());
            }
        } else {
            println!("{}", "Unexpected arguments. Usage: help".bright_red());
        }
    }
}
