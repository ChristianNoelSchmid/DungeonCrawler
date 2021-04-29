use std::collections::{hash_set, HashSet};
use std::fmt::Debug;

use rand::prelude::*;

use super::gen::gen_paths;
pub struct Dungeon {
    width: u32,
    height: u32,

    pub entrance: (u32, u32),
    pub exit: (u32, u32),

    paths: HashSet<(u32, u32)>,
}

impl Dungeon {
    ///
    /// Generates a new `Dungeon` with the specified
    /// `width` and `height`, with randomly generated
    /// paths constrained to size.
    ///
    pub fn new(width: u32, height: u32) -> Self {
        let mut rng = thread_rng();

        // Generate an entrance and exit, which will start on opposite corners
        // rand 0 to 1 for (x, y) position on entrance, multiplied by (width, height),
        // and calculate the opposite corner for exit
        let entrance = (
            rng.next_u32() % width,
            if rng.next_u32() % 2 == 0 {
                0
            } else {
                height - 1
            },
        );
        let exit = (
            rng.next_u32() % width,
            if entrance.1 == 0 { height - 1 } else { 0 },
        );

        let paths = gen_paths(rng.gen::<f64>(), width, height, entrance, exit);

        Dungeon {
            width,
            height,
            entrance,
            exit,
            paths,
        }
    }
    pub fn paths(&self) -> hash_set::Iter<(u32, u32)> {
        self.paths.iter()
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Debug for Dungeon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dungeon:\n{}", {
            let mut map = String::new();
            for row in 0..self.height {
                for col in 0..self.width {
                    map.push_str(if self.entrance == (col, row) {
                        "O "
                    } else if self.exit == (col, row) {
                        "X "
                    } else if self.paths.contains(&(col, row)) {
                        "  "
                    } else {
                        "□ "
                    })
                }
                map.push('\n');
            }
            map
        })
    }
}
