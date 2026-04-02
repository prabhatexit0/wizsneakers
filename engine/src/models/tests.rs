#[cfg(test)]
mod tests_phase_1a {
    use crate::models::faction::Faction;
    use crate::models::stats::{Stats, StatKind, StatStages, Condition};
    use crate::models::moves::MoveSlot;
    use crate::models::sneaker::{SneakerInstance, SneakerSpecies, RarityTier};
    use crate::models::inventory::{Inventory, InventoryPocket, SneakerBox, SNEAKER_BOX_MAX};
    use crate::util::rng::SeededRng;

    // ── Type Effectiveness ────────────────────────────────────────────────────

    #[test]
    fn retro_vs_skate_is_2x() {
        assert_eq!(Faction::Retro.effectiveness_against(Faction::Skate), 2.0);
    }

    #[test]
    fn techwear_vs_retro_is_2x() {
        assert_eq!(Faction::Techwear.effectiveness_against(Faction::Retro), 2.0);
    }

    #[test]
    fn skate_vs_techwear_is_2x() {
        assert_eq!(Faction::Skate.effectiveness_against(Faction::Techwear), 2.0);
    }

    #[test]
    fn high_fashion_vs_high_fashion_is_0_5x() {
        assert_eq!(Faction::HighFashion.effectiveness_against(Faction::HighFashion), 0.5);
    }

    #[test]
    fn normal_vs_anything_is_1x() {
        assert_eq!(Faction::Normal.effectiveness_against(Faction::Skate), 1.0);
        assert_eq!(Faction::Normal.effectiveness_against(Faction::Retro), 1.0);
        assert_eq!(Faction::Normal.effectiveness_against(Faction::Techwear), 1.0);
        assert_eq!(Faction::Normal.effectiveness_against(Faction::HighFashion), 1.0);
        assert_eq!(Faction::Normal.effectiveness_against(Faction::Normal), 1.0);
    }

    #[test]
    fn retro_vs_retro_is_1x() {
        assert_eq!(Faction::Retro.effectiveness_against(Faction::Retro), 1.0);
    }

    // ── Stat Stages ──────────────────────────────────────────────────────────

    #[test]
    fn stage_multiplier_zero_is_1() {
        assert_eq!(StatStages::multiplier(0), 1.0);
    }

    #[test]
    fn stage_multiplier_plus1_is_1_5() {
        assert_eq!(StatStages::multiplier(1), 1.5);
    }

    #[test]
    fn stage_multiplier_minus1_approx_0_667() {
        let m = StatStages::multiplier(-1);
        assert!((m - 2.0 / 3.0).abs() < 1e-9, "expected ~0.667, got {}", m);
    }

    #[test]
    fn stage_multiplier_plus6_is_4() {
        assert_eq!(StatStages::multiplier(6), 4.0);
    }

    #[test]
    fn stage_multiplier_minus6_is_0_25() {
        assert_eq!(StatStages::multiplier(-6), 0.25);
    }

    #[test]
    fn stage_clamping_from_plus5_plus3_gives_plus6() {
        let mut stages = StatStages::default();
        stages.hype = 5;
        stages.modify(StatKind::Hype, 3);
        assert_eq!(stages.hype, 6);
    }

    // ── Conditions ───────────────────────────────────────────────────────────

    #[test]
    fn deadstock_hype_is_1_1_rarity_is_0_9() {
        assert_eq!(Condition::Deadstock.modifier(StatKind::Hype), 1.1);
        assert_eq!(Condition::Deadstock.modifier(StatKind::Rarity), 0.9);
        assert_eq!(Condition::Deadstock.modifier(StatKind::Comfort), 1.0);
    }

    #[test]
    fn general_release_all_1_0() {
        assert_eq!(Condition::GeneralRelease.modifier(StatKind::Hype), 1.0);
        assert_eq!(Condition::GeneralRelease.modifier(StatKind::Comfort), 1.0);
        assert_eq!(Condition::GeneralRelease.modifier(StatKind::Drip), 1.0);
        assert_eq!(Condition::GeneralRelease.modifier(StatKind::Rarity), 1.0);
        assert_eq!(Condition::GeneralRelease.modifier(StatKind::Durability), 1.0);
    }

    #[test]
    fn beat_comfort_1_1_drip_0_9() {
        assert_eq!(Condition::Beat.modifier(StatKind::Comfort), 1.1);
        assert_eq!(Condition::Beat.modifier(StatKind::Drip), 0.9);
        assert_eq!(Condition::Beat.modifier(StatKind::Hype), 1.0);
    }

    // ── Stat Calculation ─────────────────────────────────────────────────────

    fn make_species(base: u16) -> SneakerSpecies {
        SneakerSpecies {
            id: 1,
            name: "TestSneaker",
            faction: Faction::Normal,
            base_stats: Stats { durability: base, hype: base, comfort: base, drip: base, rarity: base },
            rarity_tier: RarityTier::Common,
            base_catch_rate: 100,
            base_xp_yield: 50,
            ev_yield: Stats::zero(),
            learnset: &[],
            evolution: None,
            description: "Test",
        }
    }

