use crate::models::sneaker::{SneakerInstance, StatusCondition};
use crate::util::rng::SeededRng;
use crate::battle::types::{BattleSide, BattleTurnEvent};

/// Apply end-of-turn status damage and decrement counters.
/// Creased: lose 1/8 max HP. OnFire: lose 1/10 max HP. Others: decrement turn counter.
pub fn apply_end_of_turn_status(
    sneaker: &mut SneakerInstance,
    side: BattleSide,
    events: &mut Vec<BattleTurnEvent>,
) {
    let status = match sneaker.status {
        Some(s) => s,
        None => return,
    };

    match status {
        StatusCondition::Creased => {
            let dmg = (sneaker.max_hp / 8).max(1);
            let actual = dmg.min(sneaker.current_hp);
            sneaker.current_hp -= actual;
            events.push(BattleTurnEvent::StatusDamage { side: side.clone(), amount: actual });
            if sneaker.is_fainted() {
                events.push(BattleTurnEvent::Fainted { side });
            }
        }
        StatusCondition::OnFire { turns_left } => {
            let dmg = (sneaker.max_hp / 10).max(1);
            let actual = dmg.min(sneaker.current_hp);
            sneaker.current_hp -= actual;
            events.push(BattleTurnEvent::StatusDamage { side: side.clone(), amount: actual });
            if sneaker.is_fainted() {
                events.push(BattleTurnEvent::Fainted { side });
            }
            if turns_left <= 1 {
                sneaker.status = None;
            } else {
                sneaker.status = Some(StatusCondition::OnFire { turns_left: turns_left - 1 });
            }
        }
        StatusCondition::Scuffed { turns_left } => {
            if turns_left <= 1 {
                sneaker.status = None;
            } else {
                sneaker.status = Some(StatusCondition::Scuffed { turns_left: turns_left - 1 });
            }
        }
        StatusCondition::SoldOut { turns_left } => {
            if turns_left <= 1 {
                sneaker.status = None;
            } else {
                sneaker.status = Some(StatusCondition::SoldOut { turns_left: turns_left - 1 });
            }
        }
        StatusCondition::Hypnotized { turns_left } => {
            if turns_left <= 1 {
                sneaker.status = None;
            } else {
                sneaker.status = Some(StatusCondition::Hypnotized { turns_left: turns_left - 1 });
            }
        }
        StatusCondition::Deflated => {
            // Deflated persists until healed — no turn counter
        }
    }
}

/// Check if a sneaker can use a move this turn.
/// SoldOut: can't move. Hypnotized: 50% chance to be unable to act.
/// Returns true if the sneaker can move.
pub fn check_can_move(sneaker: &SneakerInstance, rng: &mut SeededRng) -> bool {
    match sneaker.status {
        Some(StatusCondition::SoldOut { .. }) => false,
        Some(StatusCondition::Hypnotized { .. }) => rng.range(0, 2) != 0,
        _ => true,
    }
}
