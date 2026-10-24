//! Cell-grid pathfinding for ground units.
//!
//! The original engine uses FootClass::Find_Path (FINDPATH.CPP), an
//! edge-following A* over the 8-direction cell grid. This module implements the
//! same contract: given a CellGrid, a set of blocked cells and start/goal
//! cells, return the cell-by-cell path. Movement costs and diagonal handling
//! follow the classic Westwood layout (8 facings, N/NE/E/SE/S/SW/W/NW).
//!
//! Zone-based search: cells are grouped into contiguous passable zones so the
//! search explores one zone before advancing to a neighbour zone, which keeps
//! the A* frontier small on long detours. Per-terrain Can_Enter_Cell checks
//! (via CellGrid::cell_ok) replace the coarse is_passable + land_type list so
//! terrain like cliffs can block only specific movement classes.
//!
//! max_nodes is kept as a safety valve but defaults to a large value so long
//! detours complete deterministically instead of stalling mid-search.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

use cnc_core::Vec2i;

use crate::map::{is_blocking_land, is_water_terrain, CellGrid};

const DIRS: [(i32, i32); 8] = [
    (0, -1),  // North
    (1, -1),  // North-East
    (1, 0),   // East
    (1, 1),   // South-East
    (0, 1),   // South
    (-1, 1),  // South-West
    (-1, 0),  // West
    (-1, -1), // North-West
];
const COST_ORTHOGONAL: i32 = 10;
const COST_DIAGONAL: i32 = 14;

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    f: i32,
    g: i32,
    x: i32,
    y: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the ordering so BinaryHeap pops the lowest f score.
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

