pub mod sneakers;
pub mod moves;
pub mod items;
pub mod trainers;

use crate::models::items::ItemData;
use crate::models::moves::MoveData;
use crate::models::sneaker::SneakerSpecies;

/// Look up a sneaker species by ID (1-30). Panics on invalid ID.
pub fn get_species(id: u16) -> &'static SneakerSpecies {
    match id {
        1 => &sneakers::RETRO_RUNNER,
        2 => &sneakers::RETRO_RUNNER_II,
        3 => &sneakers::RETRO_RUNNER_MAX,
        4 => &sneakers::CLASSIC_DUNK,
        5 => &sneakers::OG_FORCE,
        6 => &sneakers::VINTAGE_HIGH_TOP,
        7 => &sneakers::HERITAGE_COURT,
        8 => &sneakers::GENESIS_JORDAN,
        9 => &sneakers::TECH_TRAINER,
        10 => &sneakers::TECH_TRAINER_PRO,
        11 => &sneakers::TECH_TRAINER_ULTRA,
        12 => &sneakers::FOAM_CELL,
        13 => &sneakers::BOOST_CORE,
        14 => &sneakers::LED_LACE,
        15 => &sneakers::QUANTUM_SOLE,
        16 => &sneakers::GENESIS_REACT,
        17 => &sneakers::SKATE_BLAZER,
        18 => &sneakers::SKATE_BLAZER_PRO,
        19 => &sneakers::SKATE_BLAZER_ELITE,
        20 => &sneakers::GRIP_TAPE,
        21 => &sneakers::HALF_PIPE,
        22 => &sneakers::VULCANIZED,
        23 => &sneakers::BOARD_DESTROYER,
        24 => &sneakers::GENESIS_KICKFLIP,
        25 => &sneakers::RUNWAY_SLIP,
        26 => &sneakers::COUTURE_BOOT,
        27 => &sneakers::AVANT_GARDE,
        28 => &sneakers::MAISON_SOLE,
        29 => &sneakers::TRIPLE_BLACK,
        30 => &sneakers::GENESIS_COUTURE,
        _ => panic!("Unknown species id: {}", id),
    }
}

/// Look up a move by ID (1-48). Panics on invalid ID.
pub fn get_move(id: u16) -> &'static MoveData {
    match id {
        1 => &moves::LACE_UP,
        2 => &moves::FLEX,
        3 => &moves::CAMP_OUT,
        4 => &moves::QUICK_STEP,
        5 => &moves::STOMP,
        6 => &moves::DOUBLE_UP,
        7 => &moves::DEADSTOCK_STRIKE,
        8 => &moves::HYPE_TRAIN,
        9 => &moves::RESELL,
        10 => &moves::AUTHENTICATE,
        11 => &moves::CREASE,
        12 => &moves::THROWBACK,
        13 => &moves::VINTAGE_SLAM,
        14 => &moves::HERITAGE_CRUSH,
        15 => &moves::RETRO_WAVE,
        16 => &moves::CLASSIC_AURA,
        17 => &moves::OG_STAMP,
        18 => &moves::GRAIL_BEAM,
        19 => &moves::FIRMWARE_UPDATE,
        20 => &moves::SHOCK_DROP,
        21 => &moves::BLUETOOTH_BLAST,
        22 => &moves::DATA_MINE,
        23 => &moves::NEON_PULSE,
        24 => &moves::OVERCLOCK,
        25 => &moves::SYSTEM_CRASH,
        26 => &moves::QUANTUM_LEAP,
        27 => &moves::KICKFLIP,
        28 => &moves::ANKLE_BREAKER,
        29 => &moves::GRIND_RAIL,
        30 => &moves::BOARD_SLIDE,
        31 => &moves::TRE_FLIP,
        32 => &moves::SKATERS_RESOLVE,
        33 => &moves::VULC_SMASH,
        34 => &moves::NINE_HUNDRED_SPIN,
        35 => &moves::RUNWAY_STRIKE,
        36 => &moves::LABEL_DROP,
        37 => &moves::HAUTE_BEAM,
        38 => &moves::PRICE_TAG,
        39 => &moves::RED_CARPET,
        40 => &moves::FASHION_POLICE,
        41 => &moves::COUTURE_CANNON,
        42 => &moves::LIMITED_EDITION,
        43 => &moves::VINYL_SCRATCH,
        44 => &moves::DEBUG_PROTOCOL,
        45 => &moves::FIFTY_FIFTY_GRIND,
        46 => &moves::VOGUE_STRIKE,
        47 => &moves::RESELL_MARKUP,
        48 => &moves::GENESIS_AURA,
        _ => panic!("Unknown move id: {}", id),
    }
}

