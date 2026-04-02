use serde::{Deserialize, Serialize};
use crate::state::player::Direction;
use crate::util::rng::SeededRng;
use crate::world::map::{MapData, NpcMovement};
use crate::world::movement::GameEvent;

#[derive(Clone, Debug)]
pub struct TrainerNpcData {
    pub trainer_id: u16,
    pub sight_range: u8,
}

#[derive(Clone, Debug)]
pub struct NpcState {
    pub id: String,
    pub x: u16,
    pub y: u16,
    pub facing: Direction,
    pub sprite: String,
    pub movement: NpcMovement,
    pub dialogue_id: String,
    pub is_trainer: bool,
    pub trainer_data: Option<TrainerNpcData>,
    pub defeated: bool,
    pub moving: bool,
    pub move_progress: f32,
    pub move_timer: f64,
    /// Home position for RandomWalk radius anchor
    pub home_x: u16,
    pub home_y: u16,
    /// For Patrol: current index in path
    pub patrol_index: usize,
}

impl NpcState {
    pub fn facing_str(&self) -> &'static str {
        match self.facing {
            Direction::Up    => "up",
            Direction::Down  => "down",
            Direction::Left  => "left",
            Direction::Right => "right",
        }
    }

    /// Parse a facing string into a Direction
    pub fn parse_facing(s: &str) -> Direction {
        match s {
            "up"    => Direction::Up,
            "down"  => Direction::Down,
            "left"  => Direction::Left,
            "right" => Direction::Right,
            _       => Direction::Down,
        }
    }
}

/// Move all NPCs one tick. Returns any events (e.g., TrainerSpotted).
pub fn tick_npcs(
    npcs: &mut Vec<NpcState>,
    player_pos: (u16, u16),
    map: &MapData,
    dt_ms: f64,
    rng: &mut SeededRng,
) -> Vec<GameEvent> {
    let events = Vec::new();

    // Build a set of occupied positions (all NPCs) for collision
    let occupied: Vec<(u16, u16)> = npcs.iter().map(|n| (n.x, n.y)).collect();

    for i in 0..npcs.len() {
        // Advance move animation
        if npcs[i].moving {
            let advance = dt_ms / (1000.0 / 60.0) / 8.0; // 8 frames per step
            npcs[i].move_progress += advance as f32;
            if npcs[i].move_progress >= 1.0 {
                // Snap to target tile
                let (dx, dy) = npcs[i].facing.delta();
                npcs[i].x = (npcs[i].x as isize + dx) as u16;
                npcs[i].y = (npcs[i].y as isize + dy) as u16;
                npcs[i].move_progress = 0.0;
                npcs[i].moving = false;
            }
            continue;
        }

        // FacePlayer: always face toward player
        if matches!(npcs[i].movement, NpcMovement::FacePlayer) {
            let px = player_pos.0 as isize;
            let py = player_pos.1 as isize;
            let nx = npcs[i].x as isize;
            let ny = npcs[i].y as isize;
            let dx = px - nx;
            let dy = py - ny;
            npcs[i].facing = if dx.abs() >= dy.abs() {
                if dx > 0 { Direction::Right } else { Direction::Left }
            } else {
                if dy > 0 { Direction::Down } else { Direction::Up }
            };
            continue;
        }

        // Stationary: never moves
        if matches!(npcs[i].movement, NpcMovement::Stationary) {
            continue;
        }

        // Advance move timer
        npcs[i].move_timer -= dt_ms;
        if npcs[i].move_timer > 0.0 {
            continue;
        }

        match npcs[i].movement.clone() {
            NpcMovement::RandomWalk { radius } => {
                // Pick a random direction
                let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
                let dir = directions[rng.next_u64() as usize % 4];
                let (ddx, ddy) = dir.delta();
                let nx = (npcs[i].x as isize + ddx) as u16;
                let ny = (npcs[i].y as isize + ddy) as u16;

                // Check within radius of home
                let home_dx = (nx as isize - npcs[i].home_x as isize).abs();
                let home_dy = (ny as isize - npcs[i].home_y as isize).abs();
                let in_radius = home_dx <= radius as isize && home_dy <= radius as isize;

                // Check walkable and not occupied by another NPC
                let tile_ok = map.is_walkable(nx, ny);
                let not_occupied = !occupied.iter().enumerate()
                    .any(|(j, &pos)| j != i && pos == (nx, ny));

                if in_radius && tile_ok && not_occupied {
                    npcs[i].facing = dir;
                    npcs[i].moving = true;
                    npcs[i].move_progress = 0.0;
                }

                // Reset timer: 2-5 seconds
                let wait_ms = 2000.0 + (rng.next_u64() % 3000) as f64;
                npcs[i].move_timer = wait_ms;
            }
            NpcMovement::Patrol { ref path } => {
                if path.is_empty() {
                    continue;
                }
                let next_idx = (npcs[i].patrol_index + 1) % path.len();
                let (tx, ty) = path[next_idx];

                // Move one step toward target
                let dx = tx as isize - npcs[i].x as isize;
                let dy = ty as isize - npcs[i].y as isize;
                let dir = if dx.abs() >= dy.abs() {
                    if dx > 0 { Direction::Right } else if dx < 0 { Direction::Left } else { Direction::Down }
                } else {
                    if dy > 0 { Direction::Down } else { Direction::Up }
                };

                let (ddx, ddy) = dir.delta();
                let nx = (npcs[i].x as isize + ddx) as u16;
                let ny = (npcs[i].y as isize + ddy) as u16;

                let not_occupied = !occupied.iter().enumerate()
                    .any(|(j, &pos)| j != i && pos == (nx, ny));

                if map.is_walkable(nx, ny) && not_occupied {
                    npcs[i].facing = dir;
                    npcs[i].moving = true;
                    npcs[i].move_progress = 0.0;

                    // If we reached the patrol point, advance index
                    if nx == tx && ny == ty {
                        npcs[i].patrol_index = next_idx;
                    }
                }

                // Short timer for patrol (move every ~0.5s)
                npcs[i].move_timer = 500.0;
            }
            _ => {}
        }
    }

    events
}

