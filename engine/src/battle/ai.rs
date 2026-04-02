use crate::models::sneaker::SneakerInstance;
use crate::models::moves::MoveCategory;
use crate::models::stats::StatKind;
use crate::util::rng::SeededRng;
use crate::battle::types::{AiLevel, BattleAction, BattleState};
use crate::data;

/// Choose an action for the AI-controlled side (opponent).
pub fn choose_action(
    state: &BattleState,
    player_team: &[SneakerInstance],
    ai_level: &AiLevel,
    rng: &mut SeededRng,
) -> BattleAction {
    let active = &state.opponent.team[state.opponent_active];
    let player_active = &player_team[state.player_active];
    let player_species = data::get_species(player_active.species_id);
    let active_species = data::get_species(active.species_id);

    match ai_level {
        AiLevel::Random => choose_random_move(active, rng),

        AiLevel::Basic => {
            // 70% chance to pick super-effective move if available
            let se_moves = get_se_move_indices(active, player_species.faction);
            if !se_moves.is_empty() && rng.range(0, 100) < 70 {
                let idx = se_moves[rng.range(0, se_moves.len() as u32) as usize];
                return BattleAction::Fight { move_index: idx as u8 };
            }
            choose_random_move(active, rng)
        }

        AiLevel::Intermediate => {
            // Use healing item when HP < 25%
            if active.current_hp < active.max_hp / 4 {
                if let Some(item_id) = get_healing_item(&state.opponent.items) {
                    return BattleAction::Bag { item_id };
                }
            }

            // Switch if type disadvantaged and < 50% HP and better matchup on bench
            let opp_eff = player_species.faction.effectiveness_against(active_species.faction);
            if opp_eff > 1.0
                && active.current_hp < active.max_hp / 2
                && state.opponent.team.len() > 1
            {
                if let Some(switch_idx) = find_better_matchup(
                    state,
                    player_species.faction,
                    state.opponent_active,
                ) {
                    return BattleAction::Switch { party_index: switch_idx as u8 };
                }
            }

            // 80% chance to pick super-effective move
            let se_moves = get_se_move_indices(active, player_species.faction);
            if !se_moves.is_empty() && rng.range(0, 100) < 80 {
                let idx = se_moves[rng.range(0, se_moves.len() as u32) as usize];
                return BattleAction::Fight { move_index: idx as u8 };
            }
            choose_random_move(active, rng)
        }

        AiLevel::Advanced => {
            // Heal at < 30% HP
            if active.current_hp < active.max_hp * 3 / 10 {
                if let Some(item_id) = get_healing_item(&state.opponent.items) {
                    return BattleAction::Bag { item_id };
                }
            }

            // Switch aggressively to counter
            let opp_eff = player_species.faction.effectiveness_against(active_species.faction);
            if opp_eff > 1.0 && state.opponent.team.len() > 1 {
                if let Some(switch_idx) = find_better_matchup(
                    state,
                    player_species.faction,
                    state.opponent_active,
                ) {
                    return BattleAction::Switch { party_index: switch_idx as u8 };
                }
            }

            // Use stat boost moves at > 75% HP if no boosts yet
            let has_boosts = state.opponent_stages.hype > 0
                || state.opponent_stages.drip > 0
                || state.opponent_stages.comfort > 0;
            if !has_boosts && active.current_hp > active.max_hp * 3 / 4 {
                if let Some(idx) = get_stat_boost_move_index(active) {
                    return BattleAction::Fight { move_index: idx as u8 };
                }
            }

            // Always pick highest-damage move considering type
            choose_best_damage_move(active, player_species.faction, rng)
        }

        AiLevel::Expert => {
            // Heal at < 30% HP
            if active.current_hp < active.max_hp * 3 / 10 {
                if let Some(item_id) = get_healing_item(&state.opponent.items) {
                    return BattleAction::Bag { item_id };
                }
            }

            // Predict switches — if player is at type disadvantage, prepare coverage
            let player_eff = active_species.faction.effectiveness_against(player_species.faction);
            if player_eff > 1.0 {
                // Player might switch — use coverage move targeting likely switch-in
                if let Some(idx) = get_best_coverage_move_index(active, rng) {
                    return BattleAction::Fight { move_index: idx as u8 };
                }
            }

            // Switch aggressively
            let opp_eff = player_species.faction.effectiveness_against(active_species.faction);
            if opp_eff > 1.0 && state.opponent.team.len() > 1 {
                if let Some(switch_idx) = find_better_matchup(
                    state,
                    player_species.faction,
                    state.opponent_active,
                ) {
                    return BattleAction::Switch { party_index: switch_idx as u8 };
                }
            }

            // Use stat boost moves at > 75% HP if no boosts yet
            let has_boosts = state.opponent_stages.hype > 0
                || state.opponent_stages.drip > 0
                || state.opponent_stages.comfort > 0;
            if !has_boosts && active.current_hp > active.max_hp * 3 / 4 {
                if let Some(idx) = get_stat_boost_move_index(active) {
                    return BattleAction::Fight { move_index: idx as u8 };
                }
            }

            choose_best_damage_move(active, player_species.faction, rng)
        }
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

fn choose_random_move(sneaker: &SneakerInstance, rng: &mut SeededRng) -> BattleAction {
    let valid: Vec<usize> = sneaker
        .moves
        .iter()
        .enumerate()
        .filter_map(|(i, slot)| slot.as_ref().and_then(|s| {
            if s.current_pp > 0 { Some(i) } else { None }
        }))
        .collect();

    if valid.is_empty() {
        return BattleAction::Fight { move_index: 0 };
    }
    let idx = valid[rng.range(0, valid.len() as u32) as usize];
    BattleAction::Fight { move_index: idx as u8 }
}

fn get_se_move_indices(
    sneaker: &SneakerInstance,
    defender_faction: crate::models::faction::Faction,
) -> Vec<usize> {
    sneaker
        .moves
        .iter()
        .enumerate()
        .filter_map(|(i, slot)| {
            slot.as_ref().and_then(|s| {
                if s.current_pp == 0 {
                    return None;
                }
                let md = data::get_move(s.move_id);
                if md.category == MoveCategory::Status {
                    return None;
                }
                let eff = md.faction.effectiveness_against(defender_faction);
                if eff > 1.0 { Some(i) } else { None }
            })
        })
        .collect()
}

fn get_healing_item(items: &[(u16, u16)]) -> Option<u16> {
    for &(item_id, qty) in items {
        if qty > 0 {
            let item = data::get_item(item_id);
            match item.effect {
                crate::models::items::ItemEffect::HealHp(_)
                | crate::models::items::ItemEffect::HealFull => return Some(item_id),
                _ => {}
            }
        }
    }
    None
}

fn find_better_matchup(
    state: &BattleState,
    player_faction: crate::models::faction::Faction,
    current_idx: usize,
) -> Option<usize> {
    let mut best_idx: Option<usize> = None;
    let mut best_eff: f64 = 0.0;
    for (i, sneaker) in state.opponent.team.iter().enumerate() {
        if i == current_idx || sneaker.is_fainted() {
            continue;
        }
        let sp = data::get_species(sneaker.species_id);
        // How well does this sneaker attack the player?
        let attack_eff = sp.faction.effectiveness_against(player_faction);
        // How resistant is this sneaker to the player?
        let resist_eff = player_faction.effectiveness_against(sp.faction);
        let score = attack_eff / resist_eff.max(0.5);
        if score > best_eff {
            best_eff = score;
            best_idx = Some(i);
        }
    }
    // Only switch if the candidate is actually better than sticking in
    let current_species = data::get_species(state.opponent.team[current_idx].species_id);
    let current_resist = player_faction.effectiveness_against(current_species.faction);
    if best_eff > 1.0 / current_resist {
        best_idx
    } else {
        None
    }
}

fn get_stat_boost_move_index(sneaker: &SneakerInstance) -> Option<usize> {
    sneaker
        .moves
        .iter()
        .enumerate()
        .find_map(|(i, slot)| {
            slot.as_ref().and_then(|s| {
                if s.current_pp == 0 {
                    return None;
                }
                let md = data::get_move(s.move_id);
                if md.category != MoveCategory::Status {
                    return None;
                }
                // Check if it's a self-buffing move
                match md.effect {
                    crate::models::moves::MoveEffect::StatChange {
                        target: crate::models::moves::MoveTarget::Self_,
                        stages,
                        ..
                    } if stages > 0 => Some(i),
                    crate::models::moves::MoveEffect::MultiStatChange {
                        target: crate::models::moves::MoveTarget::Self_,
                        ..
                    } => Some(i),
                    _ => None,
                }
            })
        })
}

fn get_best_coverage_move_index(
    sneaker: &SneakerInstance,
    rng: &mut SeededRng,
) -> Option<usize> {
    // Pick any non-status move with PP
    let valid: Vec<usize> = sneaker
        .moves
        .iter()
        .enumerate()
        .filter_map(|(i, slot)| {
            slot.as_ref().and_then(|s| {
                if s.current_pp == 0 {
                    return None;
                }
                let md = data::get_move(s.move_id);
                if md.category == MoveCategory::Status { None } else { Some(i) }
            })
        })
        .collect();
    if valid.is_empty() {
        return None;
    }
    Some(valid[rng.range(0, valid.len() as u32) as usize])
}

fn choose_best_damage_move(
    sneaker: &SneakerInstance,
    defender_faction: crate::models::faction::Faction,
    rng: &mut SeededRng,
) -> BattleAction {
    let mut best_idx = 0usize;
    let mut best_score: f64 = -1.0;
    let mut found_any = false;

    for (i, slot) in sneaker.moves.iter().enumerate() {
        if let Some(s) = slot {
            if s.current_pp == 0 {
                continue;
            }
            let md = data::get_move(s.move_id);
            if md.category == MoveCategory::Status {
                continue;
            }
            let power = md.power.unwrap_or(0) as f64;
            let eff = md.faction.effectiveness_against(defender_faction);
            let score = power * eff;
            if score > best_score {
                best_score = score;
                best_idx = i;
                found_any = true;
            }
        }
    }

    if found_any {
        BattleAction::Fight { move_index: best_idx as u8 }
    } else {
        choose_random_move(sneaker, rng)
    }
}
