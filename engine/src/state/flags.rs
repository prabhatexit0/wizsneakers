use crate::state::game_state::GameState;

impl GameState {
    pub fn set_flag(&mut self, flag: &str) {
        self.event_flags.insert(flag.to_string());
    }

    pub fn has_flag(&self, flag: &str) -> bool {
        self.event_flags.contains(flag)
    }

    pub fn clear_flag(&mut self, flag: &str) {
        self.event_flags.remove(flag);
    }
}
