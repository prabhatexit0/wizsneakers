use crate::models::sneaker::{SneakerInstance, SneakerSpecies};
use crate::models::items::{ItemData, ItemEffect};
use crate::util::rng::SeededRng;

pub struct CaptureResult {
    pub shakes: u8,
    pub success: bool,
}

pub fn attempt_capture(
    target: &SneakerInstance,
    species: &SneakerSpecies,
    case_item: &ItemData,
    rng: &mut SeededRng,
) -> CaptureResult {
    // GuaranteedCatch → always success
    if case_item.effect == ItemEffect::GuaranteedCatch {
        return CaptureResult { shakes: 4, success: true };
    }

    let max_hp = target.max_hp as u32;
    let current_hp = target.current_hp as u32;
    let base_catch_rate = species.base_catch_rate as u32;

    let case_bonus: f64 = match case_item.effect {
        ItemEffect::CatchMultiplier(x) => x as f64 / 100.0,
        ItemEffect::CatchMultiplierFaction(faction, x) => {
            if faction == species.faction {
                3.0
            } else {
                x as f64 / 100.0
            }
        }
        _ => 1.0,
    };

    // catch_rate = ((3*max_hp - 2*current_hp) * base_catch_rate * case_bonus) / (3 * max_hp)
    let numerator = (3 * max_hp).saturating_sub(2 * current_hp) as f64
        * base_catch_rate as f64
        * case_bonus;
    let denominator = (3 * max_hp) as f64;
    let catch_rate = (numerator / denominator).clamp(1.0, 255.0) as u32;

    // shake_threshold = 1048560 / sqrt(sqrt(16711680 / catch_rate))
    let inner = 16711680.0 / catch_rate as f64;
    let shake_threshold = (1048560.0 / inner.sqrt().sqrt()) as u32;

    let mut shakes: u8 = 0;
    for _ in 0..4 {
        let roll = rng.range(0, 65536);
        if roll < shake_threshold {
            shakes += 1;
        } else {
            break;
        }
    }

    let success = shakes == 4;
    CaptureResult { shakes, success }
}
