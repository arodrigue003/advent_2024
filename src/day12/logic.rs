use crate::day12::models::{MapWithBorder, Region};
use std::fmt::Display;

/// Visit a neighbor
fn visit_neighbor(
    map: &MapWithBorder,
    visited: &mut Vec<Vec<bool>>,
    plant_type: char,
    line: usize,
    column: usize,
) -> Region {
    let neighbor = map.grid[line][column];
    if neighbor != plant_type {
        // The region is terminated here, this mean we have a new border, add it to the score
        return Region::new(0, 1);
    };

    // Check up if we didn't do it yet
    if visited[line][column] {
        Region::new(0, 0)
    } else {
        // Visit the rest of the region
        visit_region_rec(map, visited, plant_type, line, column)
    }
}

/// Compute the size of a region.
///
/// Returns
/// * The tuple (area, perimeter_length)
fn visit_region_rec(
    map: &MapWithBorder,
    visited: &mut Vec<Vec<bool>>,
    plant_type: char,
    line: usize,
    column: usize,
) -> Region {
    // The stopping condition is on visits of neighbours, no need to have one here.
    // This ensures that the tile we are on is always a plant_type one.
    if visited[line][column] {
        return Region::new(0, 0);
    }

    // Mark the current tile as visited
    visited[line][column] = true;

    // Score accumulator, we start with an area of one for the tile we are standing on.
    let mut result = Region::new(1, 0);

    result += visit_neighbor(map, visited, plant_type, line - 1, column); // up
    result += visit_neighbor(map, visited, plant_type, line, column + 1); // right
    result += visit_neighbor(map, visited, plant_type, line + 1, column); // down
    result += visit_neighbor(map, visited, plant_type, line, column - 1); // left

    result
}

pub fn solve_part_one(map: &MapWithBorder) -> usize {
    let mut visited = vec![vec![false; map.width]; map.height];
    let mut total_cost = 0;
    for line in 1..map.height - 1 {
        for column in 1..map.width - 1 {
            let res = visit_region_rec(map, &mut visited, map.grid[line][column], line, column);
            // println!("{}:{line}:{column} => {res}", map.grid[line][column]);
            total_cost += res.cost();
        }
    }

    total_cost
}

pub fn solve_part_two(map: &MapWithBorder) -> usize {
    0
}
