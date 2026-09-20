use super::{BaseGameAccount, GameAccount};

pub struct StandardAccount {
    base: BaseGameAccount,
}

impl StandardAccount {
    pub fn new(user_name: String, initial_rating: Option<i32>) -> Self {
        Self {
            base: BaseGameAccount::new(user_name, initial_rating.unwrap_or(100)),
        }
    }
}

impl GameAccount for StandardAccount {
    fn user_name(&self) -> &str {
        self.base.user_name()
    }

    fn set_user_name(&mut self, value: String) {
        self.base.set_user_name(value)
    }

    fn current_rating(&self) -> i32 {
        self.base.current_rating()
    }

    fn set_current_rating(&mut self, value: i32) {
        self.base.set_current_rating(value)
    }

    fn games_count(&self) -> usize {
        self.base.games_count()
    }

    fn games_history(&self) -> &[i32] {
        self.base.games_history()
    }
    fn games_history_mut(&mut self) -> &mut Vec<i32> {
        self.base.games_history_mut()
    }

    fn game_rating(&mut self, result: bool, rating: i32) {
        let new_rating = if result {
            self.current_rating() + rating
        } else {
            self.current_rating() - rating
        };

        self.set_current_rating(new_rating);
    }
}