    fn make_instance(level: u8, condition: Condition) -> SneakerInstance {
        SneakerInstance {
            uid: 1,
            species_id: 1,
            nickname: None,
            level,
            xp: 0,
            current_hp: 100,
            max_hp: 100,
            ivs: Stats { durability: 15, hype: 15, comfort: 15, drip: 15, rarity: 15 },
            evs: Stats::zero(),
            condition,
            moves: [None, None, None, None],
            status: None,
            on_fire_turns: 0,
            held_item: None,
            friendship: 0,
            caught_location: 0,
            original_trainer: String::from("Red"),
        }
    }

    #[test]
    fn stat_calc_known_input() {
        // base=50, iv=15, ev=0, level=10, condition=GeneralRelease (mod=1.0)
        // stat = ((2*50 + 15 + 0) * 10 / 100 + 5) * 1.0 = (115*10/100 + 5) = (11 + 5) = 16
        let species = make_species(50);
        let instance = make_instance(10, Condition::GeneralRelease);
        let stat = instance.calc_stat(&species, StatKind::Hype);
        assert_eq!(stat, 16);
    }

    #[test]
    fn hp_formula_level_5() {
        // base=50, iv=15, ev=0, level=5
        // hp = (2*50+15+0)*5/100 + 5 + 10 = 115*5/100 + 15 = 5 + 15 = 20
        let species = make_species(50);
        let instance = make_instance(5, Condition::GeneralRelease);
        let hp = instance.calc_max_hp(&species);
        assert_eq!(hp, 20);
    }

    #[test]
    fn hp_formula_level_50() {
        // base=50, iv=15, ev=0, level=50
        // hp = (2*50+15+0)*50/100 + 50 + 10 = 115*50/100 + 60 = 57 + 60 = 117
        let species = make_species(50);
        let instance = make_instance(50, Condition::GeneralRelease);
        let hp = instance.calc_max_hp(&species);
        assert_eq!(hp, 117);
    }

    // ── RNG ──────────────────────────────────────────────────────────────────

    #[test]
    fn same_seed_produces_same_sequence() {
        let mut rng1 = SeededRng::new(42);
        let mut rng2 = SeededRng::new(42);
        for _ in 0..20 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn range_stays_within_bounds() {
        let mut rng = SeededRng::new(12345);
        for _ in 0..1000 {
            let v = rng.range(5, 15);
            assert!(v >= 5 && v < 15, "range out of bounds: {}", v);
        }
    }

    #[test]
    fn chance_100_always_true() {
        let mut rng = SeededRng::new(99);
        for _ in 0..100 {
            assert!(rng.chance(100));
        }
    }

    #[test]
    fn chance_0_always_false() {
        let mut rng = SeededRng::new(99);
        for _ in 0..100 {
            assert!(!rng.chance(0));
        }
    }

    // ── Inventory ────────────────────────────────────────────────────────────

    #[test]
    fn add_item_increments_qty() {
        let mut inv = Inventory::default();
        inv.add_item(1, 3, InventoryPocket::HealItems);
        assert_eq!(inv.item_count(1, InventoryPocket::HealItems), 3);
        inv.add_item(1, 2, InventoryPocket::HealItems);
        assert_eq!(inv.item_count(1, InventoryPocket::HealItems), 5);
    }

    #[test]
    fn remove_item_decrements_qty() {
        let mut inv = Inventory::default();
        inv.add_item(1, 5, InventoryPocket::HealItems);
        let ok = inv.remove_item(1, 2, InventoryPocket::HealItems);
        assert!(ok);
        assert_eq!(inv.item_count(1, InventoryPocket::HealItems), 3);
    }

    #[test]
    fn remove_item_returns_false_when_qty_zero() {
        let mut inv = Inventory::default();
        let ok = inv.remove_item(42, 1, InventoryPocket::HealItems);
        assert!(!ok);
    }

    #[test]
    fn sneaker_box_respects_50_cap() {
        let mut sbox = SneakerBox::default();
        for i in 0..SNEAKER_BOX_MAX {
            let sneaker = SneakerInstance {
                uid: i as u64,
                species_id: 1,
                nickname: None,
                level: 5,
                xp: 0,
                current_hp: 20,
                max_hp: 20,
                ivs: Stats::zero(),
                evs: Stats::zero(),
                condition: Condition::GeneralRelease,
                moves: [None, None, None, None],
                status: None,
                on_fire_turns: 0,
                held_item: None,
                friendship: 0,
                caught_location: 0,
                original_trainer: String::from("Test"),
            };
            let ok = sbox.deposit(sneaker);
            assert!(ok, "deposit failed at index {}", i);
        }
        assert!(sbox.is_full());
        assert_eq!(sbox.count(), SNEAKER_BOX_MAX);

        // 51st deposit should fail
        let extra = SneakerInstance {
            uid: 999,
            species_id: 1,
            nickname: None,
            level: 5,
            xp: 0,
            current_hp: 20,
            max_hp: 20,
            ivs: Stats::zero(),
            evs: Stats::zero(),
            condition: Condition::GeneralRelease,
            moves: [None, None, None, None],
            status: None,
            on_fire_turns: 0,
            held_item: None,
            friendship: 0,
            caught_location: 0,
            original_trainer: String::from("Test"),
        };
        assert!(!sbox.deposit(extra));
    }
}
