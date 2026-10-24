//! Cell-grid pathfinding for ground units.
//!
//! The original engine uses FootClass::Find_Path (FINDPATH.CPP), an
//! edge-following A* over the 8-direction cell grid. This module implements the
//! same contract: given a CellGrid, a set of blocked cells and start/goal
//! cells, return the cell-by-cell path. Movement costs and diagonal handling
//! follow the classic Westwood layout (8 facings, N/NE/E/SE/S/SW/W/NW).

use std::cmp::Ordering;
use std::collections::BinaryHeap;

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
    if !grid.in_bounds(start.x, start.y) || !grid.in_bounds(goal.x, goal.y) {
        return None;
    }

    let is_blocked = |x: i32, y: i32| blocked.iter().any(|cell| cell.x == x && cell.y == y);

    let index = |x: i32, y: i32| (y * width + x) as usize;
    let terrain_ok = |x: i32, y: i32| {
        if !grid.in_bounds(x, y) {
            return false;
        }
        let cell = &grid.cells[index(x, y)];
        // Base passability check
        if !cell.is_passable {
            return false;
        }
        // Water terrain blocks ground units
        if is_water_terrain(cell.terrain_type) {
            return false;
        }
        // Blocking land types (trees, brick walls, wood walls)
        if is_blocking_land(cell.land_type) {
            return false;
        }
        true
    };
    let passable = |x: i32, y: i32, allow_blocked: bool| {
        terrain_ok(x, y) && (allow_blocked || !is_blocked(x, y))
    };

    if !terrain_ok(goal.x, goal.y) || is_blocked(goal.x, goal.y) {
        return None;
    }
    if start == goal {
        return Some(Vec::new());
    }

    let total = (width * height) as usize;
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
        if expanded > max_nodes {
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
    fn goal_must_be_free() {
        let grid = open_grid(8);
        let blocked = [Vec2i::new(4, 4)];
        assert!(find_path(&grid, &blocked, Vec2i::new(0, 0), Vec2i::new(4, 4), 1024).is_none());
    }
}
