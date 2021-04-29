//! rpg_rust - Helper functions for dungeon generation
//!
//! Christian Schmid - April 2021
//! CS510 - Rust Programming

use rand::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;

use noise::{NoiseFn, Perlin};

/// The space between perlin noise values
const PERLIN_SCALE: (f64, f64) = (0.25, 0.25);

/// The threshold a perlin noise value must be greater than to be a path
const PERLIN_THRESHOLD: f64 = 0.05;

///
/// Generates a path, given a perlin `seed`,
/// `width` and `height` of the map, and the
/// `entrance` and `exit` points
///
pub fn gen_paths(
    seed: f64,
    width: u32,
    height: u32,
    entrance: (u32, u32),
    exit: (u32, u32),
) -> HashSet<(u32, u32)> {
    // Convert the u32 values to i32s, to ensure that there
    // is no extra conversions, or overflow in the increment / decrementation
    let (width, height) = (width as i32, height as i32);
    let entrance = (entrance.0 as i32, entrance.1 as i32);
    let exit = (exit.0 as i32, exit.1 as i32);

    let mut paths = build_path(entrance, exit, width);
    layer_path(&mut paths, seed, width, height);

    paths
        .iter()
        .map(|p| (p.0 as u32, p.1 as u32))
        .collect::<HashSet<(u32, u32)>>()
}

/// Generate a path from the entrance to exit. This is NOT
/// a shortest-path route, but rather a random path
fn build_path(entrance: (i32, i32), exit: (i32, i32), width: i32) -> HashSet<(i32, i32)> {
    // Random number generator
    let mut rnd = thread_rng();

    // The paths set, returned at end
    let mut paths = HashSet::<(i32, i32)>::from_iter([entrance, exit].iter().cloned());

    // This algorithm ensures that the path being generated is always heading
    // towards the exit.
    let mut current = entrance;
    let mut last = current;
    let mut move_dirs = Vec::with_capacity(3);
    let y_dir: i32 = if entrance.1 == 0 { 1 } else { -1 };

    while current != exit {
        move_dirs.clear();
        if current.1 == exit.1 {
            current = if current.0 < exit.0 {
                (current.0 + 1, current.1)
            } else {
                (current.0 - 1, current.1)
            }
        } else {
            if current.0 != 0 {
                move_dirs.push((current.0 - 1, current.1));
                move_dirs.push((current.0 - 1, current.1));
                if current.0 > exit.0 {
                    move_dirs.push((current.0 - 1, current.1));
                }
            }
            if current.0 < width - 1 {
                move_dirs.push((current.0 + 1, current.1));
                move_dirs.push((current.0 + 1, current.1));
                if current.0 < exit.0 {
                    move_dirs.push((current.0 + 1, current.1));
                }
            }
            move_dirs.push((current.0, current.1 + y_dir));

            current = *move_dirs
                .iter()
                .filter(|dir| **dir != last)
                .choose(&mut rnd)
                .unwrap();
        }

        last = current;
        paths.insert(current);
    }

    paths
}

fn layer_path(paths: &mut HashSet<(i32, i32)>, seed: f64, width: i32, height: i32) {
    let perlin = Perlin::new();
    let mut prln_path = HashSet::new();
    let seed = if seed.is_sign_positive() { seed } else { -seed };

    // Generate perlin noise paths, as a rough
    // estimate of the dungeon
    for row in 1..=height {
        for col in 1..=width {
            let p = perlin.get([
                seed + row as f64 * PERLIN_SCALE.0,
                seed + col as f64 * PERLIN_SCALE.1,
            ]);
            if p >= PERLIN_THRESHOLD {
                prln_path.insert((col - 1, row - 1));
            }
        }
    }

    let mut added_to = true;

    while added_to {
        added_to = false;
        for path_seg in prln_path.clone().iter() {
            if paths.contains(&(path_seg.0 - 1, path_seg.1))
                || paths.contains(&(path_seg.0 + 1, path_seg.1))
                || paths.contains(&(path_seg.0, path_seg.1 + 1))
                || paths.contains(&(path_seg.0, path_seg.1 - 1))
            {
                paths.insert(*path_seg);
                prln_path.remove(&path_seg);
                added_to = true;
            }
        }
    }
}
