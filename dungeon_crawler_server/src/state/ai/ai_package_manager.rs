use std::time::Instant;

use crossbeam::channel::Sender;
use rand::{thread_rng, RngCore};

use crate::state::{transforms::world_transformer::WorldTransformer, types::ResponseType};

use super::ai_packages::{AIPackageResult, DependentPackage, IndependentPackage};

pub struct DependentManager<'a, ReqEntity, AIEntity> {
    packages: Vec<&'a DependentPackage<ReqEntity, AIEntity>>,
    selected: Option<usize>,
    start_time: Instant,
}

impl<'a, ReqEntity, AIEntity> DependentManager<'a, ReqEntity, AIEntity> {
    pub fn new(ai_packages: Vec<&'a DependentPackage<ReqEntity, AIEntity>>) -> Self {
        Self {
            packages: ai_packages,
            selected: None,
            start_time: Instant::now(),
        }
    }
    pub fn run(
        &mut self,
        transformer: &mut WorldTransformer,
        req_ent: &ReqEntity,
        ai_ent: &mut AIEntity,
        s_to_event: &Sender<ResponseType>,
    ) {
        let mut choose_new = false;
        if let Some(pack_ind) = self.selected {
            if Instant::now() - self.start_time < self.packages[pack_ind].interval {
                if (self.packages[pack_ind].step_next)(ai_ent, s_to_event) == AIPackageResult::Abort
                {
                    self.selected = None;
                }
            } else {
                choose_new = true;
            }
        } else {
            choose_new = true;
        }

        if choose_new {
            let packages = self.packages.iter().filter(|p| (p.req)(req_ent));

            let mut choice_ind = 0;
            let mut count = (thread_rng().next_u32()
                % packages.clone().fold(0, |acc, x| acc + x.pick_count))
                as i32;

            for package in packages {
                count -= package.pick_count as i32;
                if count < 0 {
                    self.selected = Some(choice_ind as usize);
                    (self.packages[choice_ind as usize].on_start)(ai_ent);
                    break;
                }
                choice_ind += 1;
            }
        }
    }
}

pub struct IndependentManager<'a, Entity: ?Sized> {
    packages: Vec<&'a IndependentPackage<Entity>>,
    selected: Option<usize>,
    start_time: Instant,
}

impl<'a, Entity: ?Sized> IndependentManager<'a, Entity> {
    pub fn new(ai_packages: Vec<&'a IndependentPackage<Entity>>) -> Self {
        Self {
            packages: ai_packages,
            selected: None,
            start_time: Instant::now(),
        }
    }
    pub fn run(
        &mut self,
        transformer: &mut WorldTransformer,
        entity: &mut Entity,
        s_to_event: &Sender<ResponseType>,
    ) {
        let mut choose_new = false;
        if let Some(selected) = self.selected {
            if Instant::now() - self.start_time < self.packages[selected].interval {
                if (self.packages[selected].step_next)(transformer, entity, s_to_event)
                    == AIPackageResult::Abort
                {
                    self.selected = None;
                    choose_new = true;
                }
            } else {
                choose_new = true;
            }
        } else {
            choose_new = true;
        }

        if choose_new {
            let packages = self
                .packages
                .iter()
                .filter(|p| (p.req)(transformer, entity));

            let mut count = ((thread_rng().next_u32()
                % packages.fold(0, |acc, x| acc + x.pick_count))
                + 1) as i32;

            for (index, package) in self.packages.iter().enumerate() {
                if (package.req)(transformer, entity) {
                    count -= package.pick_count as i32;
                    if count <= 0 {
                        println!("Chose a new AIPackage");
                        self.selected = Some(index as usize);
                        (self.packages[index as usize].on_start)(transformer, entity);
                        self.start_time = Instant::now();
                        break;
                    }
                }
            }
        }
    }
}
