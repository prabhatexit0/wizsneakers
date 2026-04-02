pub mod map;
pub mod encounters;
pub mod movement;
pub mod dialogue;
pub mod npc;
pub mod events;

#[cfg(test)]
mod tests_phase_2a {
    use crate::world::map::{MapData, MapConnections, WildEncounterEntry, TileType};
    use crate::world::encounters::{check_wild_encounter, generate_wild_sneaker};
    use crate::util::rng::SeededRng;
    use crate::GameEngine;

    // ── Minimal test map JSON ──────────────────────────────────────────────────
    //
    // 5×5 layout (row-major, y=0 at top):
    //   Row 0: 1 1 1 1 1  (border)
    //   Row 1: 1 0 0 2 1  (x=3,y=1 is tall grass)
    //   Row 2: 1 0 0 0 1
    //   Row 3: 1 0 0 0 1  (player starts at x=3,y=3)
    //   Row 4: 1 1 1 1 1  (border)
    //
    // Wall test: from (3,3) move right → (4,3) is solid
    // Grass test: from (3,3) move up to (3,2) then up to (3,1) = tall grass
    const TEST_MAP_JSON: &str = r#"{
        "id": "test_map",
        "name": "Test Map",
        "width": 5,
        "height": 5,
        "collision": [1,1,1,1,1, 1,0,0,2,1, 1,0,0,0,1, 1,0,0,0,1, 1,1,1,1,1],
        "ground":    [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        "overlay":   [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        "connections": {"north": null, "south": null, "east": null, "west": null},
        "wild_encounters": [
            {"species_id": 4, "level_min": 3, "level_max": 5, "weight": 60},
            {"species_id": 20, "level_min": 3, "level_max": 5, "weight": 40}
        ],
        "npcs": [],
        "events": [],
        "music": "test_bgm"
    }"#;

    // Helper: tick enough frames to complete one movement step
    fn complete_step(engine: &mut GameEngine, dir: &str) {
        engine.tick(16.67, dir);
        for _ in 0..20 {
            if !engine.player_moving() {
                break;
            }
            engine.tick(16.67, "none");
        }
    }

    // ── Map parsing ────────────────────────────────────────────────────────────

    #[test]
    fn parse_valid_json_gives_correct_dimensions() {
        let map = MapData::from_json(TEST_MAP_JSON).expect("Should parse");
        assert_eq!(map.width, 5);
        assert_eq!(map.height, 5);
        assert_eq!(map.id, "test_map");
        assert_eq!(map.collision.len(), 25);
    }

    #[test]
    fn parse_invalid_json_returns_error() {
        let result = MapData::from_json("this is not json");
        assert!(result.is_err());
    }

    #[test]
    fn parse_missing_field_returns_error() {
        // Missing required "width" field
        let bad = r#"{"id":"x","name":"x","height":5,"collision":[],"ground":[],"overlay":[],
            "connections":{"north":null,"south":null,"east":null,"west":null},
            "wild_encounters":[],"npcs":[],"events":[],"music":""}"#;
        let result = MapData::from_json(bad);
        assert!(result.is_err());
    }

    // ── Collision ──────────────────────────────────────────────────────────────

    #[test]
    fn is_walkable_on_walkable_tile_returns_true() {
        let map = MapData::from_json(TEST_MAP_JSON).unwrap();
        // (1,1) is 0 = walkable
        assert!(map.is_walkable(1, 1));
    }

    #[test]
    fn is_walkable_on_solid_tile_returns_false() {
        let map = MapData::from_json(TEST_MAP_JSON).unwrap();
        // (0,0) is 1 = solid (border)
        assert!(!map.is_walkable(0, 0));
    }

    #[test]
    fn is_walkable_out_of_bounds_returns_false() {
        let map = MapData::from_json(TEST_MAP_JSON).unwrap();
        assert!(!map.is_walkable(100, 100));
        assert!(!map.is_walkable(5, 0));
        assert!(!map.is_walkable(0, 5));
    }

    #[test]
    fn tile_type_at_tall_grass_returns_tall_grass() {
        let map = MapData::from_json(TEST_MAP_JSON).unwrap();
        // (3,1) = 2 = tall grass
        assert_eq!(map.tile_type_at(3, 1), TileType::TallGrass);
    }

    // ── Encounters ─────────────────────────────────────────────────────────────

