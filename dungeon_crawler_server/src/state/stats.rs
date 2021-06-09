//! Description of Actor Stats and Abilities
//!
//! Christian Schmid - June, 2021
//! CS510 - Programming Rust

///
/// A collection of health, magicka, and stamina
/// including max and current
///
#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub max_health: u32,
    pub cur_health: i32,
    pub max_stamina: u32,
    pub cur_stamina: i32,
    pub max_magicka: u32,
    pub cur_magicka: i32,
}

impl Stats {
    pub fn new(max_health: u32, max_stam: u32, max_magicka: u32) -> Self {
        Self {
            max_health,
            cur_health: max_health as i32,
            max_stamina: max_stam,
            cur_stamina: max_stam as i32,
            max_magicka,
            cur_magicka: max_magicka as i32,
        }
    }
}

///
/// An entity's might, finesse, and intellect
/// attributes
///
#[derive(Debug, Clone, Copy)]
pub struct Attributes {
    pub might: u32,
    pub fines: u32,
    pub intel: u32,
}

impl Attributes {
    pub fn new(might: u32, fines: u32, intel: u32) -> Self {
        Self {
            might,
            fines,
            intel,
        }
    }
}
