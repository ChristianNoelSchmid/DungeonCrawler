#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub max_health: u32,
    pub cur_health: u32,
    pub max_stamina: u32,
    pub cur_stamina: u32,
    pub max_magicka: u32,
    pub cur_magicka: u32,
}

impl Stats {
    pub fn new(max_health: u32, max_stam: u32, max_magicka: u32) -> Self {
        Self {
            max_health,
            cur_health: max_health,
            max_stamina: max_stam,
            cur_stamina: max_stam,
            max_magicka,
            cur_magicka: max_magicka,
        }
    }
}

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
