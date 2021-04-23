use std::collections::HashSet;

use rpg_server::dungeons::inst::Dungeon;

#[test]
fn test_100() {
    fn is_dungeon_valid(dun: Dungeon) -> bool {
        let mut visited = HashSet::new();
        let mut next = Vec::new();

        next.push(dun.entrance);
        visited.insert(dun.entrance);

        while let Some(point) = next.pop() {
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
