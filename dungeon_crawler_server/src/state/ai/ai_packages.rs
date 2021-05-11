//!
//! AIPackage struct, used to express AI functionality on
//! a given entity
//!
//! Christian Schmid - May, 2021
//!

use std::{time::Duration};

#[derive(Debug, Eq, PartialEq)]
pub enum AIPackageResult {
    Continue,
    Abort
}

///
/// A structure which holds all the information
/// for an AI entity to implement actions and
/// directions based on the given circumstances
/// There are two generics associated with this struct:
///     
/// `ReqEntity` - the trait used to determine what requirements are
///         necessary for the AIPackage to be run on the given
///         `Entity`
///
/// `AIEntity` - the collection of traits which will implement the
///            AI actions.
///
#[derive(Clone, Copy)]
pub struct DependentPackage<ReqEntity, AIEntity> {
    ///
    /// Method which determines if the given
    /// `Req` traits' conditions are met
    ///
    pub req: fn(entity: &ReqEntity) -> bool,
    ///
    /// Method that runs upon the `AIPackage`
    /// starting. Initial actions of the `Entity`.
    ///
    pub on_start: fn(entity: &mut AIEntity),
    ///
    /// An increment in the `Entity`'s AI system
    ///
    pub step_next: fn(entity: &mut AIEntity) -> AIPackageResult,
    ///
    /// The amount of time the `AIPackage` will run
    /// on the given Entity, assuming that `req` is
    /// continually met.
    ///
    pub interval: Duration,
    ///
    /// The chance of the AIPackage being chosen among
    /// several when the `AIManager` chooses.
    ///
    pub pick_count: u32,
}

///
/// A structure which holds all the information
/// for an AI entity to implement actions and
/// directions based on the given circumstances
/// There is one generic associated with this struct:
///     
/// `Entity` - the trait used to determine what requirements are
///         *and* the traits which implement the actions. 
///
#[derive(Clone, Copy)]
pub struct IndependentPackage<Entity> {
    ///
    /// Method which determines if the given
    /// `Req` traits' conditions are met
    ///
    pub req: fn(entity: &Entity) -> bool,
    ///
    /// Method that runs upon the `AIPackage`
    /// starting. Initial actions of the `Entity`.
    ///
    pub on_start: fn(entity: &mut Entity),
    ///
    /// An increment in the `Entity`'s AI system
    ///
    pub step_next: fn(entity: &mut Entity) -> AIPackageResult,
    ///
    /// The amount of time the `AIPackage` will run
    /// on the given Entity, assuming that `req` is
    /// continually met.
    ///
    pub interval: Duration,
    ///
    /// The chance of the AIPackage being chosen among
    /// several when the `AIManager` chooses.
    ///
    pub pick_count: u32,
}