/// Check if any non-defeated trainer has line-of-sight to the player.
/// Returns the NPC id of the first trainer that spots the player.
pub fn check_trainer_triggers(
    npcs: &[NpcState],
    player_pos: (u16, u16),
    map: &MapData,
) -> Option<String> {
    for npc in npcs {
        if !npc.is_trainer || npc.defeated {
            continue;
        }
        let trainer_data = match &npc.trainer_data {
            Some(td) => td,
            None => continue,
        };

        let sight = trainer_data.sight_range as isize;
        let (fdx, fdy) = npc.facing.delta();
        let (tx, ty) = (npc.x as isize, npc.y as isize);
        let (px, py) = (player_pos.0 as isize, player_pos.1 as isize);

        // Check each tile in facing direction up to sight_range
        for dist in 1..=sight {
            let cx = tx + fdx * dist;
            let cy = ty + fdy * dist;

            // Check bounds
            if cx < 0 || cy < 0 || cx >= map.width as isize || cy >= map.height as isize {
                break;
            }

            // Check if this tile is a wall (LOS blocked)
            if !map.is_walkable(cx as u16, cy as u16) {
                break; // Wall blocks LOS
            }

            // Check if player is at this tile
            if cx == px && cy == py {
                return Some(npc.id.clone());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests_npc {
    use super::*;
    use crate::world::map::{MapData, NpcMovement};

    fn make_test_map() -> MapData {
        let json = r#"{
            "id": "npc_test",
            "name": "NPC Test",
            "width": 10,
            "height": 10,
            "collision": [
                1,1,1,1,1,1,1,1,1,1,
                1,0,0,0,0,0,0,0,0,1,
                1,0,0,0,0,0,0,0,0,1,
                1,0,0,0,0,0,0,0,0,1,
                1,0,0,0,1,0,0,0,0,1,
                1,0,0,0,0,0,0,0,0,1,
                1,0,0,0,0,0,0,0,0,1,
                1,0,0,0,0,0,0,0,0,1,
                1,0,0,0,0,0,0,0,0,1,
                1,1,1,1,1,1,1,1,1,1
            ],
            "ground": [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0],
            "overlay": [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0],
            "connections": {"north": null, "south": null, "east": null, "west": null},
            "wild_encounters": [],
            "npcs": [],
            "events": [],
            "music": ""
        }"#;
        MapData::from_json(json).unwrap()
    }

    fn make_npc(id: &str, x: u16, y: u16, facing: Direction, movement: NpcMovement) -> NpcState {
        NpcState {
            id: id.to_string(),
            x, y,
            facing,
            sprite: "npc".to_string(),
            movement,
            dialogue_id: String::new(),
            is_trainer: false,
            trainer_data: None,
            defeated: false,
            moving: false,
            move_progress: 0.0,
            move_timer: 0.0,
            home_x: x,
            home_y: y,
            patrol_index: 0,
        }
    }

    #[test]
    fn stationary_npc_never_moves() {
        let map = make_test_map();
        let mut npcs = vec![make_npc("npc1", 5, 5, Direction::Down, NpcMovement::Stationary)];
        let mut rng = SeededRng::new(42);

        for _ in 0..100 {
            tick_npcs(&mut npcs, (3, 3), &map, 16.67, &mut rng);
        }

        assert_eq!(npcs[0].x, 5, "stationary NPC x should not change");
        assert_eq!(npcs[0].y, 5, "stationary NPC y should not change");
    }

    #[test]
    fn random_walk_npc_stays_within_radius() {
        let map = make_test_map();
        let radius = 2u8;
        let home_x = 5u16;
        let home_y = 5u16;
        let mut npc = make_npc("npc1", home_x, home_y, Direction::Down,
            NpcMovement::RandomWalk { radius });
        npc.home_x = home_x;
        npc.home_y = home_y;
        let mut npcs = vec![npc];
        let mut rng = SeededRng::new(1);

        // Run many ticks
        for _ in 0..2000 {
            tick_npcs(&mut npcs, (1, 1), &map, 16.67, &mut rng);
        }

        let dx = (npcs[0].x as isize - home_x as isize).abs();
        let dy = (npcs[0].y as isize - home_y as isize).abs();
        assert!(dx <= radius as isize, "NPC x {} exceeds home {} by more than radius {}", npcs[0].x, home_x, radius);
        assert!(dy <= radius as isize, "NPC y {} exceeds home {} by more than radius {}", npcs[0].y, home_y, radius);
    }

    #[test]
    fn face_player_npc_faces_player_direction() {
        let map = make_test_map();
        let mut npcs = vec![make_npc("npc1", 5, 5, Direction::Down, NpcMovement::FacePlayer)];
        let mut rng = SeededRng::new(1);

        // Player to the right of NPC
        tick_npcs(&mut npcs, (7, 5), &map, 16.67, &mut rng);
        assert_eq!(npcs[0].facing, Direction::Right, "should face right when player is to the right");

        // Player above NPC
        tick_npcs(&mut npcs, (5, 2), &map, 16.67, &mut rng);
        assert_eq!(npcs[0].facing, Direction::Up, "should face up when player is above");

        // Player below NPC
        tick_npcs(&mut npcs, (5, 8), &map, 16.67, &mut rng);
        assert_eq!(npcs[0].facing, Direction::Down, "should face down when player is below");
    }

    #[test]
    fn two_npcs_cannot_occupy_same_tile() {
        let map = make_test_map();
        // Both NPCs at adjacent positions, RandomWalk with radius 1
        let mut npc1 = make_npc("npc1", 5, 5, Direction::Down, NpcMovement::RandomWalk { radius: 1 });
        npc1.home_x = 5; npc1.home_y = 5;
        let mut npc2 = make_npc("npc2", 5, 6, Direction::Down, NpcMovement::RandomWalk { radius: 1 });
        npc2.home_x = 5; npc2.home_y = 6;
        let mut npcs = vec![npc1, npc2];
        let mut rng = SeededRng::new(99);

        for _ in 0..500 {
            tick_npcs(&mut npcs, (1, 1), &map, 16.67, &mut rng);
            // Check they never occupy same tile mid-movement
            if !npcs[0].moving && !npcs[1].moving {
                assert_ne!(
                    (npcs[0].x, npcs[0].y),
                    (npcs[1].x, npcs[1].y),
                    "two NPCs should not occupy the same tile"
                );
            }
        }
    }

    #[test]
    fn trainer_facing_right_player_within_sight_detected() {
        let map = make_test_map();
        let mut trainer = make_npc("t1", 3, 5, Direction::Right, NpcMovement::Stationary);
        trainer.is_trainer = true;
        trainer.trainer_data = Some(TrainerNpcData { trainer_id: 1, sight_range: 4 });
        let npcs = vec![trainer];

        // Player 3 tiles to the right of trainer
        let result = check_trainer_triggers(&npcs, (6, 5), &map);
        assert_eq!(result, Some("t1".to_string()), "trainer should spot player 3 tiles right");
    }

    #[test]
    fn trainer_facing_right_player_to_left_not_detected() {
        let map = make_test_map();
        let mut trainer = make_npc("t1", 5, 5, Direction::Right, NpcMovement::Stationary);
        trainer.is_trainer = true;
        trainer.trainer_data = Some(TrainerNpcData { trainer_id: 1, sight_range: 4 });
        let npcs = vec![trainer];

        // Player 3 tiles to the LEFT of trainer — not in facing direction
        let result = check_trainer_triggers(&npcs, (2, 5), &map);
        assert!(result.is_none(), "trainer should not spot player behind them");
    }

    #[test]
    fn wall_between_trainer_and_player_blocks_los() {
        let map = make_test_map(); // Wall at (4, 4)
        let mut trainer = make_npc("t1", 3, 4, Direction::Right, NpcMovement::Stationary);
        trainer.is_trainer = true;
        trainer.trainer_data = Some(TrainerNpcData { trainer_id: 1, sight_range: 4 });
        let npcs = vec![trainer];

        // Player at (5, 4) — wall at (4, 4) between trainer (3,4) and player (5,4)
        let result = check_trainer_triggers(&npcs, (5, 4), &map);
        assert!(result.is_none(), "wall should block trainer line-of-sight");
    }

    #[test]
    fn defeated_trainer_not_detected() {
        let map = make_test_map();
        let mut trainer = make_npc("t1", 3, 5, Direction::Right, NpcMovement::Stationary);
        trainer.is_trainer = true;
        trainer.defeated = true;
        trainer.trainer_data = Some(TrainerNpcData { trainer_id: 1, sight_range: 4 });
        let npcs = vec![trainer];

        let result = check_trainer_triggers(&npcs, (6, 5), &map);
        assert!(result.is_none(), "defeated trainer should not spot player");
    }

    #[test]
    fn trainer_sight_range_respected() {
        let map = make_test_map();
        let sight = 3u8;
        let mut trainer = make_npc("t1", 3, 5, Direction::Right, NpcMovement::Stationary);
        trainer.is_trainer = true;
        trainer.trainer_data = Some(TrainerNpcData { trainer_id: 1, sight_range: sight });
        let npcs = vec![trainer];

        // Player at sight_range + 1 = 4 tiles away → NOT detected
        let player_x = 3 + sight as u16 + 1;
        let result = check_trainer_triggers(&npcs, (player_x, 5), &map);
        assert!(result.is_none(), "player at range+1 should not be detected");

        // Player at exactly sight_range = 3 tiles away → detected
        let player_x_at_range = 3 + sight as u16;
        let result2 = check_trainer_triggers(&npcs, (player_x_at_range, 5), &map);
        assert_eq!(result2, Some("t1".to_string()), "player at exact sight range should be detected");
    }
}
