use std::time::Duration;

use crate::{
    astar::find_shortest_path,
    state::{ai::ai_packages::AIPackageResult, traits::AI, types::ResponseType},
};

use super::ai_packages::IndependentPackage;

pub static GOBLIN_IDLE: IndependentPackage<dyn AI> = IndependentPackage {
    req: |_, _| true,
    on_start: |transformer, entity| {
        let transform = transformer.transform(entity.id()).unwrap();
        if let Some(spot) = transformer.open_spot_within(entity.id(), 5) {
            entity.set_path(find_shortest_path(transformer, transform.position, spot));
        }
    },
    step_next: |transformer, entity, s| {
        if let Some(next) = entity.next_step() {
            if transformer.move_pos(entity.id(), next) {
                s.send(ResponseType::MonsterMoved(
                    entity.id(),
                    *transformer.transform(entity.id()).unwrap(),
                ))
                .unwrap();
            } else if let Some(target) = entity.target() {
                entity.set_path(find_shortest_path(
                    transformer,
                    transformer.pos(entity.id()).unwrap(),
                    *target,
                ));
            }
        }

        AIPackageResult::Continue
    },
    interval: Duration::from_secs(3),
    pick_count: 10,
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
