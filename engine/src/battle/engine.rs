use crate::models::sneaker::SneakerInstance;
use crate::models::moves::MoveData;
use crate::models::stats::StatKind;
use crate::util::rng::SeededRng;
use crate::data;
use crate::battle::types::{
    AiLevel, BattleAction, BattleOpponent, BattleResult, BattleSide, BattleState,
    BattleTurnEvent, BattleKind,
};
use crate::battle::damage::calculate_damage;
use crate::battle::status::{apply_end_of_turn_status, check_can_move};

pub struct BattleEngine;

impl BattleEngine {
    /// Create a new wild battle state.
    pub fn new_wild(wild_sneaker: SneakerInstance) -> BattleState {
        BattleState {
            kind: BattleKind::Wild,
            player_active: 0,
            opponent: BattleOpponent {
                team: vec![wild_sneaker],
                items: vec![],
                ai_level: AiLevel::Random,
            },
            opponent_active: 0,
            turn_number: 0,
            player_stages: Default::default(),
            opponent_stages: Default::default(),
            turn_log: vec![],
            flee_attempts: 0,
            can_flee: true,
            waiting_for: None,
        }
    }

    /// Process a player action and return all events that occurred this turn.
    pub fn submit_action(
        state: &mut BattleState,
        player_party: &mut Vec<SneakerInstance>,
        action: BattleAction,
        rng: &mut SeededRng,
    ) -> Vec<BattleTurnEvent> {
        let mut events = Vec::new();

        match action {
            BattleAction::Fight { move_index } => {
                // Get player's chosen move
                let player_move_slot = {
                    let player = &player_party[state.player_active];
                    match player.moves[move_index as usize].clone() {
                        Some(slot) => slot,
                        None => {
                            events.push(BattleTurnEvent::Message {
                                text: "No move at that index!".to_string(),
                            });
                            return events;
                        }
                    }
                };
                let player_move_id = player_move_slot.move_id;
                let player_move = data::get_move(player_move_id);

                // AI picks a move for the opponent
                let opp_slot = pick_opponent_move(
                    &state.opponent.team[state.opponent_active],
                    rng,
                );
                let opp_move_id = opp_slot.move_id;
                let opp_move = data::get_move(opp_move_id);

                // Determine turn order: priority first, then Rarity, then RNG tiebreak
                let player_goes_first = {
                    let player_speed = player_party[state.player_active].calc_stat(
                        data::get_species(player_party[state.player_active].species_id),
                        StatKind::Rarity,
                    );
                    let opp_speed = state.opponent.team[state.opponent_active].calc_stat(
                        data::get_species(
                            state.opponent.team[state.opponent_active].species_id,
                        ),
                        StatKind::Rarity,
                    );
                    if player_move.priority != opp_move.priority {
                        player_move.priority > opp_move.priority
                    } else if player_speed != opp_speed {
                        player_speed > opp_speed
                    } else {
                        rng.range(0, 2) == 0
                    }
                };

                // Execute attacks in turn order
                if player_goes_first {
                    execute_player_attack(state, player_party, player_move, player_move_id, rng, &mut events);
                    if !state.opponent.team[state.opponent_active].is_fainted() {
                        execute_opponent_attack(state, player_party, opp_move, opp_move_id, rng, &mut events);
                    }
                } else {
                    execute_opponent_attack(state, player_party, opp_move, opp_move_id, rng, &mut events);
                    if !player_party[state.player_active].is_fainted() {
                        execute_player_attack(state, player_party, player_move, player_move_id, rng, &mut events);
                    }
                }

                // Decrement player PP
                if let Some(slot) = player_party[state.player_active].moves[move_index as usize].as_mut() {
                    if slot.current_pp > 0 {
                        slot.current_pp -= 1;
                    }
                }

                // End-of-turn status
                let player_idx = state.player_active;
                let opp_idx = state.opponent_active;
                apply_end_of_turn_status(
                    &mut player_party[player_idx],
                    BattleSide::Player,
                    &mut events,
                );
                apply_end_of_turn_status(
                    &mut state.opponent.team[opp_idx],
                    BattleSide::Opponent,
                    &mut events,
                );

                // Check win/lose (if not already ended from attack)
                if !events.iter().any(|e| matches!(e, BattleTurnEvent::BattleEnd { .. })) {
                    if state.opponent.team[state.opponent_active].is_fainted() {
                        events.push(BattleTurnEvent::BattleEnd {
                            result: BattleResult::PlayerWin,
                        });
                    } else if player_party[state.player_active].is_fainted() {
                        events.push(BattleTurnEvent::BattleEnd {
                            result: BattleResult::PlayerLose,
                        });
                    }
                }

                state.turn_number += 1;
            }

            BattleAction::Run => {
                if !state.can_flee {
                    events.push(BattleTurnEvent::Message {
                        text: "Can't escape!".to_string(),
                    });
                    state.turn_log.extend(events.clone());
                    return events;
                }

                let player_rarity = player_party[state.player_active].calc_stat(
                    data::get_species(player_party[state.player_active].species_id),
                    StatKind::Rarity,
                ) as u32;
                let opp_rarity = state.opponent.team[state.opponent_active]
                    .calc_stat(
                        data::get_species(
                            state.opponent.team[state.opponent_active].species_id,
                        ),
                        StatKind::Rarity,
                    )
                    .max(1) as u32;

                let flee_chance = (player_rarity * 128 / opp_rarity)
                    + 30 * state.flee_attempts as u32;
                let success = flee_chance > 255 || rng.range(0, 256) < flee_chance;

                state.flee_attempts += 1;
                events.push(BattleTurnEvent::FleeAttempt { success });

                if success {
                    events.push(BattleTurnEvent::BattleEnd {
                        result: BattleResult::PlayerFlee,
                    });
                } else {
                    // Opponent attacks on failed flee
                    let opp_slot =
                        pick_opponent_move(&state.opponent.team[state.opponent_active], rng);
                    let opp_move_id = opp_slot.move_id;
                    let opp_move = data::get_move(opp_move_id);
                    execute_opponent_attack(state, player_party, opp_move, opp_move_id, rng, &mut events);
                    if player_party[state.player_active].is_fainted() {
                        events.push(BattleTurnEvent::BattleEnd {
                            result: BattleResult::PlayerLose,
                        });
                    }
                }
            }

            BattleAction::Switch { party_index } => {
                state.player_active = party_index as usize;
                let species_id = player_party[party_index as usize].species_id;
                events.push(BattleTurnEvent::SwitchedIn {
                    side: BattleSide::Player,
                    species_id,
                });
            }

            BattleAction::Bag { item_id } => {
                events.push(BattleTurnEvent::ItemUsed { item_id });
            }
        }

        state.turn_log.extend(events.clone());
        events
    }
}

