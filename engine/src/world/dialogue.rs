use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogueData {
    pub id: String,
    pub pages: Vec<DialoguePage>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialoguePage {
    pub speaker: Option<String>,
    pub text: String,
    pub choices: Option<Vec<DialogueChoice>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogueChoice {
    pub text: String,
    pub next_dialogue: Option<String>,
    pub set_flag: Option<String>,
    pub action: Option<String>,
}

/// Runtime state for an active dialogue session
#[derive(Clone, Debug)]
pub struct DialogueState {
    pub active_dialogue_id: String,
    pub current_page: usize,
    pub waiting_for_choice: bool,
    pub pages: Vec<DialoguePage>,
}

impl DialogueState {
    pub fn new(data: DialogueData) -> Self {
        Self {
            active_dialogue_id: data.id,
            current_page: 0,
            waiting_for_choice: false,
            pages: data.pages,
        }
    }

    pub fn current(&self) -> Option<&DialoguePage> {
        self.pages.get(self.current_page)
    }

    /// Advance to the next page. Returns true if there is a next page.
    pub fn advance(&mut self) -> bool {
        if self.current_page + 1 < self.pages.len() {
            self.current_page += 1;
            true
        } else {
            false
        }
    }
}

/// Replace template variables in dialogue text.
/// `{player_name}` → player name, `{rival_name}` → "Flip"
pub fn replace_template_vars(text: &str, player_name: &str) -> String {
    text.replace("{player_name}", player_name)
        .replace("{rival_name}", "Flip")
}

#[cfg(test)]
mod tests_dialogue {
    use super::*;

    #[test]
    fn replace_player_name() {
        let result = replace_template_vars("Hello {player_name}!", "Ace");
        assert_eq!(result, "Hello Ace!");
    }

    #[test]
    fn replace_rival_name() {
        let result = replace_template_vars("Your rival is {rival_name}.", "Player");
        assert_eq!(result, "Your rival is Flip.");
    }

    #[test]
    fn replace_both_template_vars() {
        let result = replace_template_vars("{player_name} vs {rival_name}", "Jordan");
        assert_eq!(result, "Jordan vs Flip");
    }
}