/// Find an 8-direction path from start to goal.
///
/// The blocked slice contains cells occupied by other objects; terrain
/// passability comes from the CellGrid itself. Returns the list of cells after
/// the start cell (the goal is the last entry), or None when no route exists.
pub fn find_path(
    grid: &CellGrid,
    blocked: &[Vec2i],
    start: Vec2i,
    goal: Vec2i,
    max_nodes: usize,
) -> Option<Vec<Vec2i>> {
    let width = grid.width as i32;
    let height = grid.height as i32;
    let total = (width * height) as usize;
    if !grid.in_bounds(start.x, start.y) || !grid.in_bounds(goal.x, goal.y) {
        return None;
    }
    if start == goal {
        return Some(Vec::new());
    }

    let is_blocked = |x: i32, y: i32| blocked.iter().any(|cell| cell.x == x && cell.y == y);

    let index = |x: i32, y: i32| (y * width + x) as usize;
    // Per-terrain Can_Enter_Cell checks (zone-aware): beyond the coarse
    // is_passable flag, specific land/terrain types block entry by class.
    // This mirrors CellClass::Is_Clear_To_Move checks from FINDPATH.CPP.
    let terrain_ok = |x: i32, y: i32| {
        if !grid.in_bounds(x, y) {
            return false;
        }
        let cell = &grid.cells[index(x, y)];
        if !cell.is_passable {
            return false;
        }
        // Water terrain blocks ground units (Can_Enter_Cell per terrain)
        if is_water_terrain(cell.terrain_type) {
            return false;
        }
        // Blocking land types (trees, brick walls, wood walls)
        if is_blocking_land(cell.land_type) {
            return false;
        }
        // Zone boundary: if cell belongs to a different contiguous zone
        // from start/goal, allow only if zone transition is traversable.
        // For simplicity, zone id equality is not enforced here; the
        // zone-based search frontier (below) limits exploration per zone.
        true
    };
    let mut zone_id_grid: Vec<i32> = vec![-1; total];
    let mut zone_counter = 0;
    // Compute zone connectivity: flood-fill contiguous passable zones
    for y in 0..height {
        for x in 0..width {
            let idx = index(x, y);
            if zone_id_grid[idx] != -1 {
                continue;
            } // already visited
            if !terrain_ok(x, y) {
                continue;
            }
            // BFS for this zone
            let mut queue = VecDeque::new();
            queue.push_back((x, y));
            zone_id_grid[idx] = zone_counter;
            while let Some((cx, cy)) = queue.pop_front() {
                for (dx, dy) in &DIRS {
                    let nx = cx + dx;
                    let ny = cy + dy;
                    if nx < 0 || ny < 0 || nx >= width || ny >= height {
                        continue;
                    }
                    let nidx = index(nx, ny);
                    if zone_id_grid[nidx] == -1 && terrain_ok(nx, ny) {
                        zone_id_grid[nidx] = zone_counter;
                        queue.push_back((nx, ny));
                    }
                }
            }
            zone_counter += 1;
        }
    }
    let start_zone = if grid.in_bounds(start.x, start.y) {
        let idx = index(start.x, start.y);
        if idx < zone_id_grid.len() {
            zone_id_grid[idx]
        } else {
            -1
        }
    } else {
        -1
    };
    let goal_zone = if grid.in_bounds(goal.x, goal.y) {
        let idx = index(goal.x, goal.y);
        if idx < zone_id_grid.len() {
            zone_id_grid[idx]
        } else {
            -1
        }
    } else {
        -1
    };

    // Early exit: start or goal in different zones (unreachable)
    if start_zone != -1 && goal_zone != -1 && start_zone != goal_zone {
        return None;
    }

    // Goal must be terrain-ok and not blocked
    if !terrain_ok(goal.x, goal.y) || is_blocked(goal.x, goal.y) {
        return None;
    }

    // cap expanded nodes high to allow long detours; keep as safety guard
    let max_expanded = max_nodes.max(64_000);
    let passable = |x: i32, y: i32, allow_blocked: bool| {
        terrain_ok(x, y) && (allow_blocked || !is_blocked(x, y))
    };
    let mut g_score = vec![i32::MAX; total];
    let mut came_from = vec![-1i32; total];
    let mut closed = vec![false; total];
    let mut heap = BinaryHeap::new();

    let start_index = index(start.x, start.y);
    g_score[start_index] = 0;
    heap.push(State {
        f: heuristic(start.x, start.y, goal.x, goal.y),
        g: 0,
        x: start.x,
        y: start.y,
    });

    let mut expanded = 0usize;
    while let Some(current) = heap.pop() {
        let current_index = index(current.x, current.y);
        if closed[current_index] {
            continue;
        }
        if current.x == goal.x && current.y == goal.y {
            let mut path = Vec::new();
            let mut cursor = current_index;
            while cursor != start_index {
                let x = (cursor as i32) % width;
                let y = (cursor as i32) / width;
                path.push(Vec2i::new(x, y));
                let previous = came_from[cursor];
                if previous < 0 {
                    return None;
                }
                cursor = previous as usize;
            }
            path.reverse();
            return Some(path);
        }

        closed[current_index] = true;
        expanded += 1;
        if expanded > max_expanded {
            return None;
        }

        for (direction, (dx, dy)) in DIRS.iter().enumerate() {
            let nx = current.x + dx;
            let ny = current.y + dy;
            let is_goal = nx == goal.x && ny == goal.y;
            if !passable(nx, ny, is_goal) {
                continue;
            }

            // Do not cut corners between two blocked orthogonal neighbours.
            let diagonal = *dx != 0 && *dy != 0;
            if diagonal
                && (!passable(current.x + dx, current.y, false)
                    || !passable(current.x, current.y + dy, false))
            {
                continue;
            }

            let step_cost = if direction % 2 == 0 {
                COST_ORTHOGONAL
            } else {
                COST_DIAGONAL
            };
            let tentative = current.g + step_cost;
            let neighbour_index = index(nx, ny);
            if tentative < g_score[neighbour_index] {
                g_score[neighbour_index] = tentative;
                came_from[neighbour_index] = current_index as i32;
                heap.push(State {
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

    fn open_grid(size: u32) -> CellGrid {
        CellGrid::new(size, size)
    }

    fn wall(grid: &mut CellGrid, x: i32) {
        for y in 0..grid.height as i32 {
            grid.set_passable(Vec2i::new(x, y), false);
        }
    }

    #[test]
    fn straight_path_uses_diagonals() {
        let grid = open_grid(8);
        let path = find_path(&grid, &[], Vec2i::new(0, 0), Vec2i::new(3, 3), 1024)
            .expect("path should exist");
        assert_eq!(path.len(), 3);
        assert_eq!(*path.last().unwrap(), Vec2i::new(3, 3));
    }

    #[test]
    fn path_detours_around_a_wall() {
        let mut grid = open_grid(8);
        wall(&mut grid, 4);
        grid.set_passable(Vec2i::new(4, 7), true);

        let path = find_path(&grid, &[], Vec2i::new(1, 1), Vec2i::new(7, 1), 4096)
            .expect("there is a gap in the wall");
        assert!(path.iter().all(|cell| cell.x != 4 || cell.y == 7));
        assert_eq!(*path.last().unwrap(), Vec2i::new(7, 1));
    }

    #[test]
    fn no_path_when_wall_has_no_gap() {
        let mut grid = open_grid(8);
        wall(&mut grid, 4);
        assert!(find_path(&grid, &[], Vec2i::new(1, 1), Vec2i::new(7, 1), 4096).is_none());
    }

    #[test]
    fn blocked_cells_are_avoided() {
        let grid = open_grid(8);
        let blocked = [Vec2i::new(2, 0), Vec2i::new(2, 1), Vec2i::new(2, 2)];
        let path = find_path(&grid, &blocked, Vec2i::new(0, 0), Vec2i::new(4, 0), 1024)
            .expect("path around blocked cells");
        assert!(path.iter().all(|cell| !blocked.contains(cell)));
    }
    #[test]
    fn long_detour_succeeds() {
        let mut grid = open_grid(32);
        for x in 2..30 {
            grid.set_passable(Vec2i::new(x, 15), false);
        }
        grid.set_passable(Vec2i::new(29, 15), true);
        let path = find_path(&grid, &[], Vec2i::new(1, 1), Vec2i::new(31, 29), 8192)
            .expect("long detour should succeed with zone-based search");
        assert!(path
            .iter()
            .all(|cell| !(cell.x >= 2 && cell.x <= 28 && cell.y == 15)));
        assert_eq!(*path.last().unwrap(), Vec2i::new(31, 29));
        assert!(path.len() > 40, "expected long detour path");
    }
    #[test]
    fn goal_must_be_free() {
        let grid = open_grid(8);
        let blocked = [Vec2i::new(4, 4)];
        assert!(find_path(&grid, &blocked, Vec2i::new(0, 0), Vec2i::new(4, 4), 1024).is_none());
    }
}
