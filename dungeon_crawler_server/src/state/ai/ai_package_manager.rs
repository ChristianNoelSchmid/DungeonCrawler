use std::time::{Duration, Instant};

use crossbeam::channel::Sender;
use rand::{thread_rng, RngCore};

use crate::state::{transforms::world_stage::WorldStage, types::ResponseType};

use super::ai_packages::{AIPackageResult, IndependentPackage};

///
/// A manager which controls a collection of `IndependentPackage` AIs,
/// controlling a specified generic `Entity`, where the `IndependentPackage`
/// relies on the `Entity` itself to determine if the package can be run.
/// See (`DependentPackage` vs. `IndependentPackage`)
///
pub struct IndependentManager<'a, Entity: ?Sized> {
    /// The collection of packages to consider
    packages: Vec<&'a IndependentPackage<Entity>>,
    /// The currently selected package
    selected: Option<usize>,
    /// The duration the selected package is run
    chosen_dur: Duration,
    /// The time at which the current package began running
    start_time: Instant,
}

// Setting `Entity: ?Sized` requires that the Entity is not sized.
// Generally the IndependentManager requires `trait`s, rather than `struct`s.
impl<'a, Entity: ?Sized> IndependentManager<'a, Entity> {
    /// Creates a new `IndependendManager` with the specified `packages`.
    /// No package is chosen to run on start, and it's, rather, assumed that
    /// the AI manager will choose one upon calling `run`.
    pub fn new(packages: Vec<&'a IndependentPackage<Entity>>) -> Self {
        Self {
            packages,
            selected: None,
            start_time: Instant::now(),
            chosen_dur: Duration::from_secs(0),
        }
    }

    /// Runs the current `IndependentPackage`,
    /// or if the package has expired (ie. aborted or timeout)
    /// chooses a new package.
    /// Requires the `world_stage` of the game, the `entity`
    /// being handled, and a `s_to_event Sender<ResponseType>`, to
    /// inform the `EventManager` of any changes in state.
    pub fn run(
        &mut self,
        world_stage: &mut WorldStage,
        entity: &mut Entity,
        s_to_event: &Sender<ResponseType>,
    ) {
        if Instant::now() - self.start_time < self.chosen_dur {
            if let Some(selected) = self.selected {
                if (self.packages[selected].step_next)(world_stage, entity, s_to_event)
                    != AIPackageResult::Abort
                {
                    return;
                }
            }
        }
        self.select_new_pkg(world_stage, entity, s_to_event);
    }

    fn select_new_pkg(
        &mut self,
        world_stage: &mut WorldStage,
        entity: &mut Entity,
        s_to_event: &Sender<ResponseType>,
    ) {
        let packages = self
            .packages
            .iter()
            .filter(|p| (p.req)(world_stage, entity));

        let mut count =
            ((thread_rng().next_u32() % packages.fold(0, |acc, x| acc + x.pick_count)) + 1) as i32;

        for (index, package) in self.packages.iter().enumerate() {
            if (package.req)(world_stage, entity) {
                count -= package.pick_count as i32;
                if count <= 0 {
                    let sel_pkg = self.packages[index as usize];
                    self.selected = Some(index as usize);
                    (sel_pkg.on_start)(world_stage, entity, s_to_event);
                    self.start_time = Instant::now();

                    let rnd_dur = (thread_rng().next_u64() as u128
                        % (sel_pkg.intv_range.1 - sel_pkg.intv_range.0).as_millis()
                        + sel_pkg.intv_range.0.as_millis())
                        as u64;
                    self.chosen_dur = Duration::from_millis(rnd_dur);
                }
            }
        }
    }
}

/*
TODO - eventually build a DependentManger, where the
AI Entity relies on another Entity to determine whether
they can run a certain package (game example: a goblin's
base is being attacked, run back to base!)

pub struct DependentManager<'a, ReqEntity, AIEntity> {
    packages: Vec<&'a DependentPackage<ReqEntity, AIEntity>>,
    selected: Option<usize>,
    start_time: Instant,
}

impl<'a, ReqEntity, AIEntity> DependentManager<'a, ReqEntity, AIEntity> {
    pub fn new(...) -> Self {
        todo!()
    }
    pub fn run(...)) {
        todo!()
    }
}
 */