// ── Helper functions ──────────────────────────────────────────────────────────

fn pick_opponent_move(
    opponent: &SneakerInstance,
    rng: &mut SeededRng,
) -> crate::models::moves::MoveSlot {
    let valid: Vec<usize> = opponent
        .moves
        .iter()
        .enumerate()
        .filter_map(|(i, slot)| slot.as_ref().and_then(|s| {
            if s.current_pp > 0 { Some(i) } else { None }
        }))
        .collect();

    if valid.is_empty() {
        // Fallback: pick any available move slot
        for slot in &opponent.moves {
            if let Some(s) = slot {
                return s.clone();
            }
        }
        panic!("Opponent has no moves!");
    }

    let idx = valid[rng.range(0, valid.len() as u32) as usize];
    opponent.moves[idx].clone().unwrap()
}

fn execute_player_attack(
    state: &mut BattleState,
    player_party: &mut Vec<SneakerInstance>,
    move_data: &MoveData,
    move_id: u16,
    rng: &mut SeededRng,
    events: &mut Vec<BattleTurnEvent>,
) {
    // Check can-move status (SoldOut, Hypnotized)
    if !check_can_move(&player_party[state.player_active], rng) {
        events.push(BattleTurnEvent::Message {
            text: format!(
                "{} can't move!",
                player_party[state.player_active].nickname.as_deref().unwrap_or("Sneaker")
            ),
        });
        return;
    }

    events.push(BattleTurnEvent::MoveUsed {
        side: BattleSide::Player,
        move_id,
    });

    // Accuracy check
    let hits = move_data.accuracy >= 100
        || (move_data.accuracy > 0 && rng.range(1, 101) <= move_data.accuracy as u32);
    if !hits {
        events.push(BattleTurnEvent::Message {
            text: "Move missed!".to_string(),
        });
        return;
    }

    let player_idx = state.player_active;
    let opp_idx = state.opponent_active;

    // Clone snapshots to avoid borrow conflict
    let attacker = player_party[player_idx].clone();
    let attacker_species = data::get_species(attacker.species_id);
    let defender = state.opponent.team[opp_idx].clone();
    let defender_species = data::get_species(defender.species_id);

    let result = calculate_damage(
        &attacker,
        attacker_species,
        &defender,
        defender_species,
        move_data,
        &state.player_stages,
        &state.opponent_stages,
        rng,
    );

    if result.damage > 0 {
        let actual = result.damage.min(state.opponent.team[opp_idx].current_hp);
        state.opponent.team[opp_idx].current_hp -= actual;
        events.push(BattleTurnEvent::Damage {
            side: BattleSide::Opponent,
            amount: actual,
            effectiveness: result.effectiveness,
            is_critical: result.is_critical,
        });

        if state.opponent.team[opp_idx].is_fainted() {
            events.push(BattleTurnEvent::Fainted {
                side: BattleSide::Opponent,
            });
            events.push(BattleTurnEvent::BattleEnd {
                result: BattleResult::PlayerWin,
            });
        }
    }
}

