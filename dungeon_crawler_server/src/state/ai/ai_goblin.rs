use std::time::Duration;

use crate::state::{ai::ai_packages::AIPackageResult, traits::AI};

use super::ai_packages::IndependentPackage;

pub static GOBLIN_IDLE: IndependentPackage<dyn AI> = IndependentPackage {
    req: |_| true,
    on_start: |entity| {
        if let Some(spot) = entity.spot_within(5) {
            entity.set_target(*spot);
        }
    },
    step_next: |entity| {
        entity.move_next();
        AIPackageResult::Continue
    },
    interval: Duration::from_secs(10),
    pick_count: 1,
};

/*static GOBLIN_COMBAT: AIPackage<&dyn Combater, &mut dyn AI> = AIPackage {
    req: |entity| {
        entity.attk();
        true
    },
    on_start: |entity| {
        entity.set_target()
    },
    step_next: |entity| {
        entity.move_next();
        if entity.next_to_target() {
            entity.attk();
        }
    },
    pick_count: 1,
    interval: Duration::from_secs(3),
};*/
