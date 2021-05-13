use std::time::{Duration, Instant};

use rand::{prelude::IteratorRandom, thread_rng};

use crate::{
    astar::{find_shortest_path, test_visible_actors},
    state::{actor::ActorId, ai::ai_packages::AIPackageResult, traits::AI, types::ResponseType},
};

use super::ai_packages::IndependentPackage;

pub static GOBLIN_IDLE: IndependentPackage<dyn AI> = IndependentPackage {
    req: |_world_stage, entity| entity.in_combat_with().is_none(),
    on_start: |world_stage, entity| {
        let transform = world_stage.actor(entity.id()).unwrap().tr;
        if let Some(spot) = world_stage.open_spot_within(entity.id(), 5) {
            entity.set_path(find_shortest_path(world_stage, transform.pos, spot));
        }
    },
    step_next: |world_stage, entity, s| {
        let actor_tr = world_stage.actor(entity.id()).unwrap().tr;
        let vis_pls =
            test_visible_actors(world_stage, actor_tr, ActorId::Player, entity.sight_range());

        if vis_pls.len() > 0 {
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
    on_start: |world_stage, entity| {
        let ent_tr = world_stage.actor(entity.id()).unwrap().tr;
        let vis_pls =
            test_visible_actors(world_stage, ent_tr, ActorId::Player, entity.sight_range());

        let other_act = world_stage.actor(entity.in_combat_with().unwrap()).unwrap();
        if vis_pls.contains(&other_act.id) {
            entity.set_last_sighting(Instant::now());
            entity.set_path(find_shortest_path(
                world_stage,
                world_stage.actor(entity.id()).unwrap().tr.pos,
                world_stage
                    .actor(entity.in_combat_with().unwrap())
                    .unwrap()
                    .tr
                    .pos,
            ));
        }
    },
    step_next: |world_stage, entity, s| {
        let ent_tr = world_stage.actor(entity.id()).unwrap().tr;

        let vis_pls =
            test_visible_actors(world_stage, ent_tr, ActorId::Player, entity.sight_range());

        let other_act = world_stage.actor(entity.in_combat_with().unwrap()).unwrap();
        if vis_pls.contains(&other_act.id) {
            entity.set_last_sighting(Instant::now());
        }

        let last = Instant::now() - entity.last_sighting();

        if last > Duration::from_secs(3) {
            entity.stop_combat();
            return AIPackageResult::Abort;
        } else if last < Duration::from_secs(1) {
            entity.set_path(find_shortest_path(
                world_stage,
                world_stage.actor(entity.id()).unwrap().tr.pos,
                world_stage
                    .actor(entity.in_combat_with().unwrap())
                    .unwrap()
                    .tr
                    .pos,
            ));
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
    pick_count: 1,
    intv_range: (Duration::from_secs(5), Duration::from_secs(10)),
};
