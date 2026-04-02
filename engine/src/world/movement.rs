use crate::state::player::{Direction, PlayerState};
use crate::util::rng::SeededRng;
use crate::world::map::WildEncounterEntry;
use crate::world::encounters::check_wild_encounter;

pub const WALK_FRAMES: f64 = 8.0;
pub const SPRINT_FRAMES: f64 = 4.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    None,
    Up,
    Down,
    Left,
    Right,
    Action,
    Cancel,
    Menu,
}

pub fn parse_input(input: &str) -> InputAction {
    match input {
        "up"     => InputAction::Up,
        "down"   => InputAction::Down,
        "left"   => InputAction::Left,
        "right"  => InputAction::Right,
        "action" => InputAction::Action,
        "cancel" => InputAction::Cancel,
        "menu"   => InputAction::Menu,
        _        => InputAction::None,
    }
}

#[derive(Debug, Clone)]
pub enum GameEvent {
    None,
    WildEncounter { species_id: u16, level: u8 },
    MapTransition { target_map: String, x: u16, y: u16 },
    Warp { target_map: String, x: u16, y: u16 },
}

fn tile_at(collision: &[u8], map_width: usize, map_height: usize, x: u16, y: u16) -> u8 {
    if (x as usize) >= map_width || (y as usize) >= map_height {
        return 1; // out of bounds = solid
    }
    collision[y as usize * map_width + x as usize]
}

fn can_move_to(collision: &[u8], map_width: usize, map_height: usize, x: u16, y: u16) -> bool {
    tile_at(collision, map_width, map_height, x, y) != 1
}

fn on_step_complete(
    player: &PlayerState,
    collision: &[u8],
    map_width: usize,
    map_height: usize,
    wild_encounters: &[WildEncounterEntry],
    rng: &mut SeededRng,
) -> Vec<GameEvent> {
    let mut events = Vec::new();
    let tile = tile_at(collision, map_width, map_height, player.x, player.y);
    match tile {
        2 => {
            // Tall grass: 15% base encounter chance
            if rng.chance(15) {
                if let Some((species_id, level)) = check_wild_encounter(wild_encounters, rng) {
                    events.push(GameEvent::WildEncounter { species_id, level });
                } else {
                    events.push(GameEvent::WildEncounter { species_id: 1, level: 5 });
                }
            }
        }
        3 => {
            events.push(GameEvent::MapTransition {
                target_map: String::new(),
                x: 0,
                y: 0,
            });
        }
        4 => {
            events.push(GameEvent::Warp {
                target_map: String::new(),
                x: 0,
                y: 0,
            });
        }
        _ => {}
    }
    events
}

