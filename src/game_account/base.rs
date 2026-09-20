pub struct BaseGameAccount {
    user_name: String,
    current_rating: i32,
    games_history: Vec<i32>,
}

impl BaseGameAccount {
    pub fn new(user_name: String, initial_rating: i32) -> Self {
        Self {
            user_name,
            current_rating: initial_rating.max(1),
            games_history: Vec::new(),
        }
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn set_user_name(&mut self, value: String) {
        self.user_name = value;
    }

    pub fn current_rating(&self) -> i32 {
        self.current_rating
    }

    pub fn set_current_rating(&mut self, value: i32) {
        self.current_rating = value.max(1);
    }

    pub fn games_count(&self) -> usize {
        self.games_history.len()
    }

    pub fn games_history(&self) -> &[i32] {
        &self.games_history
    }

    pub fn games_history_mut(&mut self) -> &mut Vec<i32> {
        &mut self.games_history
    }
}
