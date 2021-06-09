//! Test for the DungeonGenerator
//!
//! Christian Schmid - June 2021
//! CS510 - Rust Programming

use std::collections::HashSet;

use dungeon_generator::inst::Dungeon;

/// Generates about 50 dungeons of different sizes, and
/// tests that each one successfully creates and entrance-
/// to-exit path. If any single dungeon does not do so,
/// the test fails. Performs a breadth-first-search
#[test]
fn test_100() {
    // Function for determining whether the entrance
    // eventually traverses to the exit
    fn is_dungeon_valid(dun: Dungeon) -> bool {
        // The list of paths already visited.
        let mut visited = HashSet::new();
        // The list of next paths to visit
        let mut next = Vec::new();

        // Push the entrance onto the stack
        next.push(dun.entrance);
        visited.insert(dun.entrance);

        // While all paths have not been searched,
        // take the current one and check its adjacent
        // neighbors.
        while let Some(point) = next.pop() {
            // If the current path is the exit, return true
            if point == dun.exit {
                return true;
            }
            for p in dun.paths() {
                if (point.1 == p.1 && (point.0 as i32 - p.0 as i32).abs() == 1)
                    || (point.0 == p.0 && (point.1 as i32 - p.1 as i32).abs() == 1)
                {
                    if !visited.contains(p) {
                        next.push(*p);
                        visited.insert(*p);
                    }
                }
            }
        }

        return false;
    }

    for i in (20..75).step_by(2) {
        assert_eq!(is_dungeon_valid(Dungeon::new(i, i)), true);
    }
}
