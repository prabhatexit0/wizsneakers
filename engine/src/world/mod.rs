pub mod map;
pub mod encounters;
pub mod movement;

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