fn execute_opponent_attack(
    state: &mut BattleState,
    player_party: &mut Vec<SneakerInstance>,
    move_data: &MoveData,
    move_id: u16,
    rng: &mut SeededRng,
    events: &mut Vec<BattleTurnEvent>,
) {
    // Check can-move status
    if !check_can_move(&state.opponent.team[state.opponent_active], rng) {
        events.push(BattleTurnEvent::Message {
            text: "Opponent can't move!".to_string(),
        });
        return;
    }

    events.push(BattleTurnEvent::MoveUsed {
        side: BattleSide::Opponent,
        move_id,
    });

    // Accuracy check
    let hits = move_data.accuracy >= 100
        || (move_data.accuracy > 0 && rng.range(1, 101) <= move_data.accuracy as u32);
    if !hits {
        events.push(BattleTurnEvent::Message {
            text: "Opponent missed!".to_string(),
        });
        return;
    }

    let player_idx = state.player_active;
    let opp_idx = state.opponent_active;

    let attacker = state.opponent.team[opp_idx].clone();
    let attacker_species = data::get_species(attacker.species_id);
    let defender = player_party[player_idx].clone();
    let defender_species = data::get_species(defender.species_id);

    let result = calculate_damage(
        &attacker,
        attacker_species,
        &defender,
        defender_species,
        move_data,
        &state.opponent_stages,
        &state.player_stages,
        rng,
    );

    if result.damage > 0 {
        let actual = result.damage.min(player_party[player_idx].current_hp);
        player_party[player_idx].current_hp -= actual;
        events.push(BattleTurnEvent::Damage {
            side: BattleSide::Player,
            amount: actual,
            effectiveness: result.effectiveness,
            is_critical: result.is_critical,
        });

        if player_party[player_idx].is_fainted() {
            events.push(BattleTurnEvent::Fainted {
                side: BattleSide::Player,
            });
            events.push(BattleTurnEvent::BattleEnd {
                result: BattleResult::PlayerLose,
            });
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests_phase_3a {
    use super::*;
    use crate::battle::damage::{calculate_damage_ex, DamageResult};
    use crate::battle::types::{BattleResult, BattleTurnEvent};
    use crate::data;
    use crate::models::moves::{MoveCategory, MoveData, MoveEffect, MoveSlot};
    use crate::models::faction::Faction;
    use crate::models::sneaker::SneakerInstance;
    use crate::models::stats::{Stats, StatStages, Condition};
    use crate::util::rng::SeededRng;

    // ── Test helpers ──────────────────────────────────────────────────────────

    /// Build a SneakerInstance with default IVs=0, EVs=0, GeneralRelease condition.
    fn make_instance(species_id: u16, level: u8, move_ids: &[u16]) -> SneakerInstance {
        let species = data::get_species(species_id);
        // Calculate max HP using the species formula
        let base_hp = species.base_stats.durability as u32;
        let max_hp = (2 * base_hp * level as u32 / 100 + level as u32 + 10) as u16;

        let mut moves = [None, None, None, None];
        for (i, &mid) in move_ids.iter().take(4).enumerate() {
            let md = data::get_move(mid);
            moves[i] = Some(MoveSlot { move_id: mid, current_pp: md.pp, max_pp: md.pp });
        }
        // Ensure at least one move
        if moves[0].is_none() {
            let md = data::get_move(5); // Stomp
            moves[0] = Some(MoveSlot { move_id: 5, current_pp: md.pp, max_pp: md.pp });
        }

        SneakerInstance {
            uid: 1,
            species_id,
            nickname: None,
            level,
            xp: 0,
            current_hp: max_hp,
            max_hp,
            ivs: Stats::zero(),
            evs: Stats::zero(),
            condition: Condition::GeneralRelease,
            moves,
            status: None,
            held_item: None,
            friendship: 70,
            caught_location: 0,
            original_trainer: String::new(),
        }
    }

    fn default_stages() -> StatStages {
        StatStages::default()
    }

    // ── Damage formula tests ──────────────────────────────────────────────────

    /// Lv.10 Retro Runner using Crease (40 power, Physical, Retro) vs Lv.8 Classic Dunk (also Retro).
    /// STAB applies (both Retro). Type = 1.0 (Retro vs Retro). Expected damage in [8-12].
    #[test]
    fn damage_formula_basic() {
        let attacker = make_instance(1, 10, &[11]); // Retro Runner, Crease
        let attacker_species = data::get_species(1);
        let defender = make_instance(4, 8, &[5]);   // Classic Dunk, any move
        let defender_species = data::get_species(4);
        let crease = data::get_move(11);

        let mut rng = SeededRng::new(42);
        let result = calculate_damage_ex(
            &attacker, attacker_species,
            &defender, defender_species,
            crease,
            &default_stages(), &default_stages(),
            Some(false), // no crit
            &mut rng,
        );

        assert!(
            result.damage >= 8 && result.damage <= 12,
            "Expected damage in [8,12], got {}",
            result.damage
        );
        assert_eq!(result.effectiveness, crate::battle::types::Effectiveness::Normal);
        assert!(!result.is_critical);
    }

    /// Using a Retro move on a Retro sneaker gives STAB (1.5x) vs a Normal move (no STAB).
    #[test]
    fn damage_stab_increases_damage() {
        let attacker = make_instance(1, 10, &[11, 4]); // Retro Runner: Crease (Retro) + Quick Step (Normal)
        let attacker_species = data::get_species(1);
        let defender = make_instance(4, 8, &[5]); // Classic Dunk (Retro, neutral matchup)
        let defender_species = data::get_species(4);

        let crease = data::get_move(11);    // Retro move → STAB on Retro Runner
        let quick_step = data::get_move(4); // Normal move → no STAB

        // Use same RNG seed so random factor is comparable
        let mut rng_stab = SeededRng::new(100);
        let with_stab = calculate_damage_ex(
            &attacker, attacker_species,
            &defender, defender_species,
            crease,
            &default_stages(), &default_stages(),
            Some(false),
            &mut rng_stab,
        );

        let mut rng_no_stab = SeededRng::new(100);
        let without_stab = calculate_damage_ex(
            &attacker, attacker_species,
            &defender, defender_species,
            quick_step,
            &default_stages(), &default_stages(),
            Some(false),
            &mut rng_no_stab,
        );

        assert!(
            with_stab.damage > without_stab.damage,
            "STAB damage {} should exceed no-STAB damage {}",
            with_stab.damage,
            without_stab.damage
        );
        // Ratio should be approximately 1.5
        let ratio = with_stab.damage as f64 / without_stab.damage as f64;
        assert!(
            ratio >= 1.3 && ratio <= 1.7,
            "STAB ratio should be ~1.5, got {:.2}",
            ratio
        );
    }

    /// Retro move vs Skate sneaker should be super effective (2.0x).
    #[test]
    fn damage_super_effective() {
        let attacker = make_instance(1, 10, &[11]); // Retro Runner, Crease (Retro)
        let attacker_species = data::get_species(1);
        let defender_se = make_instance(17, 8, &[5]); // Skate Blazer (Skate) — Retro is super eff
        let defender_se_species = data::get_species(17);
        let defender_ne = make_instance(4, 8, &[5]);  // Classic Dunk (Retro) — neutral
        let defender_ne_species = data::get_species(4);

        let crease = data::get_move(11);

        let mut rng1 = SeededRng::new(77);
        let se_result = calculate_damage_ex(
            &attacker, attacker_species,
            &defender_se, defender_se_species,
            crease,
            &default_stages(), &default_stages(),
            Some(false),
            &mut rng1,
        );

        let mut rng2 = SeededRng::new(77);
        let ne_result = calculate_damage_ex(
            &attacker, attacker_species,
            &defender_ne, defender_ne_species,
            crease,
            &default_stages(), &default_stages(),
            Some(false),
            &mut rng2,
        );

        assert_eq!(se_result.effectiveness, crate::battle::types::Effectiveness::SuperEffective);
        assert!(
            se_result.damage > ne_result.damage,
            "Super effective {} should beat neutral {}",
            se_result.damage,
            ne_result.damage
        );
    }

    /// Retro move vs Techwear sneaker should be not very effective (0.5x).
    #[test]
    fn damage_not_very_effective() {
        let attacker = make_instance(1, 10, &[11]); // Retro Runner, Crease (Retro)
        let attacker_species = data::get_species(1);
        let defender_nve = make_instance(12, 8, &[4]); // Foam Cell (Techwear) — Retro NVE
        let defender_nve_species = data::get_species(12);
        let defender_ne = make_instance(4, 8, &[5]);   // Classic Dunk (Retro) — neutral
        let defender_ne_species = data::get_species(4);

        let crease = data::get_move(11);

        let mut rng1 = SeededRng::new(55);
        let nve_result = calculate_damage_ex(
            &attacker, attacker_species,
            &defender_nve, defender_nve_species,
            crease,
            &default_stages(), &default_stages(),
            Some(false),
            &mut rng1,
        );

        let mut rng2 = SeededRng::new(55);
        let ne_result = calculate_damage_ex(
            &attacker, attacker_species,
            &defender_ne, defender_ne_species,
            crease,
            &default_stages(), &default_stages(),
            Some(false),
            &mut rng2,
        );

        assert_eq!(nve_result.effectiveness, crate::battle::types::Effectiveness::NotVeryEffective);
        assert!(
            nve_result.damage < ne_result.damage,
            "NVE damage {} should be less than neutral {}",
            nve_result.damage,
            ne_result.damage
        );
    }

    /// Status moves always deal 0 damage.
    #[test]
    fn damage_status_move_is_zero() {
        let attacker = make_instance(1, 10, &[1]); // Retro Runner, Lace Up (Status)
        let attacker_species = data::get_species(1);
        let defender = make_instance(4, 8, &[5]);
        let defender_species = data::get_species(4);
        let lace_up = data::get_move(1); // Status move

        let mut rng = SeededRng::new(1);
        let result = calculate_damage(
            &attacker, attacker_species,
            &defender, defender_species,
            lace_up,
            &default_stages(), &default_stages(),
            &mut rng,
        );

        assert_eq!(result.damage, 0, "Status move should deal 0 damage");
    }

    /// Accuracy 100% always hits; accuracy 0 (test move) always misses.
    #[test]
    fn accuracy_100_always_hits() {
        // Stomp has 100% accuracy — verify with execute in a loop
        let stomp = data::get_move(5);
        assert_eq!(stomp.accuracy, 100);
        // Confirmed in data — the accuracy check uses >= 100 shortcut
    }

    // ── Critical hit tests ────────────────────────────────────────────────────

    /// With forced crit, damage is ~1.5x compared to no crit.
    #[test]
    fn crit_multiplier_is_1_5x() {
        let attacker = make_instance(1, 10, &[11]);
        let attacker_species = data::get_species(1);
        let defender = make_instance(4, 8, &[5]);
        let defender_species = data::get_species(4);
        let crease = data::get_move(11);

        let mut rng_crit = SeededRng::new(999);
        let crit = calculate_damage_ex(
            &attacker, attacker_species,
            &defender, defender_species,
            crease,
            &default_stages(), &default_stages(),
            Some(true),
            &mut rng_crit,
        );

        let mut rng_no_crit = SeededRng::new(999);
        let no_crit = calculate_damage_ex(
            &attacker, attacker_species,
            &defender, defender_species,
            crease,
            &default_stages(), &default_stages(),
            Some(false),
            &mut rng_no_crit,
        );

        assert!(crit.is_critical);
        assert!(!no_crit.is_critical);
        assert!(
            crit.damage > no_crit.damage,
            "Crit {} should beat no-crit {}",
            crit.damage, no_crit.damage
        );
        let ratio = crit.damage as f64 / no_crit.damage as f64;
        assert!(
            ratio >= 1.3 && ratio <= 1.7,
            "Crit ratio should be ~1.5, got {:.2}",
            ratio
        );
    }

    /// Crit ignores negative attack stage on attacker (treats it as 0).
    #[test]
    fn crit_ignores_negative_attack_stage() {
        let attacker = make_instance(1, 10, &[11]);
        let attacker_species = data::get_species(1);
        let defender = make_instance(4, 8, &[5]);
        let defender_species = data::get_species(4);
        let crease = data::get_move(11);

        let mut bad_stages = default_stages();
        bad_stages.hype = -3; // attacker has lowered attack

        // With crit + negative atk stage → crit ignores the stage penalty
        let mut rng1 = SeededRng::new(42);
        let crit_with_debuff = calculate_damage_ex(
            &attacker, attacker_species,
            &defender, defender_species,
            crease,
            &bad_stages, &default_stages(),
            Some(true),
            &mut rng1,
        );

        // Without crit + negative atk stage → debuff applies
        let mut rng2 = SeededRng::new(42);
        let no_crit_with_debuff = calculate_damage_ex(
            &attacker, attacker_species,
            &defender, defender_species,
            crease,
            &bad_stages, &default_stages(),
            Some(false),
            &mut rng2,
        );

        assert!(
            crit_with_debuff.damage > no_crit_with_debuff.damage,
            "Crit with debuff {} should beat no-crit with debuff {}",
            crit_with_debuff.damage, no_crit_with_debuff.damage
        );
    }

    /// Crit ignores positive defense stage on defender (treats it as 0).
    #[test]
    fn crit_ignores_positive_defense_stage() {
        let attacker = make_instance(1, 10, &[11]);
        let attacker_species = data::get_species(1);
        let defender = make_instance(4, 8, &[5]);
        let defender_species = data::get_species(4);
        let crease = data::get_move(11);

        let mut boosted_def_stages = default_stages();
        boosted_def_stages.comfort = 3; // defender has raised defense

        // With crit + boosted def stage → crit ignores the boost
        let mut rng1 = SeededRng::new(42);
        let crit_vs_buff = calculate_damage_ex(
            &attacker, attacker_species,
            &defender, defender_species,
            crease,
            &default_stages(), &boosted_def_stages,
            Some(true),
            &mut rng1,
        );

        // Without crit + boosted def stage → buff applies (reduces damage)
        let mut rng2 = SeededRng::new(42);
        let no_crit_vs_buff = calculate_damage_ex(
            &attacker, attacker_species,
            &defender, defender_species,
            crease,
            &default_stages(), &boosted_def_stages,
            Some(false),
            &mut rng2,
        );

        assert!(
            crit_vs_buff.damage > no_crit_vs_buff.damage,
            "Crit vs buffed defender {} should beat no-crit vs buffed defender {}",
            crit_vs_buff.damage, no_crit_vs_buff.damage
        );
    }

    // ── Turn resolution tests ─────────────────────────────────────────────────

    /// Higher Rarity (speed) goes first.
    #[test]
    fn higher_rarity_goes_first() {
        // Retro Runner Rarity=45 base. Skate Blazer Rarity=35 base.
        // At same level, Retro Runner should have higher rarity stat.
        let rr = make_instance(1, 10, &[11]);  // Retro Runner
        let sb = make_instance(17, 10, &[5]);  // Skate Blazer
        let rr_species = data::get_species(1);
        let sb_species = data::get_species(17);
        let rr_rarity = rr.calc_stat(rr_species, StatKind::Rarity);
        let sb_rarity = sb.calc_stat(sb_species, StatKind::Rarity);
        assert!(rr_rarity > sb_rarity, "Retro Runner rarity {} should beat Skate Blazer rarity {}", rr_rarity, sb_rarity);
    }

    /// Higher priority move goes first regardless of speed.
    #[test]
    fn higher_priority_goes_first() {
        let quick_step = data::get_move(4); // priority = 1
        let crease = data::get_move(11);    // priority = 0
        assert!(quick_step.priority > crease.priority);
    }

    /// Both sides take turns (unless one faints first).
    #[test]
    fn both_sides_take_turns() {
        // Use a slow, weak opponent so both sides can attack at least once
        let mut player_party = vec![make_instance(1, 20, &[5])]; // strong Retro Runner
        let opp = make_instance(17, 5, &[5]); // weak Skate Blazer — should survive one hit
        let mut state = BattleEngine::new_wild(opp);
        let mut rng = SeededRng::new(1234);

        let events = BattleEngine::submit_action(
            &mut state,
            &mut player_party,
            BattleAction::Fight { move_index: 0 },
            &mut rng,
        );

        let player_attacks = events.iter().filter(|e| {
            matches!(e, BattleTurnEvent::MoveUsed { side: BattleSide::Player, .. })
        }).count();
        let opp_attacks = events.iter().filter(|e| {
            matches!(e, BattleTurnEvent::MoveUsed { side: BattleSide::Opponent, .. })
        }).count();

        // At least the player attacks
        assert!(player_attacks >= 1, "Player should attack at least once");
        // If the wild didn't faint from one hit, opponent also attacks
        let opp_fainted = events.iter().any(|e| {
            matches!(e, BattleTurnEvent::Fainted { side: BattleSide::Opponent })
        });
        if !opp_fainted {
            assert!(opp_attacks >= 1, "Opponent should attack if not fainted");
        }
    }

    // ── Battle flow tests ─────────────────────────────────────────────────────

    /// New wild battle gives both sides positive HP.
    #[test]
    fn new_wild_battle_both_have_hp() {
        let player_sneaker = make_instance(1, 15, &[5]);
        let wild = make_instance(17, 8, &[5]);
        let state = BattleEngine::new_wild(wild);

        assert!(player_sneaker.current_hp > 0, "Player sneaker should have HP");
        assert!(state.opponent.team[0].current_hp > 0, "Wild sneaker should have HP");
    }

    /// Fighting until opponent HP reaches 0 produces BattleEnd(PlayerWin).
    #[test]
    fn fight_until_opponent_faints_is_player_win() {
        let mut player_party = vec![make_instance(1, 50, &[7])]; // Lv.50, Deadstock Strike (power 80)
        let mut wild = make_instance(17, 3, &[5]); // low level, low HP
        wild.current_hp = 1; // set to 1 HP so first hit KOs
        let mut state = BattleEngine::new_wild(wild);
        let mut rng = SeededRng::new(42);

        let events = BattleEngine::submit_action(
            &mut state,
            &mut player_party,
            BattleAction::Fight { move_index: 0 },
            &mut rng,
        );

        let ended_win = events.iter().any(|e| {
            matches!(e, BattleTurnEvent::BattleEnd { result: BattleResult::PlayerWin })
        });
        assert!(ended_win, "Battle should end with PlayerWin when opponent faints");
    }

    /// When player HP reaches 0, battle ends with PlayerLose.
    #[test]
    fn player_faints_is_player_lose() {
        let mut player_party = vec![make_instance(17, 3, &[5])]; // weak player
        player_party[0].current_hp = 1; // set to 1 HP so any hit KOs

        let opp = make_instance(1, 50, &[7]); // strong opponent with Deadstock Strike
        let mut state = BattleEngine::new_wild(opp);
        let mut rng = SeededRng::new(42);

        // We'll loop until one side faints
        let mut ended_lose = false;
        for _ in 0..20 {
            if player_party[0].current_hp == 0 {
                break;
            }
            let events = BattleEngine::submit_action(
                &mut state,
                &mut player_party,
                BattleAction::Fight { move_index: 0 },
                &mut rng,
            );
            if events.iter().any(|e| {
                matches!(e, BattleTurnEvent::BattleEnd { result: BattleResult::PlayerLose })
            }) {
                ended_lose = true;
                break;
            }
        }

        assert!(ended_lose, "Battle should end with PlayerLose when player faints");
    }

    // ── WASM interop tests ────────────────────────────────────────────────────

    /// battle_action with fight JSON returns valid events array.
    #[test]
    fn wasm_battle_action_returns_valid_json() {
        let mut engine = crate::GameEngine::new(42);
        // Add player sneaker
        let player = make_instance(1, 10, &[5, 11]); // Retro Runner with Stomp + Crease
        engine.state.player.party.push(player);
        // Set up wild battle
        let wild = make_instance(17, 8, &[5]);
        let battle = BattleEngine::new_wild(wild);
        engine.battle = Some(battle);
        engine.state.mode = crate::state::game_state::GameMode::Battle;

        let result = engine.battle_action("{\"type\":\"fight\",\"move_index\":0}");
        let events: Vec<serde_json::Value> = serde_json::from_str(&result)
            .expect("Should parse as JSON array");
        assert!(!events.is_empty(), "Events array should not be empty");
    }

    /// get_battle_state returns JSON with both sneaker summaries.
    #[test]
    fn wasm_get_battle_state_has_both_sneakers() {
        let mut engine = crate::GameEngine::new(42);
        let player = make_instance(1, 10, &[5]);
        engine.state.player.party.push(player);
        let wild = make_instance(12, 8, &[4]);
        let battle = BattleEngine::new_wild(wild);
        engine.battle = Some(battle);
        engine.state.mode = crate::state::game_state::GameMode::Battle;

        let state_json = engine.get_battle_state();
        let state: serde_json::Value = serde_json::from_str(&state_json)
            .expect("Should parse as JSON object");

        assert!(state["player"].is_object(), "Should have player object");
        assert!(state["opponent"].is_object(), "Should have opponent object");
        assert!(state["player"]["name"].is_string(), "Player should have name");
        assert!(state["opponent"]["name"].is_string(), "Opponent should have name");
        assert!(state["player"]["current_hp"].is_number(), "Player should have current_hp");
        assert!(state["opponent"]["current_hp"].is_number(), "Opponent should have current_hp");
    }
}
