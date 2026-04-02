#[cfg(test)]
mod tests_phase_1c {
    use crate::state::game_state::{GameState, GameMode};
    use crate::state::player::Direction;
    use crate::GameEngine;

    /// Helper: perform a complete movement step in the given direction.
    /// Calls tick until movement animation completes (or max 30 frames).
    fn complete_step(engine: &mut GameEngine, dir: &str) {
        engine.tick(16.67, dir);
        for _ in 0..30 {
            if !engine.player_moving() {
                break;
            }
            engine.tick(16.67, "none");
        }
    }

    fn dir_str(d: u8) -> &'static str {
        match d {
            1 => "up",
            2 => "down",
            3 => "left",
            4 => "right",
            _ => "none",
        }
    }

    // ── GameState initialization ──────────────────────────────────────────────

    #[test]
    fn new_game_state_mode_is_overworld() {
        let gs = GameState::new();
        assert_eq!(gs.mode, GameMode::Overworld);
    }

    #[test]
    fn player_starts_at_expected_position() {
        let gs = GameState::new();
        assert_eq!(gs.player.x, 3);
        assert_eq!(gs.player.y, 3);
    }

    #[test]
    fn party_is_empty_initially() {
        let gs = GameState::new();
        assert!(gs.player.party.is_empty());
    }

    #[test]
    fn money_is_zero_initially() {
        let gs = GameState::new();
        assert_eq!(gs.player.money, 0);
    }

    #[test]
    fn no_stamps_earned_initially() {
        let gs = GameState::new();
        assert!(!gs.authentication_stamps.iter().any(|&s| s));
    }

    #[test]
    fn story_progress_is_zero() {
        let gs = GameState::new();
        assert_eq!(gs.story_progress, 0);
    }

    // ── Event flags ───────────────────────────────────────────────────────────

    #[test]
    fn set_flag_then_has_flag_returns_true() {
        let mut gs = GameState::new();
        gs.set_flag("INTRO_DONE");
        assert!(gs.has_flag("INTRO_DONE"));
    }

    #[test]
    fn has_flag_on_unset_returns_false() {
        let gs = GameState::new();
        assert!(!gs.has_flag("NEVER_SET"));
    }

    #[test]
    fn clear_flag_removes_the_flag() {
        let mut gs = GameState::new();
        gs.set_flag("TO_REMOVE");
        gs.clear_flag("TO_REMOVE");
        assert!(!gs.has_flag("TO_REMOVE"));
    }

    // ── Direction ─────────────────────────────────────────────────────────────

    #[test]
    fn up_delta_is_0_neg1() {
        assert_eq!(Direction::Up.delta(), (0, -1));
    }

    #[test]
    fn down_delta_is_0_pos1() {
        assert_eq!(Direction::Down.delta(), (0, 1));
    }

    #[test]
    fn left_delta_is_neg1_0() {
        assert_eq!(Direction::Left.delta(), (-1, 0));
    }

    #[test]
    fn right_delta_is_pos1_0() {
        assert_eq!(Direction::Right.delta(), (1, 0));
    }

    // ── SeededRng integration ─────────────────────────────────────────────────

    #[test]
    fn same_seed_produces_same_encounter_sequence() {
        let mut e1 = GameEngine::new(42);
        let mut e2 = GameEngine::new(42);

        // Navigate to tall grass: up to (3,2), right×6 to (9,2) [tall grass], then step back/forth
        let dirs: &[u8] = &[1, 4, 4, 4, 4, 4, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3];
        for &d in dirs {
            complete_step(&mut e1, dir_str(d));
            complete_step(&mut e2, dir_str(d));
            assert_eq!(
                e1.encounter_triggered(),
                e2.encounter_triggered(),
                "encounter mismatch at direction {}",
                d
            );
        }
    }

    #[test]
    fn different_seed_produces_different_sequence() {
        let mut e1 = GameEngine::new(1);
        let mut e2 = GameEngine::new(999999);

        // Walk to tall grass: up then right×6 → (9,2)
        let setup: &[u8] = &[1, 4, 4, 4, 4, 4, 4];
        for &d in setup {
            complete_step(&mut e1, dir_str(d));
            complete_step(&mut e2, dir_str(d));
        }

        // Step back and forth in tall grass 40 times
        let mut any_diff = false;
        for i in 0..40 {
            let d = if i % 2 == 0 { 3u8 } else { 4u8 }; // alternate left/right
            complete_step(&mut e1, dir_str(d));
            complete_step(&mut e2, dir_str(d));
            if e1.encounter_triggered() != e2.encounter_triggered() {
                any_diff = true;
                break;
            }
        }
        assert!(
            any_diff,
            "different seeds produced identical encounter sequence for 40 grass steps"
        );
    }

    // ── Movement via new string API ───────────────────────────────────────────

    #[test]
    fn tick_string_input_moves_player() {
        let mut engine = GameEngine::new(0);
        // Start at (3,3), complete a full step right to (4,3)
        complete_step(&mut engine, "right");
        assert_eq!(engine.player_x(), 4);
        assert_eq!(engine.player_y(), 3);
    }

    #[test]
    fn player_x_y_correct_after_movement() {
        let mut engine = GameEngine::new(0);
        // Start at (3,3), move right to (4,3)
        complete_step(&mut engine, "right");
        assert_eq!(engine.player_x(), 4);
        assert_eq!(engine.player_y(), 3);
        // Move down to (4,4)
        complete_step(&mut engine, "down");
        assert_eq!(engine.player_x(), 4);
        assert_eq!(engine.player_y(), 4);
        // Move down to (4,5)
        complete_step(&mut engine, "down");
        assert_eq!(engine.player_x(), 4);
        assert_eq!(engine.player_y(), 5);
    }

    #[test]
    fn game_engine_new_with_seed_returns_valid_engine() {
        let engine = GameEngine::new(12345);
        assert_eq!(engine.player_x(), 3);
        assert_eq!(engine.player_y(), 3);
        assert_eq!(engine.map_width(), 20);
        assert_eq!(engine.map_height(), 15);
    }
}
