use std::time::{Duration, Instant};

use rand::{prelude::IteratorRandom, thread_rng};

use crate::{
    astar::{find_shortest_path, visible_actors},
    state::{actor::ActorId, ai::ai_packages::AIPackageResult, traits::AI},
};

use super::ai_packages::IndependentPackage;

pub static IDLE: IndependentPackage<dyn AI> = IndependentPackage {
    req: |_world_stage, entity| entity.combat_target().is_none(),
    on_start: |world_stage, entity, _s| {
        let transform = world_stage.actor(entity.id()).unwrap().tr;
        if let Some(spot) = world_stage.open_spot_within(entity.id(), 5) {
            entity.set_path(find_shortest_path(world_stage, transform.pos, spot));
        }
    },
    step_next: |world_stage, entity, _s| {
        let ent_tr = world_stage.actor(entity.id()).unwrap().tr;
        let vis_pls = visible_actors(
            world_stage,
            ent_tr,
            &[ActorId::Player],
            entity.sight_range(),
        );

        if !vis_pls.is_empty() {
            entity.start_combat_with(*vis_pls.iter().choose(&mut thread_rng()).unwrap());
            return AIPackageResult::Abort;
        }

        if let Some(next) = entity.next_step() {
            if !world_stage.move_pos(entity.id(), next) {
                if let Some(target) = entity.target() {
                    let target = *target;
                    entity.set_path(find_shortest_path(
                        world_stage,
                        world_stage.pos(entity.id()).unwrap(),
                        target,
                    ));
                }
            }
        }

        AIPackageResult::Continue
    },
    intv_range: (Duration::from_secs(5), Duration::from_secs(10)),
    pick_count: 10,
};

pub static MELEE_COMBAT: IndependentPackage<dyn AI> = IndependentPackage {
    req: |_world_stage, entity| entity.combat_target().is_some(),
    on_start: |world_stage, entity, _s| {
        // Get the entity and its target transforms
        entity.reset_last_sighting();
        let entity_tr = world_stage.actor(entity.id()).unwrap().tr;
        let target_tr = world_stage.actor(entity.combat_target().unwrap())
            .unwrap().tr;

        // Establish the entity's first path as directly towards
        // the target.
        entity.set_path(find_shortest_path(
            world_stage,
            entity_tr.pos,
            target_tr.pos,
        ));
    },
    step_next: |world_stage, entity, _s| {
        // Get the entity transform, and check if there are any visible players
        // in its view.
        let ent_tr = world_stage.actor(entity.id()).unwrap().tr;
        let target_id = entity.combat_target().unwrap();
        let target_tr = world_stage.actor(target_id).unwrap().tr;
        let targets_in_sight = visible_actors(
            world_stage,
            ent_tr,
            &[ActorId::Player],
            entity.sight_range(),
        );

        if targets_in_sight.contains(&target_id) {
            entity.reset_last_sighting();
            if ent_tr.pos.distance(target_tr.pos) <= 1.0 {
                world_stage.attk(entity.id(), target_id);
                return AIPackageResult::Continue;
            }
        }

        // Retrieve the time since the enemy last saw its target
        let last_sighting = Instant::now() - entity.last_sighting();

        match last_sighting {
            // If greater than 3 seconds, stop combat
            s if s > Duration::from_secs(3) => {
                entity.stop_combat();
                return AIPackageResult::Abort;
            }

            s if s > Duration::from_secs_f32(0.5) => {
                if !targets_in_sight.is_empty() {
                    entity.start_combat_with(
                        *targets_in_sight.iter().choose(&mut thread_rng()).unwrap(),
                    )
                }
            }
            // If less than a quarter second, reset its path to its target,
            // this gives the enemy a "search" kind of AI, where it will
            // continue to follow the target's path even if it doesn't see
            // it for a short time
            _ => entity.set_path(find_shortest_path(world_stage, ent_tr.pos, target_tr.pos)),
        }

        if let Some(next) = entity.next_step() {
            if !world_stage.move_pos(entity.id(), next) {
                if let Some(target) = entity.target() {
                    let target = *target;
                    entity.set_path(find_shortest_path(world_stage, ent_tr.pos, target));
                }
            }
        }

        AIPackageResult::Continue
    },
    pick_count: 1,
    intv_range: (Duration::from_secs(2000), Duration::from_secs(3000)),
};