/// Look up an item by ID. Panics on invalid ID.
pub fn get_item(id: u16) -> &'static ItemData {
    match id {
        1 => &items::SOLE_SAUCE,
        2 => &items::INSOLE_PAD,
        3 => &items::FULL_RESTORE_SPRAY,
        4 => &items::MAX_REVIVE_LACE,
        5 => &items::REVIVAL_THREAD,
        6 => &items::CREASE_GUARD,
        7 => &items::BUFF_SPRAY,
        8 => &items::SMELLING_SALTS,
        9 => &items::PUMP,
        10 => &items::FULL_CLEANSE,
        11 => &items::PP_RESTORE,
        12 => &items::PP_MAX,
        20 => &items::HYPE_POTION,
        21 => &items::DRIP_POTION,
        22 => &items::GUARD_SPRAY,
        23 => &items::SPEED_LACE,
        24 => &items::X_ALL,
        25 => &items::CRIT_LENS,
        26 => &items::FOCUS_SASH,
        30 => &items::SNEAKER_CASE,
        31 => &items::PREMIUM_CASE,
        32 => &items::GRAIL_CASE,
        33 => &items::MASTER_CASE,
        34 => &items::RETRO_CASE,
        35 => &items::TECH_CASE,
        36 => &items::SKATE_CASE,
        37 => &items::FASHION_CASE,
        50 => &items::SNEAKERDEX,
        51 => &items::TOWN_MAP,
        52 => &items::ESCAPE_ROPE,
        53 => &items::REPEL,
        54 => &items::SUPER_REPEL,
        55 => &items::AUTHENTICATION_STAMP,
        56 => &items::SYNDICATE_JOURNAL,
        57 => &items::TEMPLE_KEY,
        58 => &items::ELEVATOR_PASS,
        59 => &items::BICYCLE,
        70 => &items::HERITAGE_SOLE,
        71 => &items::NANO_FIBER_SOLE,
        72 => &items::SKATE_SOLE,
        73 => &items::SILK_INSOLE,
        74 => &items::SNACK_PACK,
        75 => &items::FOCUS_BAND,
        76 => &items::QUICK_LACE,
        77 => &items::WIDE_LENS,
        78 => &items::MUSCLE_BAND,
        79 => &items::WISE_GLASSES,
        80 => &items::CHOICE_BAND,
        81 => &items::CHOICE_SPECS,
        82 => &items::CHOICE_LACE,
        83 => &items::EV_BAND_HYPE,
        84 => &items::EV_BAND_ALL,
        _ => panic!("Unknown item id: {}", id),
    }
}

#[cfg(test)]
mod tests_phase_1b {
    use super::*;
    use crate::models::items::ItemEffect;
    use crate::models::moves::MoveCategory;
    use crate::models::sneaker::RarityTier;

    // ── Sneaker data integrity ────────────────────────────────────────────────

    #[test]
    fn all_species_have_unique_ids() {
        let mut ids = std::collections::HashSet::new();
        for s in sneakers::ALL_SPECIES {
            assert!(ids.insert(s.id), "Duplicate species id: {}", s.id);
        }
        assert_eq!(ids.len(), 30);
    }

    #[test]
    fn species_ids_are_1_to_30() {
        for id in 1u16..=30 {
            assert_eq!(get_species(id).id, id);
        }
    }

    #[test]
    fn all_species_have_nonzero_base_stats() {
        for s in sneakers::ALL_SPECIES {
            let st = &s.base_stats;
            assert!(st.durability > 0, "{} has zero durability", s.name);
            assert!(st.hype > 0, "{} has zero hype", s.name);
            assert!(st.comfort > 0, "{} has zero comfort", s.name);
            assert!(st.drip > 0, "{} has zero drip", s.name);
            assert!(st.rarity > 0, "{} has zero rarity", s.name);
        }
    }

    #[test]
    fn starter_species_have_bst_215() {
        for &id in &[1u16, 9, 17] {
            let s = get_species(id);
            let bst = s.base_stats.durability
                + s.base_stats.hype
                + s.base_stats.comfort
                + s.base_stats.drip
                + s.base_stats.rarity;
            assert_eq!(bst, 215, "Starter {} (id={}) has BST {}, expected 215", s.name, id, bst);
        }
    }

    #[test]
    fn legendary_species_bst_at_least_480() {
        for s in sneakers::ALL_SPECIES {
            if s.rarity_tier == RarityTier::Legendary {
                let bst = s.base_stats.durability
                    + s.base_stats.hype
                    + s.base_stats.comfort
                    + s.base_stats.drip
                    + s.base_stats.rarity;
                assert!(
                    bst >= 480,
                    "Legendary {} has BST {} < 480",
                    s.name,
                    bst
                );
            }
        }
    }

    #[test]
    fn evolution_targets_are_valid_ids() {
        for s in sneakers::ALL_SPECIES {
            if let Some((_, target_id)) = s.evolution {
                assert!(
                    target_id >= 1 && target_id <= 30,
                    "{} evolves into invalid species id {}",
                    s.name,
                    target_id
                );
            }
        }
    }

