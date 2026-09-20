use super::Command;
use crate::{db::DataServiceLike, game::GameType, game_account::GameAccountType};
use colored::Colorize;
use std::{cell::RefCell, rc::Rc};

pub struct AutoTestCommand {
    data_service: Rc<RefCell<dyn DataServiceLike>>,
    test_runned: bool,
}

impl AutoTestCommand {
    pub fn new(data_service: Rc<RefCell<dyn DataServiceLike>>) -> Self {
        Self {
            data_service,
            test_runned: false,
        }
    }
}

impl Command for AutoTestCommand {
    fn execute(&mut self, command_parts: Vec<&str>) {
        if command_parts.len() != 1 {
            println!(
                "{}",
                format!("Unexpected arguments. Usage: {}", self.name()).bright_red()
            );
            return;
        }
        if self.test_runned {
            println!("{}", "Already generated test units.".bright_red());
            return;
        }

        let mut data_service = self.data_service.borrow_mut();

        let result: Result<(), String> = (|| {
            data_service.create_game_account(
                GameAccountType::Standard,
                "Crico(test)".to_owned(),
                Some(1000),
            )?;
            data_service.create_game_account(
                GameAccountType::WinStreak,
                "Kawasaki(test)".to_owned(),
                Some(500),
            )?;
            data_service.create_game_account(
                GameAccountType::Noob,
                "Cago(test)".to_owned(),
                Some(1000),
            )?;
            data_service.create_game_account(
                GameAccountType::Standard,
                "Elstripper(test)".to_owned(),
                None,
            )?;
            data_service.create_game_account(
                GameAccountType::WinStreak,
                "Jotaro(test)".to_owned(),
                None,
            )?;
            println!("Accounts created.");
            data_service.create_game_ai("Cago(test)".to_owned(), 10, true)?;
            data_service.create_game_ai("Crico(test)".to_owned(), 100, false)?;
            data_service.create_game_ai("Kawasaki(test)".to_owned(), 50, true)?;
            data_service.create_game_ai("Cago(test)".to_owned(), 50, false)?;
            data_service.create_game(
                GameType::AI,
                "Elstripper(test)".to_owned(),
                "Jotaro(test)".to_owned(),
                25,
                true,
            )?;

            data_service.create_game(
                GameType::Standard,
                "Cago(test)".to_owned(),
                "Crico(test)".to_owned(),
                20,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Cago(test)".to_owned(),
                "Kawasaki(test)".to_owned(),
                10,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Cago(test)".to_owned(),
                "Jotaro(test)".to_owned(),
                34,
                false,
            )?;

            data_service.create_game(
                GameType::Training,
                "Cago(test)".to_owned(),
                "Crico(test)".to_owned(),
                100,
                true,
            )?;
            data_service.create_game(
                GameType::Training,
                "Elstripper(test)".to_owned(),
                "Cago(test)".to_owned(),
                10,
                false,
            )?;
            data_service.create_game(
                GameType::Training,
                "Kawasaki(test)".to_owned(),
                "Jotaro(test)".to_owned(),
                34,
                false,
            )?;

            data_service.create_game(
                GameType::Standard,
                "Cago(test)".to_owned().to_owned(),
                "Crico(test)".to_owned(),
                20,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Cago(test)".to_owned(),
                "Kawasaki(test)".to_owned(),
                10,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Cago(test)".to_owned(),
                "Jotaro(test)".to_owned(),
                34,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Cago(test)".to_owned(),
                "Crico(test)".to_owned(),
                20,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Cago(test)".to_owned(),
                "Kawasaki(test)".to_owned(),
                10,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Cago(test)".to_owned(),
                "Jotaro(test)".to_owned(),
                34,
                false,
            )?;

            data_service.create_game(
                GameType::Standard,
                "Crico(test)".to_owned(),
                "Kawasaki(test)".to_owned(),
                85,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Kawasaki(test)".to_owned(),
                "Cago(test)".to_owned(),
                60,
                true,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Cago(test)".to_owned(),
                "Elstripper(test)".to_owned(),
                45,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Elstripper(test)".to_owned(),
                "Jotaro(test)".to_owned(),
                75,
                true,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Jotaro(test)".to_owned(),
                "Crico(test)".to_owned(),
                50,
                false,
            )?;

            data_service.create_game(
                GameType::Standard,
                "Crico(test)".to_owned(),
                "Cago(test)".to_owned(),
                70,
                true,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Crico(test)".to_owned(),
                "Elstripper(test)".to_owned(),
                35,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Crico(test)".to_owned(),
                "Jotaro(test)".to_owned(),
                92,
                true,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Kawasaki(test)".to_owned(),
                "Cago(test)".to_owned(),
                15,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Kawasaki(test)".to_owned(),
                "Elstripper(test)".to_owned(),
                80,
                true,
            )?;

            data_service.create_game(
                GameType::Standard,
                "Kawasaki(test)".to_owned(),
                "Jotaro(test)".to_owned(),
                55,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Cago(test)".to_owned(),
                "Elstripper(test)".to_owned(),
                88,
                true,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Cago(test)".to_owned(),
                "Jotaro(test)".to_owned(),
                40,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Elstripper(test)".to_owned(),
                "Jotaro(test)".to_owned(),
                70,
                true,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Kawasaki(test)".to_owned(),
                "Crico(test)".to_owned(),
                25,
                false,
            )?;

            data_service.create_game(
                GameType::Standard,
                "Cago(test)".to_owned(),
                "Crico(test)".to_owned(),
                95,
                true,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Elstripper(test)".to_owned(),
                "Crico(test)".to_owned(),
                50,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Jotaro(test)".to_owned(),
                "Crico(test)".to_owned(),
                65,
                true,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Cago(test)".to_owned(),
                "Kawasaki(test)".to_owned(),
                30,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Elstripper(test)".to_owned(),
                "Kawasaki(test)".to_owned(),
                75,
                true,
            )?;

            data_service.create_game(
                GameType::Standard,
                "Jotaro(test)".to_owned(),
                "Kawasaki(test)".to_owned(),
                50,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Elstripper(test)".to_owned(),
                "Cago(test)".to_owned(),
                65,
                true,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Jotaro(test)".to_owned(),
                "Cago(test)".to_owned(),
                30,
                false,
            )?;
            data_service.create_game(
                GameType::Standard,
                "Jotaro(test)".to_owned(),
                "Elstripper(test)".to_owned(),
                75,
                true,
            )?;
            Ok(())
        })();
        match result {
            Ok(_) => println!("Games created."),
            Err(message) => eprintln!("{}", message),
        }
        // data_service.print_games();
        // data_service.print_games_user("Cago(test)".to_owned());
        // data_service.print_games_user("Crico(test)".to_owned());
        // data_service.print_games_user("Jotaro(test)".to_owned());
        // data_service.print_games_user("Kawasaki(test)".to_owned());
        // data_service.print_games_user("Elstripper(test)".to_owned());
        self.test_runned = true;
    }

    fn show_info(&self) {
        println!("{}", "\n/test".bright_yellow());
        println!(
            "{}",
            "#generate test players and games, once per session".bright_green()
        );
    }

    fn name(&self) -> &str {
        "/test"
    }
}
