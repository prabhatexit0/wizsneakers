use crate::models::sneaker::{SneakerInstance, SneakerSpecies, StatusCondition};
use crate::models::moves::{MoveData, MoveCategory};
use crate::models::stats::{StatKind, StatStages};
use crate::util::rng::SeededRng;
use crate::battle::types::Effectiveness;

#[derive(Debug, Clone)]
pub struct DamageResult {
    pub damage: u16,
    pub effectiveness: Effectiveness,
    pub is_critical: bool,
    pub type_multiplier: f64,
}

/// Calculate damage for an attack (standard 1/16 crit rate).
pub fn calculate_damage(
    attacker: &SneakerInstance,
    attacker_species: &SneakerSpecies,
    defender: &SneakerInstance,
    defender_species: &SneakerSpecies,
    move_data: &MoveData,
    attacker_stages: &StatStages,
    defender_stages: &StatStages,
    rng: &mut SeededRng,
) -> DamageResult {
    let power = match move_data.power {
        Some(p) if move_data.category != MoveCategory::Status => p as f64,
        _ => {
            return DamageResult {
                damage: 0,
                effectiveness: Effectiveness::Normal,
                is_critical: false,
                type_multiplier: 1.0,
            };
        }
    };

    let is_critical = rng.range(0, 16) == 0;
    calculate_damage_inner(
        attacker, attacker_species,
        defender, defender_species,
        move_data, attacker_stages, defender_stages,
        power, is_critical, rng,
    )
}

/// Extended damage calculation with optional forced crit (for testing).
pub fn calculate_damage_ex(
    attacker: &SneakerInstance,
    attacker_species: &SneakerSpecies,
    defender: &SneakerInstance,
    defender_species: &SneakerSpecies,
    move_data: &MoveData,
    attacker_stages: &StatStages,
    defender_stages: &StatStages,
    force_critical: Option<bool>,
    rng: &mut SeededRng,
) -> DamageResult {
    let power = match move_data.power {
        Some(p) if move_data.category != MoveCategory::Status => p as f64,
        _ => {
            return DamageResult {
                damage: 0,
                effectiveness: Effectiveness::Normal,
                is_critical: false,
                type_multiplier: 1.0,
            };
        }
    };

    let is_critical = match force_critical {
        Some(b) => b,
        None => rng.range(0, 16) == 0,
    };

    calculate_damage_inner(
        attacker, attacker_species,
        defender, defender_species,
        move_data, attacker_stages, defender_stages,
        power, is_critical, rng,
    )
}

/// Damage with an explicit power override (for PowerEqualsLevel, etc.) and optional forced crit.
pub fn calculate_damage_with_override(
    attacker: &SneakerInstance,
    attacker_species: &SneakerSpecies,
    defender: &SneakerInstance,
    defender_species: &SneakerSpecies,
    move_data: &MoveData,
    attacker_stages: &StatStages,
    defender_stages: &StatStages,
    power_override: f64,
    force_critical: Option<bool>,
    rng: &mut SeededRng,
) -> DamageResult {
    let is_critical = match force_critical {
        Some(b) => b,
        None => rng.range(0, 16) == 0,
    };
    calculate_damage_inner(
        attacker, attacker_species,
        defender, defender_species,
        move_data, attacker_stages, defender_stages,
        power_override, is_critical, rng,
    )
}

fn calculate_damage_inner(
    attacker: &SneakerInstance,
    attacker_species: &SneakerSpecies,
    defender: &SneakerInstance,
    defender_species: &SneakerSpecies,
    move_data: &MoveData,
    attacker_stages: &StatStages,
    defender_stages: &StatStages,
    power: f64,
    is_critical: bool,
    rng: &mut SeededRng,
) -> DamageResult {
    let level = attacker.level as f64;

    // Attack stat: Hype for Physical, Drip for Special
    // Crits ignore negative attack stages.
    let mut attack_stat = match move_data.category {
        MoveCategory::Physical => {
            let base = attacker.calc_stat(attacker_species, StatKind::Hype) as f64;
            let stage = if is_critical { attacker_stages.hype.max(0) } else { attacker_stages.hype };
            base * StatStages::multiplier(stage)
        }
        MoveCategory::Special => {
            let base = attacker.calc_stat(attacker_species, StatKind::Drip) as f64;
            let stage = if is_critical { attacker_stages.drip.max(0) } else { attacker_stages.drip };
            base * StatStages::multiplier(stage)
        }
        MoveCategory::Status => unreachable!(),
    };

    // Status modifiers on attacker's Hype (physical only):
    //   Scuffed:  -50% Hype
    //   OnFire:   +50% Hype (volatile, checked via on_fire_turns)
    if move_data.category == MoveCategory::Physical {
        if matches!(attacker.status, Some(StatusCondition::Scuffed { .. })) {
            attack_stat *= 0.5;
        }
        if attacker.on_fire_turns > 0 {
            attack_stat *= 1.5;
        }
    }

    // Defense stat: Comfort — crits ignore positive defense stages
    let def_base = defender.calc_stat(defender_species, StatKind::Comfort) as f64;
    let def_stage = if is_critical {
        defender_stages.comfort.min(0)
    } else {
        defender_stages.comfort
    };
    let defense = (def_base * StatStages::multiplier(def_stage)).max(1.0);

    // Damage formula: ((2*level/5+2) * power * attack/defense) / 50 + 2
    let level_factor = 2.0 * level / 5.0 + 2.0;
    let base = (level_factor * power * attack_stat / defense) / 50.0 + 2.0;

    // STAB
    let stab = if move_data.faction == attacker_species.faction { 1.5_f64 } else { 1.0_f64 };

    // Type effectiveness
    let type_mult = move_data.faction.effectiveness_against(defender_species.faction);
    let effectiveness = if type_mult > 1.0 {
        Effectiveness::SuperEffective
    } else if type_mult < 1.0 {
        Effectiveness::NotVeryEffective
    } else {
        Effectiveness::Normal
    };

    // Critical hit multiplier
    let crit_mult = if is_critical { 1.5_f64 } else { 1.0_f64 };

    // Random factor [0.85, 1.00]
    let random = 0.85 + rng.next_f64() * 0.15;

    let damage = (base * stab * type_mult * crit_mult * random).max(1.0) as u16;

    DamageResult {
        damage,
        effectiveness,
        is_critical,
        type_multiplier: type_mult,
    }
}