    #[test]
    fn check_wild_encounter_with_nonempty_table_always_returns_some() {
        let table = vec![
            WildEncounterEntry { species_id: 4, level_min: 3, level_max: 5, weight: 100 },
        ];
        let mut rng = SeededRng::new(42);
        // Try 50 times — should always return Some since table is non-empty
        for _ in 0..50 {
            let result = check_wild_encounter(&table, &mut rng);
            assert!(result.is_some());
        }
    }

    #[test]
    fn check_wild_encounter_species_and_levels_within_range() {
        let table = vec![
            WildEncounterEntry { species_id: 4,  level_min: 3, level_max: 5, weight: 50 },
            WildEncounterEntry { species_id: 20, level_min: 6, level_max: 8, weight: 50 },
        ];
        let mut rng = SeededRng::new(1);
        for _ in 0..200 {
            let (species, level) = check_wild_encounter(&table, &mut rng).unwrap();
            assert!(species == 4 || species == 20);
            if species == 4 {
                assert!(level >= 3 && level <= 5, "level {} out of range for species 4", level);
            } else {
                assert!(level >= 6 && level <= 8, "level {} out of range for species 20", level);
            }
        }
    }

    #[test]
    fn weighted_selection_most_common_entry_appears_most() {
        // Weight 80:20 — species 4 should appear ~80% of the time
        let table = vec![
            WildEncounterEntry { species_id: 4,  level_min: 5, level_max: 5, weight: 80 },
            WildEncounterEntry { species_id: 20, level_min: 5, level_max: 5, weight: 20 },
        ];
        let mut rng = SeededRng::new(99);
        let mut count_4 = 0usize;
        let n = 1000;
        for _ in 0..n {
            let (species, _) = check_wild_encounter(&table, &mut rng).unwrap();
            if species == 4 { count_4 += 1; }
        }
        // Species 4 should win most — at least 60% (well below the 80% expected)
        assert!(count_4 > 600, "species 4 appeared {} times out of {}, expected > 600", count_4, n);
    }

    #[test]
    fn generate_wild_sneaker_has_ivs_in_0_to_31() {
        let mut rng = SeededRng::new(7);
        let inst = generate_wild_sneaker(4, 5, &mut rng);
        assert!(inst.ivs.durability <= 31);
        assert!(inst.ivs.hype <= 31);
        assert!(inst.ivs.comfort <= 31);
        assert!(inst.ivs.drip <= 31);
        assert!(inst.ivs.rarity <= 31);
    }

    #[test]
    fn generate_wild_sneaker_instance_is_valid() {
        let mut rng = SeededRng::new(42);
        let inst = generate_wild_sneaker(4, 5, &mut rng);
        assert_eq!(inst.species_id, 4);
        assert_eq!(inst.level, 5);
        assert!(inst.current_hp > 0);
        assert_eq!(inst.current_hp, inst.max_hp);
        assert!(inst.status.is_none());
    }

    #[test]
    fn generate_wild_sneaker_has_appropriate_moves_for_level() {
        // Classic Dunk (id=4) learnset: (1,5),(5,11),(9,2),(13,12)
        // At level 5: should know moves from levels 1 and 5 — move_ids 5 and 11
        let mut rng = SeededRng::new(1);
        let inst = generate_wild_sneaker(4, 5, &mut rng);

        let move_ids: Vec<u16> = inst.moves.iter()
            .filter_map(|s| s.as_ref().map(|slot| slot.move_id))
            .collect();

        assert!(!move_ids.is_empty(), "sneaker should have at least one move");

        // All moves must be from the learnset at or below level 5
        let species = crate::data::get_species(4);
        for &mid in &move_ids {
            let in_learnset = species.learnset.iter()
                .any(|&(lv, id)| id == mid && lv <= 5);
            assert!(in_learnset, "move_id {} is not in learnset at or below level 5", mid);
        }
    }

    // ── Integration ────────────────────────────────────────────────────────────

    #[test]
    fn load_map_move_into_wall_position_unchanged() {
        let mut engine = GameEngine::new(1);
        engine.load_map_from_json(TEST_MAP_JSON).expect("map should load");

        // Player starts at (3,3). Move right → (4,3) is solid border.
        let before_x = engine.player_x();
        let before_y = engine.player_y();
        engine.tick(16.67, "right");
        assert_eq!(engine.player_x(), before_x, "x should be unchanged after hitting wall");
        assert_eq!(engine.player_y(), before_y, "y should be unchanged after hitting wall");
    }