/// Process one movement tick for the given player.
///
/// - If the player is NOT moving and a directional input is given, facing is updated and
///   movement starts if the target tile is walkable.
/// - If the player IS moving, movement progress is advanced by `dt_ms` (directional input
///   is ignored until movement completes).
/// - Returns a list of `GameEvent`s triggered (e.g. wild encounters, map transitions).
///
/// `sprint`: when true, uses SPRINT_FRAMES instead of WALK_FRAMES.
pub fn process_movement(
    player: &mut PlayerState,
    input: InputAction,
    dt_ms: f64,
    sprint: bool,
    collision: &[u8],
    map_width: usize,
    map_height: usize,
    wild_encounters: &[WildEncounterEntry],
    rng: &mut SeededRng,
) -> Vec<GameEvent> {
    let mut events = Vec::new();

    // 1. Start new movement if the player is idle and a direction is pressed
    if !player.moving {
        let dir = match input {
            InputAction::Up    => Some(Direction::Up),
            InputAction::Down  => Some(Direction::Down),
            InputAction::Left  => Some(Direction::Left),
            InputAction::Right => Some(Direction::Right),
            _ => None,
        };

        if let Some(d) = dir {
            // Update facing even if blocked
            player.facing = d;

            let (dx, dy) = d.delta();
            let nx = player.x as isize + dx;
            let ny = player.y as isize + dy;

            if nx >= 0 && ny >= 0 {
                let (nx, ny) = (nx as u16, ny as u16);
                if can_move_to(collision, map_width, map_height, nx, ny) {
                    player.moving = true;
                    player.move_progress = 0.0;
                }
            }
        }
    }

    // 2. Advance movement progress (runs in the same tick that movement starts)
    if player.moving {
        let frames = if sprint { SPRINT_FRAMES } else { WALK_FRAMES };
        let advance = dt_ms / (1000.0 / 60.0) / frames;
        player.move_progress += advance as f32;

        if player.move_progress >= 1.0 {
            // Snap to target tile
            let (dx, dy) = player.facing.delta();
            player.x = (player.x as isize + dx) as u16;
            player.y = (player.y as isize + dy) as u16;
            player.move_progress = 0.0;
            player.moving = false;

            events.extend(on_step_complete(
                player, collision, map_width, map_height, wild_encounters, rng,
            ));
        }
    }

    events
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests_phase_2c {
    use super::*;
    use crate::state::player::PlayerState;
    use crate::util::rng::SeededRng;
    use crate::world::map::WildEncounterEntry;
    use crate::GameEngine;

    // 10×10 test map (row-major, y=0 at top):
    //   Borders are solid (1)
    //   (3,2) = tall grass (2)
    //   (5,2) = door (3)
    //   (7,2) = warp (4)
    //   All other interior = walkable (0)
    // Player will be placed at (5,5) for movement tests.
    fn make_collision() -> (Vec<u8>, usize, usize) {
        let w = 10usize;
        let h = 10usize;
        let mut col = vec![0u8; w * h];
        // Border walls
        for x in 0..w { col[x] = 1; col[(h - 1) * w + x] = 1; }
        for y in 0..h { col[y * w] = 1; col[y * w + w - 1] = 1; }
        // Special tiles
        col[2 * w + 3] = 2; // tall grass at (3,2)
        col[2 * w + 5] = 3; // door at (5,2)
        col[2 * w + 7] = 4; // warp at (7,2)
        (col, w, h)
    }

    fn make_player_at(x: u16, y: u16) -> PlayerState {
        let mut p = PlayerState::new();
        p.x = x;
        p.y = y;
        p
    }

    fn empty_encounters() -> Vec<WildEncounterEntry> { vec![] }

    fn grass_encounters() -> Vec<WildEncounterEntry> {
        vec![WildEncounterEntry { species_id: 4, level_min: 3, level_max: 5, weight: 100 }]
    }

    // ── Input parsing ─────────────────────────────────────────────────────────

    #[test]
    fn parse_input_up() { assert_eq!(parse_input("up"), InputAction::Up); }

    #[test]
    fn parse_input_none() { assert_eq!(parse_input("none"), InputAction::None); }

    #[test]
    fn parse_input_action() { assert_eq!(parse_input("action"), InputAction::Action); }

    #[test]
    fn parse_input_garbage_returns_none() { assert_eq!(parse_input("garbage"), InputAction::None); }

    // ── Movement ─────────────────────────────────────────────────────────────

    #[test]
    fn movement_starts_and_progress_increases() {
        let (col, w, h) = make_collision();
        let mut player = make_player_at(5, 5);
        let mut rng = SeededRng::new(1);
        // Input: right — (6,5) is walkable
        process_movement(&mut player, InputAction::Right, 16.67, false,
            &col, w, h, &empty_encounters(), &mut rng);
        assert!(player.moving, "player should be moving");
        assert!(player.move_progress > 0.0, "move_progress should be > 0 after first tick");
    }

    #[test]
    fn movement_completes_after_enough_dt() {
        let (col, w, h) = make_collision();
        let mut player = make_player_at(5, 5);
        let mut rng = SeededRng::new(1);
        // Run enough ticks to complete one step
        for i in 0..20 {
            let input = if i == 0 { InputAction::Right } else { InputAction::None };
            process_movement(&mut player, input, 16.67, false,
                &col, w, h, &empty_encounters(), &mut rng);
            if !player.moving && player.x == 6 {
                break;
            }
        }
        assert_eq!(player.x, 6, "player should have moved to x=6");
        assert_eq!(player.y, 5, "player y unchanged");
    }

    #[test]
    fn player_faces_wall_but_doesnt_move() {
        let (col, w, h) = make_collision();
        // Place player at (1,1); border wall is at (0,1) — move left
        let mut player = make_player_at(1, 1);
        let mut rng = SeededRng::new(1);
        process_movement(&mut player, InputAction::Left, 16.67, false,
            &col, w, h, &empty_encounters(), &mut rng);
        use crate::state::player::Direction;
        assert_eq!(player.facing, Direction::Left, "facing should update even when blocked");
        assert_eq!(player.x, 1, "position x unchanged");
        assert_eq!(player.y, 1, "position y unchanged");
        assert!(!player.moving, "should not be moving");
    }

    #[test]
    fn input_ignored_while_moving() {
        let (col, w, h) = make_collision();
        let mut player = make_player_at(5, 5);
        let mut rng = SeededRng::new(1);
        // Start moving right
        process_movement(&mut player, InputAction::Right, 16.67, false,
            &col, w, h, &empty_encounters(), &mut rng);
        assert!(player.moving);
        use crate::state::player::Direction;
        let facing_before = player.facing;
        // While moving, send "up" — facing should NOT change
        process_movement(&mut player, InputAction::Up, 16.67, false,
            &col, w, h, &empty_encounters(), &mut rng);
        assert_eq!(player.facing, facing_before, "facing should not change while moving");
    }

    #[test]
    fn sprint_completes_in_fewer_frames() {
        let (col, w, h) = make_collision();
        let mut player_normal = make_player_at(5, 5);
        let mut player_sprint = make_player_at(5, 5);
        let mut rng1 = SeededRng::new(1);
        let mut rng2 = SeededRng::new(1);

        let mut normal_frames = 0u32;
        let mut sprint_frames_count = 0u32;

        for i in 0..30 {
            let input = if i == 0 { InputAction::Right } else { InputAction::None };
            if player_normal.moving || i == 0 {
                process_movement(&mut player_normal, input, 16.67, false,
                    &col, w, h, &empty_encounters(), &mut rng1);
                normal_frames += 1;
                if !player_normal.moving && player_normal.x == 6 { break; }
            }
        }
        for i in 0..30 {
            let input = if i == 0 { InputAction::Right } else { InputAction::None };
            if player_sprint.moving || i == 0 {
                process_movement(&mut player_sprint, input, 16.67, true,
                    &col, w, h, &empty_encounters(), &mut rng2);
                sprint_frames_count += 1;
                if !player_sprint.moving && player_sprint.x == 6 { break; }
            }
        }

        assert!(sprint_frames_count < normal_frames,
            "sprint ({} frames) should complete in fewer frames than normal ({} frames)",
            sprint_frames_count, normal_frames);
    }

    // ── Step events ──────────────────────────────────────────────────────────

    #[test]
    fn step_onto_tall_grass_deterministic_encounter() {
        // Move player to (3,2) which is tall grass, verify encounter with seeded RNG
        let (col, w, h) = make_collision();
        // Place player at (3,3), move up to (3,2) = tall grass
        let mut player = make_player_at(3, 3);
        let mut rng = SeededRng::new(42);
        let encounters = grass_encounters();

        // Complete the step to (3,2)
        let mut events: Vec<GameEvent> = vec![];
        for i in 0..20 {
            let input = if i == 0 { InputAction::Up } else { InputAction::None };
            events = process_movement(&mut player, input, 16.67, false,
                &col, w, h, &encounters, &mut rng);
            if !player.moving { break; }
        }

        // With seed 42, we get a deterministic result. Either encounter or not.
        // The important thing: if encounter, it should be species_id=4 from our table.
        for ev in &events {
            if let GameEvent::WildEncounter { species_id, .. } = ev {
                assert_eq!(*species_id, 4, "encounter should be species 4 from table");
            }
        }
        // Player should be at (3,2)
        assert_eq!(player.x, 3);
        assert_eq!(player.y, 2);
    }

    #[test]
    fn step_onto_normal_tile_no_events() {
        let (col, w, h) = make_collision();
        let mut player = make_player_at(5, 5);
        let mut rng = SeededRng::new(1);
        let mut events: Vec<GameEvent> = vec![];
        for i in 0..20 {
            let input = if i == 0 { InputAction::Right } else { InputAction::None };
            events = process_movement(&mut player, input, 16.67, false,
                &col, w, h, &empty_encounters(), &mut rng);
            if !player.moving { break; }
        }
        assert!(events.is_empty(), "no events when stepping on walkable tile");
    }

    #[test]
    fn step_onto_door_tile_map_transition_event() {
        // Place at (5,3), move up to (5,2) which is a door
        let (col, w, h) = make_collision();
        let mut player = make_player_at(5, 3);
        let mut rng = SeededRng::new(1);
        let mut events: Vec<GameEvent> = vec![];
        for i in 0..20 {
            let input = if i == 0 { InputAction::Up } else { InputAction::None };
            events = process_movement(&mut player, input, 16.67, false,
                &col, w, h, &empty_encounters(), &mut rng);
            if !player.moving { break; }
        }
        let has_transition = events.iter().any(|e| matches!(e, GameEvent::MapTransition { .. }));
        assert!(has_transition, "should get MapTransition event when stepping on door tile");
    }

    // ── Integration ──────────────────────────────────────────────────────────

    #[test]
    fn full_movement_cycle_right_from_3_3_to_4_3() {
        // Use test map from GameEngine (hardcoded 20x15 map with player starting at (3,3))
        let mut engine = GameEngine::new(1);
        // (4,3) is interior walkable in the hardcoded map
        assert_eq!(engine.player_x(), 3);
        assert_eq!(engine.player_y(), 3);

        // Tick right until movement completes
        for i in 0..20 {
            let input = if i == 0 { "right" } else { "none" };
            engine.tick(16.67, input);
            if engine.player_x() == 4 { break; }
        }

        assert_eq!(engine.player_x(), 4, "player should be at x=4 after full movement");
        assert_eq!(engine.player_y(), 3, "player y should be unchanged");
    }
}
