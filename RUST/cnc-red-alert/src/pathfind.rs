//! Cell-grid A* pathfinding for Red Alert ground, naval and air units.
//!
//! Movement classes select the passability rule: ground units need land,
//! naval units need water, amphibious units traverse both, and aircraft ignore terrain.
//! The search itself is the classic 8-direction A* used by the Westwood engines.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use cnc_core::Vec2i;

use crate::map::Map;

const DIRS: [(i32, i32); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

const COST_ORTHOGONAL: i32 = 10;
const COST_DIAGONAL: i32 = 14;

/// Which cells a unit may traverse.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveClass {
    Ground,
    Naval,
    Air,
    Amphibious,
}

impl MoveClass {
    pub fn passable(self, map: &Map, cell: Vec2i) -> bool {
        match self {
            MoveClass::Ground => map.is_land(cell),
            MoveClass::Naval => map.is_water(cell),
            MoveClass::Air => map.in_bounds(cell),
            MoveClass::Amphibious => map.is_land(cell) || map.is_water(cell),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            MoveClass::Ground => "ground",
            MoveClass::Naval => "naval",
            MoveClass::Air => "air",
            MoveClass::Amphibious => "amphibious",
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    f: i32,
    g: i32,
    x: i32,
    y: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f
            .cmp(&self.f)
            .then_with(|| other.g.cmp(&self.g))
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(x: i32, y: i32, goal_x: i32, goal_y: i32) -> i32 {
    let dx = (x - goal_x).abs();
    let dy = (y - goal_y).abs();
    COST_ORTHOGONAL * (dx + dy) - (COST_ORTHOGONAL * 2 - COST_DIAGONAL) * dx.min(dy)
}

/// Find an 8-direction path for one movement class.
///
/// Returns the cells after the start cell (the goal is last), or None when
/// the goal is unreachable. Cells in the blocked slice are treated as occupied
/// terrain for ground and naval units; aircraft ignore them. Threat costs are
/// added to the path cost to prefer safer routes (Red Alert threat avoidance).
pub fn find_path(
    map: &Map,
    class: MoveClass,
    blocked: &[Vec2i],
    start: Vec2i,
    goal: Vec2i,
    threat_cost: &HashSet<(i32, i32)>,
) -> Option<Vec<Vec2i>> {
    if !map.in_bounds(start) || !map.in_bounds(goal) {
        return None;
    }
    if !class.passable(map, goal) {
        return None;
    }
    if start == goal {
        return Some(Vec::new());
    }

    let width = map.width as i32;
    let height = map.height as i32;
    let total = (width * height) as usize;
    let index = |x: i32, y: i32| (y * width + x) as usize;

    let blocked: HashSet<(i32, i32)> = if class == MoveClass::Air {
        HashSet::new()
    } else {
        blocked
            .iter()
            .filter(|cell| **cell != goal)
            .map(|cell| (cell.x, cell.y))
            .collect()
    };

    let mut open = BinaryHeap::new();
    let mut g_score = vec![i32::MAX; total];
    let mut came_from = vec![i32::MAX; total];
    let mut closed = vec![false; total];

    let start_index = index(start.x, start.y);
    g_score[start_index] = 0;
    open.push(State {
        f: heuristic(start.x, start.y, goal.x, goal.y),
        g: 0,
        x: start.x,
        y: start.y,
    });
    while let Some(State { x, y, g, .. }) = open.pop() {
        let current = index(x, y);
        if closed[current] {
            continue;
        }
        if x == goal.x && y == goal.y {
            let mut path = Vec::new();
            let mut node = current;
            while node != start_index {
                let px = (node as i32) % width;
                let py = (node as i32) / width;
                path.push(Vec2i::new(px, py));
                let previous = came_from[node];
                if previous == i32::MAX {
                    return None;
                }
                node = previous as usize;
            }
            path.reverse();
            return Some(path);
        }
        closed[current] = true;
        // No node cap - search exhaustively (max_nodes removed per parity-100 task 26)

        for (dx, dy) in DIRS {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || ny < 0 || nx >= width || ny >= height {
                continue;
            }
            let next = Vec2i::new(nx, ny);
            if !class.passable(map, next) || blocked.contains(&(nx, ny)) {
                continue;
            }
            let next_index = index(nx, ny);
            if closed[next_index] {
                continue;
            }
            let step = if dx != 0 && dy != 0 {
                COST_DIAGONAL
            } else {
                COST_ORTHOGONAL
            };
            // Add threat cost if this cell is in a danger zone
            let threat = if threat_cost.contains(&(nx, ny)) {
                50 // Threat penalty - makes pathfinder prefer safer routes
            } else {
                0
            };
            let tentative = g + step + threat;
            if tentative < g_score[next_index] {
                g_score[next_index] = tentative;
                came_from[next_index] = current as i32;
                open.push(State {
                    f: tentative + heuristic(nx, ny, goal.x, goal.y),
                    g: tentative,
                    x: nx,
                    y: ny,
                });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ground_path_stays_on_land_and_avoids_the_river() {
        let map = Map::new(24, 24);
        let start = Vec2i::new(4, 4);
        let goal = Vec2i::new(8, 10);
        let threat = HashSet::new();
        let path = find_path(&map, MoveClass::Ground, &[], start, goal, &threat).unwrap();
        assert_eq!(path.last().copied(), Some(goal));
        assert!(path.iter().all(|cell| map.is_land(*cell)));
    }

    #[test]
    fn naval_paths_follow_water_and_ground_cannot_enter_it() {
        let map = Map::new(24, 24);
        let start = Vec2i::new(12, 2);
        let goal = Vec2i::new(13, 20);
        let threat = HashSet::new();
        let path = find_path(&map, MoveClass::Naval, &[], start, goal, &threat).unwrap();
        assert_eq!(path.last().copied(), Some(goal));
        assert!(path.iter().all(|cell| map.is_water(*cell)));

        let start = Vec2i::new(4, 4);
        let goal = Vec2i::new(12, 4);
        assert!(
            find_path(&map, MoveClass::Ground, &[], start, goal, &threat).is_none(),
            "ground path must not end in the river"
        );
    }

    #[test]
    fn air_units_ignore_water_and_blocked_cells() {
        let map = Map::new(24, 24);
        let start = Vec2i::new(4, 4);
        let goal = Vec2i::new(20, 20);
        let blocked = vec![Vec2i::new(12, 12)];
        let threat = HashSet::new();
        let path = find_path(&map, MoveClass::Air, &blocked, start, goal, &threat).unwrap();
        assert_eq!(path.last().copied(), Some(goal));
        assert!(path.contains(&Vec2i::new(12, 12)));
    }

    #[test]
    fn blocked_cells_force_a_detour_or_failure() {
        let map = Map::new(16, 16);
        let start = Vec2i::new(3, 3);
        let goal = Vec2i::new(6, 3);
        let blocked = vec![Vec2i::new(5, 2), Vec2i::new(5, 3), Vec2i::new(5, 4)];
        let threat = HashSet::new();
        let path = find_path(&map, MoveClass::Ground, &blocked, start, goal, &threat).unwrap();
        assert!(!path.contains(&Vec2i::new(5, 3)));
        assert_eq!(path.last().copied(), Some(goal));
    }

    #[test]
    fn amphibious_crosses_land_and_water_in_one_path() {
        let map = Map::new(24, 24);
        // Start on land, goal in water - amphibious should cross the shore
        let start = Vec2i::new(4, 4);
        let goal = Vec2i::new(13, 20);
        let threat = HashSet::new();
        let path = find_path(&map, MoveClass::Amphibious, &[], start, goal, &threat).unwrap();
        assert_eq!(path.last().copied(), Some(goal));
        // Path should contain both land and water cells
        let has_land = path.iter().any(|cell| map.is_land(*cell));
        let has_water = path.iter().any(|cell| map.is_water(*cell));
        assert!(
            has_land && has_water,
            "amphibious path must cross both land and water"
        );
    }
}