    #[test]
    fn load_map_move_into_tall_grass_encounter_can_trigger() {
        // Navigate to (3,1) which is tall grass, step back/forth until encounter triggers
        let mut engine = GameEngine::new(5);
        engine.load_map_from_json(TEST_MAP_JSON).expect("map should load");

        // From (3,3): up → (3,2), up → (3,1) [tall grass]
        complete_step(&mut engine, "up");
        complete_step(&mut engine, "up"); // now at (3,1) tall grass

        // Step left/right between (2,1) and (3,1) up to 200 full steps
        let mut triggered = false;
        for i in 0..200 {
            let d = if i % 2 == 0 { "left" } else { "right" };
            complete_step(&mut engine, d);
            if engine.encounter_triggered() {
                triggered = true;
                break;
            }
        }
        assert!(triggered, "encounter should trigger within 200 tall-grass steps");
    }
}

#[cfg(test)]
mod tests_phase_6 {
    use crate::world::dialogue::{DialogueData, DialoguePage, DialogueState, replace_template_vars};
    use crate::world::npc::{NpcState, TrainerNpcData, tick_npcs, check_trainer_triggers};
    use crate::world::map::{MapData, NpcMovement};
    use crate::state::player::Direction;
    use crate::util::rng::SeededRng;
    use crate::GameEngine;

    fn make_map_10x10() -> MapData {
        let mut collision = vec![0u8; 100];
        // Border walls
        for x in 0..10usize { collision[x] = 1; collision[90 + x] = 1; }
        for y in 0..10usize { collision[y * 10] = 1; collision[y * 10 + 9] = 1; }
        // Wall at (4,4)
        collision[4 * 10 + 4] = 1;

        let json = format!(r#"{{
            "id": "phase6_test",
            "name": "Phase 6 Test",
            "width": 10,
            "height": 10,
            "collision": {:?},
            "ground": [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            "overlay": [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            "connections": {{"north": null, "south": null, "east": null, "west": null}},
            "wild_encounters": [],
            "npcs": [],
            "events": [],
            "music": ""
        }}"#, collision);
        MapData::from_json(&json).unwrap()
    }

    fn make_npc(id: &str, x: u16, y: u16, dialogue_id: &str) -> NpcState {
        NpcState {
            id: id.to_string(),
            x, y,
            facing: Direction::Down,
            sprite: "npc".to_string(),
            movement: NpcMovement::Stationary,
            dialogue_id: dialogue_id.to_string(),
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

    // ── Dialogue tests ─────────────────────────────────────────────────────────

    #[test]
    fn template_replacement_player_name() {
        let result = replace_template_vars("Hi {player_name}!", "Red");
        assert_eq!(result, "Hi Red!");
    }

    #[test]
    fn multi_page_dialogue_advances_correctly() {
        let data = DialogueData {
            id: "test".to_string(),
            pages: vec![
                DialoguePage { speaker: None, text: "Page 1".to_string(), choices: None },
                DialoguePage { speaker: None, text: "Page 2".to_string(), choices: None },
                DialoguePage { speaker: None, text: "Page 3".to_string(), choices: None },
            ],
        };
        let mut state = DialogueState::new(data);
        assert_eq!(state.current().unwrap().text, "Page 1");
        assert!(state.advance());
        assert_eq!(state.current().unwrap().text, "Page 2");
        assert!(state.advance());
        assert_eq!(state.current().unwrap().text, "Page 3");
        // Should not advance past last page
        assert!(!state.advance());
        assert_eq!(state.current().unwrap().text, "Page 3");
    }

    #[test]
    fn choices_set_flags_via_engine() {
        let mut engine = GameEngine::new(42);
        // Load a dialogue with a choice that sets a flag
        let dialogue_json = r#"[{
            "id": "choice_test",
            "pages": [{
                "speaker": null,
                "text": "Choose!",
                "choices": [
                    {"text": "Yes", "next_dialogue": null, "set_flag": "said_yes", "action": null},
                    {"text": "No", "next_dialogue": null, "set_flag": null, "action": null}
                ]
            }]
        }]"#;
        engine.load_dialogue_json(dialogue_json).unwrap();

        // Set player position and facing to be next to NPC
        engine.state.player.x = 5;
        engine.state.player.y = 5;
        engine.state.player.facing = Direction::Down;

        // Manually put engine in dialogue mode with the choice dialogue
        let data = engine.dialogue_db.get("choice_test").cloned().unwrap();
        engine.dialogue_state = Some(crate::world::dialogue::DialogueState::new(data));
        engine.state.mode = crate::state::GameMode::Dialogue;

        // Select choice 0 (Yes) which sets "said_yes" flag
        engine.select_choice(0);
        assert!(
            engine.state.event_flags.contains("said_yes"),
            "selecting choice should set the associated flag"
        );
    }

    // ── Interaction tests ──────────────────────────────────────────────────────

    #[test]
    fn facing_npc_and_interact_starts_dialogue() {
        let mut engine = GameEngine::new(1);

        // Place player at (5,5) facing down, NPC at (5,6)
        engine.state.player.x = 5;
        engine.state.player.y = 5;
        engine.state.player.facing = Direction::Down;

        engine.npcs.push(make_npc("guard", 5, 6, "guard_dialogue"));

        // Load some dialogue for the NPC
        let dialogue_json = r#"[{"id": "guard_dialogue", "pages": [{"speaker": "Guard", "text": "Halt!", "choices": null}]}]"#;
        engine.load_dialogue_json(dialogue_json).unwrap();

        let result = engine.interact();
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["type"], "dialogue", "interact should return dialogue type when facing NPC");
        assert_eq!(engine.state.mode, crate::state::GameMode::Dialogue);
    }