    #[test]
    fn learnsets_sorted_by_level_ascending() {
        for s in sneakers::ALL_SPECIES {
            let levels: Vec<u8> = s.learnset.iter().map(|(lv, _)| *lv).collect();
            let mut sorted = levels.clone();
            sorted.sort();
            assert_eq!(levels, sorted, "{} learnset not sorted by level", s.name);
        }
    }

    // ── Move data integrity ───────────────────────────────────────────────────

    #[test]
    fn all_moves_have_unique_ids() {
        let mut ids = std::collections::HashSet::new();
        for m in moves::ALL_MOVES {
            assert!(ids.insert(m.id), "Duplicate move id: {}", m.id);
        }
        assert_eq!(ids.len(), 48);
    }

    #[test]
    fn move_ids_are_1_to_48() {
        for id in 1u16..=48 {
            assert_eq!(get_move(id).id, id);
        }
    }

    #[test]
    fn physical_special_moves_have_power_status_moves_have_none() {
        for m in moves::ALL_MOVES {
            match m.category {
                MoveCategory::Physical | MoveCategory::Special => {
                    // Resell Markup (47) is Special with no power (deals % current HP)
                    if m.id == 47 {
                        assert!(
                            m.power.is_none(),
                            "Move {} should have None power",
                            m.name
                        );
                    } else {
                        assert!(
                            m.power.is_some(),
                            "Physical/Special move {} must have Some(power)",
                            m.name
                        );
                    }
                }
                MoveCategory::Status => {
                    assert!(m.power.is_none(), "Status move {} must have None power", m.name);
                }
            }
        }
    }

    #[test]
    fn all_accuracy_values_are_1_to_100() {
        for m in moves::ALL_MOVES {
            assert!(
                m.accuracy >= 1 && m.accuracy <= 100,
                "Move {} has accuracy {} outside 1-100",
                m.name,
                m.accuracy
            );
        }
    }

    #[test]
    fn all_pp_values_are_positive() {
        for m in moves::ALL_MOVES {
            assert!(m.pp > 0, "Move {} has pp=0", m.name);
        }
    }

    #[test]
    fn quick_step_has_priority_plus_1() {
        let m = get_move(4);
        assert_eq!(m.name, "Quick Step");
        assert_eq!(m.priority, 1);
    }

    #[test]
    fn all_other_moves_have_priority_0() {
        for m in moves::ALL_MOVES {
            if m.id != 4 {
                assert_eq!(m.priority, 0, "Move {} (id={}) has unexpected priority {}", m.name, m.id, m.priority);
            }
        }
    }

    // ── Item data integrity ───────────────────────────────────────────────────

    #[test]
    fn sneaker_case_catch_multipliers() {
        // Basic = 100 (1.0x)
        let basic = get_item(30);
        assert_eq!(basic.name, "Sneaker Case");
        assert_eq!(basic.effect, ItemEffect::CatchMultiplier(100));

        // Premium = 150 (1.5x)
        let premium = get_item(31);
        assert_eq!(premium.name, "Premium Case");
        assert_eq!(premium.effect, ItemEffect::CatchMultiplier(150));

        // Grail = 250 (2.5x)
        let grail = get_item(32);
        assert_eq!(grail.name, "Grail Case");
        assert_eq!(grail.effect, ItemEffect::CatchMultiplier(250));

        // Master = guaranteed catch
        let master = get_item(33);
        assert_eq!(master.name, "Master Case");
        assert_eq!(master.effect, ItemEffect::GuaranteedCatch);
    }

    #[test]
    fn heal_items_have_positive_hp_values() {
        for &id in &[1u16, 2] {
            let item = get_item(id);
            match item.effect {
                ItemEffect::HealHp(hp) => assert!(hp > 0, "Item {} heals 0 HP", item.name),
                _ => panic!("Item {} should be HealHp", item.name),
            }
        }
    }

    #[test]
    fn all_items_have_category() {
        // Spot-check a few items across categories
        use crate::models::items::ItemCategory;
        assert_eq!(get_item(1).category, ItemCategory::HealItem);
        assert_eq!(get_item(20).category, ItemCategory::BattleItem);
        assert_eq!(get_item(30).category, ItemCategory::SneakerCase);
        assert_eq!(get_item(50).category, ItemCategory::KeyItem);
        assert_eq!(get_item(70).category, ItemCategory::HeldItem);
    }

    // ── Data lookups ──────────────────────────────────────────────────────────

    #[test]
    fn get_species_1_returns_retro_runner() {
        assert_eq!(get_species(1).name, "Retro Runner");
    }

    #[test]
    fn get_move_1_returns_lace_up() {
        assert_eq!(get_move(1).name, "Lace Up");
    }

    #[test]
    fn get_item_1_returns_sole_sauce() {
        assert_eq!(get_item(1).name, "Sole Sauce");
    }

    #[test]
    #[should_panic(expected = "Unknown species id: 999")]
    fn get_species_999_panics() {
        get_species(999);
    }
}
