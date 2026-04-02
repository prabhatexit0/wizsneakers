use crate::models::sneaker::{SneakerInstance};
use crate::models::stats::{Stats, Condition};
use crate::models::moves::MoveSlot;
use crate::util::rng::SeededRng;
use crate::world::map::WildEncounterEntry;
use crate::data;

/// Weighted random selection from encounter table.
/// Returns `(species_id, level)` where level is random within the entry's range.
///
/// The 15% base encounter chance must be checked by the **caller** before calling this.
/// Returns `None` if the table is empty or has zero total weight.
pub fn check_wild_encounter(
    table: &[WildEncounterEntry],
    rng: &mut SeededRng,
) -> Option<(u16, u8)> {
    if table.is_empty() {
        return None;
    }

    let total_weight: u32 = table.iter().map(|e| e.weight).sum();
    if total_weight == 0 {
        return None;
    }

    let roll = rng.range(0, total_weight);
    let mut cumulative = 0u32;
    for entry in table {
        cumulative += entry.weight;
        if roll < cumulative {
            let level = pick_level(entry, rng);
            return Some((entry.species_id, level));
        }
    }

    // Fallback (shouldn't be reached with valid weights)
    let entry = &table[table.len() - 1];
    Some((entry.species_id, pick_level(entry, rng)))
}

fn pick_level(entry: &WildEncounterEntry, rng: &mut SeededRng) -> u8 {
    if entry.level_min >= entry.level_max {
        entry.level_min
    } else {
        rng.range(entry.level_min as u32, entry.level_max as u32 + 1) as u8
    }
}

/// Generate a wild `SneakerInstance` with random IVs, a random condition, and
/// appropriate moves from the species learnset.
pub fn generate_wild_sneaker(
    species_id: u16,
    level: u8,
    rng: &mut SeededRng,
) -> SneakerInstance {
    let species = data::get_species(species_id);

    // Random IVs: 0–31 each stat
    let ivs = Stats {
        durability: rng.range(0, 32) as u16,
        hype:       rng.range(0, 32) as u16,
        comfort:    rng.range(0, 32) as u16,
        drip:       rng.range(0, 32) as u16,
        rarity:     rng.range(0, 32) as u16,
    };

    // Random condition from all 9 variants
    let condition = match rng.range(0, 9) {
        0 => Condition::Deadstock,
        1 => Condition::Beat,
        2 => Condition::Restored,
        3 => Condition::Custom,
        4 => Condition::Vintage,
        5 => Condition::Prototype,
        6 => Condition::PlayerExclusive,
        7 => Condition::Sample,
        _ => Condition::GeneralRelease,
    };

    // Learnset moves at or below `level` — take up to the last 4 (highest-level)
    let available: Vec<(u8, u16)> = species.learnset
        .iter()
        .copied()
        .filter(|&(lv, _)| lv <= level)
        .collect();

    let start = available.len().saturating_sub(4);
    let mut moves = [None, None, None, None];
    for (i, &(_, move_id)) in available[start..].iter().enumerate() {
        let md = data::get_move(move_id);
        moves[i] = Some(MoveSlot { move_id, current_pp: md.pp, max_pp: md.pp });
    }

    // If nothing was learned yet, give the species' first learnset move
    if moves[0].is_none() {
        if let Some(&(_, move_id)) = species.learnset.first() {
            let md = data::get_move(move_id);
            moves[0] = Some(MoveSlot { move_id, current_pp: md.pp, max_pp: md.pp });
        }
    }

    // Calculate max HP: (2*base + iv) * level/100 + level + 10  (no EVs for wild)
    let base_hp = species.base_stats.durability as u32;
    let iv_hp   = ivs.durability as u32;
    let inner   = (2 * base_hp + iv_hp) * level as u32 / 100;
    let max_hp  = (inner + level as u32 + 10) as u16;

    let uid = rng.next_u64();

    SneakerInstance {
        uid,
        species_id,
        nickname: None,
        level,
        xp: 0,
        current_hp: max_hp,
        max_hp,
        ivs,
        evs: Stats::zero(),
        condition,
        moves,
        status: None,
        held_item: None,
        friendship: 70,
        caught_location: 0,
        original_trainer: String::new(),
    }
}
