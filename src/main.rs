use std::{cell::RefCell, rc::Rc};

use lab4::{
    db::{DataService, DbContext, GameAccountRepository, GameRepository},
    terminal::TerminalCommandProcessor,
};

fn main() {
    let db_context = Rc::new(RefCell::new(DbContext::new()));
    let game_repository = Box::new(GameRepository::new(db_context.clone()));
    let game_account_repository = Box::new(GameAccountRepository::new(db_context));
    let data_service = Rc::new(RefCell::new(DataService::new(
        game_repository,
        game_account_repository,
    )));
    let mut program = TerminalCommandProcessor::new(data_service);
    program.process_commands();
}
