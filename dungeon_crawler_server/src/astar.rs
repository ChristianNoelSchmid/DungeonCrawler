//! A* Pathfinding Implementation on Square Map
//!
//! Christian Schmid, May 2021

use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::state::transform::Transform;

const POS_TO_CONSIDER: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

///
/// A wrapper around (u32, u32), with cost
/// has Ord implemented.
///
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Path {
    pos: (i32, i32),
    cost: u32,
}

impl Ord for Path {
    // Implement Ord in the reverse direction to
    // ensure the binary heap pops off the min value,
    // rather than the max
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other // other being compared, rather than self
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

///
/// Finds the shortest path from `start` to `end`.
/// Requires a collection of `paths`, and a collection
/// of the currently `filled_spots` on the `paths`.
///
pub fn find_shortest_path(
    paths: &HashSet<(i32, i32)>,
    filled_spots: &HashSet<(i32, i32)>,
    start: (i32, i32),
    end: (i32, i32),
) -> Vec<(i32, i32)> {
    if !paths.contains(&start) || !paths.contains(&end) {
        return vec![start];
    }

    // The true distances from start point to path key
    let mut dist_map = HashMap::<(i32, i32), u32>::new();
    // A map of connecting previous positions, for rebuilding
    // the shortest path afterwards
    let mut prev_pos = HashMap::<(i32, i32), (i32, i32)>::new();
    let mut last_pos_cons = start;

    // A binary heap (priority queue)
    // storing the heuristic distance
    // between the start and all considered points
    let mut queue = BinaryHeap::new();

    dist_map.insert(start, 0);
    queue.push(Path {
        pos: start,
        cost: 0,
    });

    // While there are still paths to be considered
    while let Some(u) = queue.pop() {
        // Set last considered position 
        // to u position
        last_pos_cons = u.pos;

        // If end has been found, finished!
        if u.pos == end {
            break;
        }

        for path in POS_TO_CONSIDER.iter() {
            let new_pos = Transform::add_pos(u.pos, *path);
            if paths.contains(&new_pos) && (!filled_spots.contains(&new_pos) || new_pos == end) {
                let new_cost = dist_map[&u.pos] + 1;

                if let Some(cost) = dist_map.get(&new_pos) {
                    if *cost <= new_cost {
                        continue;
                    }
                }

                dist_map.insert(new_pos, new_cost);
                prev_pos.insert(new_pos, u.pos);
                
                // Compute the A* heuristic, and apply to heap.
                // Multiply by 1000 to maintain decimal difference between
                // two similar f32s converted to u32s
                queue.push(Path {
                    pos: new_pos,
                    cost: (new_cost as f32 + (Transform::distance(new_pos, end)) * 1000.0) as u32,
                });
            }
        }
    }

    let mut shortest_path = Vec::new();
    if last_pos_cons != end {
        last_pos_cons = *dist_map.keys().min_by(|p, p2| Transform::distance(**p, end).partial_cmp(&Transform::distance(**p2, end)).unwrap()).unwrap();
    }

    while last_pos_cons != start {
        shortest_path.push(last_pos_cons);
        last_pos_cons = prev_pos[&last_pos_cons];
    }

    shortest_path
}