    #[test]
    fn facing_wall_no_interaction() {
        let mut engine = GameEngine::new(1);
        // Default map has walls at the border
        // Player at (1,1) facing left → (0,1) is solid wall
        engine.state.player.x = 1;
        engine.state.player.y = 1;
        engine.state.player.facing = Direction::Left;

        let result = engine.interact();
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["type"], "none", "facing wall should return none");
    }

    #[test]
    fn facing_sign_returns_sign_text() {
        let mut engine = GameEngine::new(1);

        // Load a map with a sign event
        let map_json = r#"{
            "id": "sign_test",
            "name": "Sign Test",
            "width": 10,
            "height": 10,
            "collision": [1,1,1,1,1,1,1,1,1,1, 1,0,0,0,0,0,0,0,0,1, 1,0,0,0,0,0,0,0,0,1, 1,0,0,0,0,0,0,0,0,1, 1,0,0,0,0,0,0,0,0,1, 1,0,0,0,0,0,0,0,0,1, 1,0,0,0,0,0,0,0,0,1, 1,0,0,0,0,0,0,0,0,1, 1,0,0,0,0,0,0,0,0,1, 1,1,1,1,1,1,1,1,1,1],
            "ground": [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            "overlay": [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            "connections": {"north": null, "south": null, "east": null, "west": null},
            "wild_encounters": [],
            "npcs": [],
            "events": [{"id": "sign1", "x": 5, "y": 3, "event_type": "sign", "data": "Boxfresh Town Pop. 50"}],
            "music": ""
        }"#;
        engine.load_map_from_json(map_json).unwrap();

        // Player at (5,4) facing up toward sign at (5,3)
        engine.state.player.x = 5;
        engine.state.player.y = 4;
        engine.state.player.facing = Direction::Up;

        let result = engine.interact();
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["type"], "sign", "interact should return sign type when facing sign");
        assert_eq!(v["text"], "Boxfresh Town Pop. 50", "should return sign text");
    }

    // ── NPC movement tests ─────────────────────────────────────────────────────

    fn make_movement_npc(id: &str, x: u16, y: u16, facing: Direction, movement: NpcMovement) -> NpcState {
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
        let map = make_map_10x10();
        let mut npcs = vec![make_movement_npc("npc1", 5, 5, Direction::Down, NpcMovement::Stationary)];
        let mut rng = SeededRng::new(42);
        for _ in 0..200 {
            tick_npcs(&mut npcs, (3, 3), &map, 16.67, &mut rng);
        }
        assert_eq!(npcs[0].x, 5, "stationary NPC x should not change");
        assert_eq!(npcs[0].y, 5, "stationary NPC y should not change");
    }

    #[test]
    fn random_walk_npc_stays_within_radius() {
        let map = make_map_10x10();
        let radius = 2u8;
        let home_x = 5u16;
        let home_y = 5u16;
        let mut npc = make_movement_npc("npc1", home_x, home_y, Direction::Down,
            NpcMovement::RandomWalk { radius });
        npc.home_x = home_x;
        npc.home_y = home_y;
        let mut npcs = vec![npc];
        let mut rng = SeededRng::new(1);
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
        let map = make_map_10x10();
        let mut npcs = vec![make_movement_npc("npc1", 5, 5, Direction::Down, NpcMovement::FacePlayer)];
        let mut rng = SeededRng::new(1);
        tick_npcs(&mut npcs, (7, 5), &map, 16.67, &mut rng);
        assert_eq!(npcs[0].facing, Direction::Right, "should face right when player is to the right");
        tick_npcs(&mut npcs, (5, 2), &map, 16.67, &mut rng);
        assert_eq!(npcs[0].facing, Direction::Up, "should face up when player is above");
    }

    #[test]
    fn npc_collision_two_npcs_cannot_occupy_same_tile() {
        let map = make_map_10x10();
        let mut npc1 = make_movement_npc("npc1", 5, 5, Direction::Down, NpcMovement::RandomWalk { radius: 1 });
        npc1.home_x = 5; npc1.home_y = 5;
        let mut npc2 = make_movement_npc("npc2", 5, 6, Direction::Down, NpcMovement::RandomWalk { radius: 1 });
        npc2.home_x = 5; npc2.home_y = 6;
        let mut npcs = vec![npc1, npc2];
        let mut rng = SeededRng::new(99);
        for _ in 0..500 {
            tick_npcs(&mut npcs, (1, 1), &map, 16.67, &mut rng);
            if !npcs[0].moving && !npcs[1].moving {
                assert_ne!(
                    (npcs[0].x, npcs[0].y),
                    (npcs[1].x, npcs[1].y),
                    "two NPCs should not occupy the same tile"
                );
            }
        }
    }

    // ── Trainer line-of-sight tests ────────────────────────────────────────────

    fn make_trainer_npc(id: &str, x: u16, y: u16, facing: Direction, sight: u8) -> NpcState {
        NpcState {
            id: id.to_string(),
            x, y,
            facing,
            sprite: "trainer".to_string(),
            movement: NpcMovement::Stationary,
            dialogue_id: String::new(),
            is_trainer: true,
            trainer_data: Some(TrainerNpcData { trainer_id: 1, sight_range: sight }),
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
    fn trainer_facing_right_player_within_sight_detected() {
        let map = make_map_10x10();
        let npcs = vec![make_trainer_npc("t1", 3, 5, Direction::Right, 4)];
        let result = check_trainer_triggers(&npcs, (6, 5), &map);
        assert_eq!(result, Some("t1".to_string()), "trainer should spot player 3 tiles right");
    }

    #[test]
    fn trainer_facing_right_player_to_left_not_detected() {
        let map = make_map_10x10();
        let npcs = vec![make_trainer_npc("t1", 5, 5, Direction::Right, 4)];
        let result = check_trainer_triggers(&npcs, (2, 5), &map);
        assert!(result.is_none(), "trainer should not spot player behind them");
    }

    #[test]
    fn wall_between_trainer_and_player_blocks_los() {
        let map = make_map_10x10(); // Wall at (4,4)
        let npcs = vec![make_trainer_npc("t1", 3, 4, Direction::Right, 4)];
        // Player at (5,4) — wall at (4,4) blocks LOS from trainer (3,4)
        let result = check_trainer_triggers(&npcs, (5, 4), &map);
        assert!(result.is_none(), "wall should block trainer line-of-sight");
    }

    #[test]
    fn defeated_trainer_not_detected() {
        let map = make_map_10x10();
        let mut trainer = make_trainer_npc("t1", 3, 5, Direction::Right, 4);
        trainer.defeated = true;
        let npcs = vec![trainer];
        let result = check_trainer_triggers(&npcs, (6, 5), &map);
        assert!(result.is_none(), "defeated trainer should not spot player");
    }

    #[test]
    fn trainer_sight_range_respected() {
        let map = make_map_10x10();
        let sight = 3u8;
        let npcs = vec![make_trainer_npc("t1", 3, 5, Direction::Right, sight)];
        // Player at sight_range + 1 = 4 tiles away → NOT detected
        let player_x_too_far = 3 + sight as u16 + 1;
        let result = check_trainer_triggers(&npcs, (player_x_too_far, 5), &map);
        assert!(result.is_none(), "player at range+1 should not be detected");
        // Player at exactly sight_range = 3 tiles away → detected
        let player_x_at_range = 3 + sight as u16;
        let result2 = check_trainer_triggers(&npcs, (player_x_at_range, 5), &map);
        assert_eq!(result2, Some("t1".to_string()), "player at exact sight range should be detected");
    }
}
