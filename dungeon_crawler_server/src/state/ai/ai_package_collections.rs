use std::time::{Duration, Instant};

use rand::{prelude::IteratorRandom, thread_rng};

use crate::{
    astar::{find_shortest_path, visible_actors},
    state::{
        actor::ActorId,
        ai::ai_packages::AIPackageResult,
        traits::{AttackResult, AI},
        transforms::vec2::Vec2,
        types::ResponseType,
    },
};

use super::ai_packages::IndependentPackage;

pub static IDLE: IndependentPackage<dyn AI> = IndependentPackage {
    req: |_world_stage, entity| entity.in_combat_with().is_none(),
    on_start: |world_stage, entity, _s| {
        let transform = world_stage.actor(entity.id()).unwrap().tr;
        if let Some(spot) = world_stage.open_spot_within(entity.id(), 5) {
            entity.set_path(find_shortest_path(world_stage, transform.pos, spot));
        }
    },
    step_next: |world_stage, entity, s| {
        let actor_tr = world_stage.actor(entity.id()).unwrap().tr;
        let vis_pls = visible_actors(
            world_stage,
            actor_tr,
            &[ActorId::Player],
            entity.sight_range(),
        );

        if !vis_pls.is_empty() {
            entity.start_combat_with(*vis_pls.iter().choose(&mut thread_rng()).unwrap());
            return AIPackageResult::Abort;
        }

        if let Some(next) = entity.next_step() {
            if world_stage.move_pos(entity.id(), next) {
                s.send(ResponseType::MonsterMoved(
                    entity.id(),
                    world_stage.actor(entity.id()).unwrap().tr,
                ))
                .unwrap();
            } else if let Some(target) = entity.target() {
                let target = *target;
                entity.set_path(find_shortest_path(
                    world_stage,
                    world_stage.pos(entity.id()).unwrap(),
                    target,
                ));
            }
        }

        AIPackageResult::Continue
    },
    intv_range: (Duration::from_secs(5), Duration::from_secs(10)),
    pick_count: 10,
};

pub static MELEE_COMBAT: IndependentPackage<dyn AI> = IndependentPackage {
    req: |_world_stage, entity| entity.in_combat_with().is_some(),
    on_start: |world_stage, entity, _s| {
        // Get the entity and its target transforms
        entity.set_last_sighting(Instant::now());
        let entity_tr = world_stage.actor(entity.id()).unwrap().tr;
        let target_tr = world_stage
            .actor(entity.in_combat_with().unwrap())
            .unwrap()
            .tr;

        // Establish the entity's first path as directly towards
        // the target.
        entity.set_path(find_shortest_path(
            world_stage,
            entity_tr.pos,
            target_tr.pos,
        ));
    },
    step_next: |world_stage, entity, s| {
        // Get the entity transform, and check if there are any visible players
        // in its view.
        let ent_tr = world_stage.actor(entity.id()).unwrap().tr;

        let vis_pls = visible_actors(
            world_stage,
            ent_tr,
            &ActorId::all_but(ActorId::Monster),
            entity.sight_range(),
        );

        // If it sees its combat target, set its last sighting to now
        let other_act = world_stage.actor(entity.in_combat_with().unwrap()).unwrap();

        if vis_pls.contains(&other_act.id) {
            entity.set_last_sighting(Instant::now());

            if Vec2::distance(ent_tr.pos, other_act.tr.pos) <= 1.0 {
                match world_stage.attk(entity.id(), entity.in_combat_with().unwrap()) {
                    AttackResult::Hit(id, cur_health) => s.send(ResponseType::Hit(id, entity.in_combat_with().unwrap(), cur_health)).unwrap(),
                    AttackResult::Miss(id) => s.send(ResponseType::Miss(id, entity.in_combat_with().unwrap())).unwrap(),
                }
                return AIPackageResult::Continue;
            }
        }

        let other_tr = other_act.tr;

        // Retrieve the time since the enemy last saw its target
        let last = Instant::now() - entity.last_sighting();

        // If greater than 3 seconds, stop combat
        if last > Duration::from_secs(3) {
            entity.stop_combat();
            return AIPackageResult::Abort;
        // If less than a quarter second, reset its path to its target,
        // this gives the enemy a "search" kind of AI, where it will
        // continue to follow the target's path even if it doesn't see
        // it for a short time
        } else if last < Duration::from_secs_f32(0.5) {
            entity.set_path(find_shortest_path(
                world_stage,
                ent_tr.pos,
                other_tr.pos,
            ));
        }

        if let Some(next) = entity.next_step() {
            if world_stage.move_pos(entity.id(), next) {
                s.send(ResponseType::MonsterMoved(
                    entity.id(),
                    ent_tr,
                ))
                .unwrap();
            } else if let Some(target) = entity.target() {
                let target = *target;
                entity.set_path(find_shortest_path(
                    world_stage,
                    ent_tr.pos,
                    target,
                ));
            }
        }
        AIPackageResult::Continue
    },
    pick_count: 1,
    intv_range: (Duration::from_secs(2000), Duration::from_secs(3000)),
};
