use std::time::Instant;

use rand::{thread_rng, RngCore};

use super::ai_packages::{AIPackageResult, DependentPackage, IndependentPackage};

pub struct DependentManager<'a, ReqEntity, AIEntity> {
    ai_packages: Vec<&'a DependentPackage<ReqEntity, AIEntity>>,
    sel_pack_ind: Option<usize>,
    pack_start: Instant,
}

impl<'a, ReqEntity, AIEntity> DependentManager<'a, ReqEntity, AIEntity> {
    pub fn new(ai_packages: Vec<&'a DependentPackage<ReqEntity, AIEntity>>) -> Self {
        Self {
            ai_packages: ai_packages,
            sel_pack_ind: None,
            pack_start: Instant::now(),
        }
    }
    pub fn run(&mut self, req_ent: &ReqEntity, ai_ent: &mut AIEntity) {
        let mut choose_new = false;
        if let Some(pack_ind) = self.sel_pack_ind {
            if Instant::now() - self.pack_start < self.ai_packages[pack_ind].interval {
                if (self.ai_packages[pack_ind].step_next)(ai_ent) == AIPackageResult::Abort {
                    self.sel_pack_ind = None;
                }
            } else {
                choose_new = true;
            }
        } else {
            choose_new = true;
        }

        if choose_new {
            let packages = self.ai_packages.iter().filter(|p| (p.req)(req_ent));

            let mut choice_ind = 0;
            let mut count = (thread_rng().next_u32()
                % packages.clone().fold(0, |acc, x| acc + x.pick_count))
                as i32;

            for package in packages {
                count -= package.pick_count as i32;
                if count < 0 {
                    self.sel_pack_ind = Some(choice_ind as usize);
                    (self.ai_packages[choice_ind as usize].on_start)(ai_ent);
                    break;
                }
                choice_ind += 1;
            }
        }
    }
}

pub struct IndependentManager<'a, Entity: ?Sized> {
    ai_packages: Vec<&'a IndependentPackage<Entity>>,
    sel_pack_ind: Option<usize>,
    pack_start: Instant,
}

impl<'a, Entity: ?Sized> IndependentManager<'a, Entity> {
    pub fn new(ai_packages: Vec<&'a IndependentPackage<Entity>>) -> Self {
        Self {
            ai_packages: ai_packages,
            sel_pack_ind: None,
            pack_start: Instant::now(),
        }
    }
    pub fn run(&mut self, entity: &mut Entity) {
        let mut choose_new = false;
        if let Some(pack_ind) = self.sel_pack_ind {
            if Instant::now() - self.pack_start < self.ai_packages[pack_ind].interval {
                if (self.ai_packages[pack_ind].step_next)(entity) == AIPackageResult::Abort {
                    self.sel_pack_ind = None;
                }
            } else {
                choose_new = true;
            }
        } else {
            choose_new = true;
        }

        if choose_new {
            let packages = self.ai_packages.iter().filter(|p| (p.req)(entity));

            let mut choice_ind = 0;
            let mut count = (thread_rng().next_u32()
                % packages.clone().fold(0, |acc, x| acc + x.pick_count))
                as i32;

            for package in packages {
                count -= package.pick_count as i32;
                if count < 0 {
                    self.sel_pack_ind = Some(choice_ind as usize);
                    (self.ai_packages[choice_ind as usize].on_start)(entity);
                    break;
                }
                choice_ind += 1;
            }
        }
    }
}
