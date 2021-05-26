use std::collections::{hash_set, HashSet};
use std::fmt::Debug;

use rand::prelude::*;
use simple_serializer::Serialize;

use super::gen::gen_paths;

///
/// A collection of paths which represents
/// a given dungeon. The dungeon has a width, height,
/// entrance, and exit.
///
#[derive(Clone)]
pub struct Dungeon {
    width: u32,
    height: u32,

    pub entrance: (i32, i32),
    pub exit: (i32, i32),

    paths: HashSet<(i32, i32)>,
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
            (rng.next_u32() % width) as i32,
            if rng.next_u32() % 2 == 0 {
                0
            } else {
                (height - 1) as i32
            },
        );
        let exit = (
            (rng.next_u32() % width) as i32,
            if entrance.1 == 0 {
                (height - 1) as i32
            } else {
                0
            },
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
    /// An iterator over the paths in the dungeon.
    pub fn paths(&self) -> hash_set::Iter<(i32, i32)> {
        self.paths.iter()
    }
    /// A reference to the paths HashSet
    pub fn paths_ref(&self) -> &HashSet<(i32, i32)> {
        &self.paths
    }
    /// The horizontal bounds of the dungeon
    pub fn width(&self) -> u32 {
        self.width
    }
    /// The vertical bounds of the dungeon
    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Serialize for Dungeon {
    type SerializeTo = String;
    /// For the purpose of the Dungeon Crawler project,
    /// Dungeon will Serialize to a String with the appropriate
    /// delimeter between each value ("::")
    fn serialize(&self) -> String {
        let mut path_str = self.paths().len().to_string();

        for path in self.paths() {
            path_str.push_str("::");
            path_str.push_str(&path.0.to_string());
            path_str.push_str("::");
            path_str.push_str(&path.1.to_string());
        }

        path_str.push_str("::");
        path_str.push_str(&self.entrance.0.to_string());
        path_str.push_str("::");
        path_str.push_str(&self.entrance.1.to_string());

        path_str.push_str("::");
        path_str.push_str(&self.exit.0.to_string());
        path_str.push_str("::");
        path_str.push_str(&self.exit.1.to_string());

        path_str
    }
}

// A simple text generation of the Dungeon, displaying its
// walls, paths, entrance, and exit
impl Debug for Dungeon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dungeon:\n{}", {
            let mut map = String::new();
            for row in 0..self.height as i32 {
                for col in 0..self.width as i32 {
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
