use crate::models::sneaker::{SneakerInstance, StatusCondition};
use crate::models::moves::StatusType;
use crate::util::rng::SeededRng;
use crate::battle::types::{BattleSide, BattleTurnEvent};

/// Apply end-of-turn status effects (damage, decrements, expiry).
/// Handles both major status and volatile OnFire separately.
pub fn apply_end_of_turn_status(
    sneaker: &mut SneakerInstance,
    side: BattleSide,
    events: &mut Vec<BattleTurnEvent>,
) {
    // ── Volatile: OnFire ──────────────────────────────────────────────────────
    if sneaker.on_fire_turns > 0 {
        let dmg = (sneaker.max_hp / 10).max(1);
        let actual = dmg.min(sneaker.current_hp);
        sneaker.current_hp -= actual;
        events.push(BattleTurnEvent::StatusDamage { side: side.clone(), amount: actual });
        if sneaker.is_fainted() {
            events.push(BattleTurnEvent::Fainted { side: side.clone() });
        }
        sneaker.on_fire_turns -= 1;
    }

    // ── Major status ──────────────────────────────────────────────────────────
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
            // Creased persists until healed — no turn counter
        }
        StatusCondition::Scuffed { turns_left } => {
            if turns_left <= 1 {
                sneaker.status = None;
                events.push(BattleTurnEvent::Message {
                    text: "The Scuffed status wore off!".to_string(),
                });
            } else {
                sneaker.status = Some(StatusCondition::Scuffed { turns_left: turns_left - 1 });
            }
        }
        StatusCondition::SoldOut { turns_left } => {
            if turns_left <= 1 {
                sneaker.status = None;
                events.push(BattleTurnEvent::Message {
                    text: "The SoldOut status wore off!".to_string(),
                });
            } else {
                sneaker.status = Some(StatusCondition::SoldOut { turns_left: turns_left - 1 });
            }
        }
        StatusCondition::Hypnotized { turns_left } => {
            if turns_left <= 1 {
                sneaker.status = None;
                events.push(BattleTurnEvent::Message {
                    text: "Snapped out of hypnosis!".to_string(),
                });
            } else {
                sneaker.status = Some(StatusCondition::Hypnotized { turns_left: turns_left - 1 });
            }
        }
        StatusCondition::Deflated => {
            // Deflated persists until healed — no turn counter
        }
    }
}

/// Check whether a sneaker CAN use a move this turn.
/// SoldOut: always can't move. Hypnotized: 50% chance to self-hit instead (handled in engine).
pub fn check_can_move_sold_out(sneaker: &SneakerInstance) -> bool {
    !matches!(sneaker.status, Some(StatusCondition::SoldOut { .. }))
}

/// Returns true if a major status can be applied to this sneaker
/// (i.e. it has no existing major status).
pub fn can_apply_major_status(sneaker: &SneakerInstance) -> bool {
    sneaker.status.is_none()
}

/// Returns true if OnFire can be applied (not already on fire).
pub fn can_apply_onfire(sneaker: &SneakerInstance) -> bool {
    sneaker.on_fire_turns == 0
}

/// Build a StatusCondition from a StatusType with an RNG-rolled duration.
pub fn make_status_condition(status_type: StatusType, rng: &mut SeededRng) -> Option<StatusCondition> {
    match status_type {
        StatusType::Creased  => Some(StatusCondition::Creased),
        StatusType::Scuffed  => Some(StatusCondition::Scuffed  { turns_left: (rng.range(1, 5)) as u8 }),
        StatusType::SoldOut  => Some(StatusCondition::SoldOut  { turns_left: (rng.range(1, 3)) as u8 }),
        StatusType::Hypnotized => Some(StatusCondition::Hypnotized { turns_left: (rng.range(1, 5)) as u8 }),
        StatusType::Deflated => Some(StatusCondition::Deflated),
        StatusType::OnFire   => None, // OnFire is applied via on_fire_turns, not this path
    }
}

// ── Legacy shim (kept so old call-sites compile) ──────────────────────────────
/// Deprecated: use check_can_move_sold_out instead.
/// Kept for backwards compat — now checks SoldOut and Hypnotized (50% skip).
pub fn check_can_move(sneaker: &SneakerInstance, rng: &mut SeededRng) -> bool {
    match sneaker.status {
        Some(StatusCondition::SoldOut { .. }) => false,
        Some(StatusCondition::Hypnotized { .. }) => rng.range(0, 2) != 0,
        _ => true,
    }
}
