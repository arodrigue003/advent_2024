use crate::day20::astar::astar;
use crate::day20::models::{Map, Tile};
use hashbrown::HashMap;
use std::cmp::min;

pub fn find_shortcuts(map: &Map, scores: &[Vec<usize>], max_size: usize) -> usize {
    // Store found shortcuts
    let mut shortcuts: HashMap<usize, usize> = HashMap::new();

    // For each point in the grind, find it's shortcuts
    for y in 0..map.height {
        for x in 0..map.width {
            // Only considers floor tiles
            if map.grid[y][x] == Tile::Wall {
                continue;
            }

            let start_score = scores[y][x];

            // For every reachable point that has a max distance of max_size
            let y_min = if y < max_size { 0 } else { y - max_size };
            let x_min = if x < max_size { 0 } else { x - max_size };
            for y_end in y_min..min(y + max_size + 1, map.height) {
                for x_end in x_min..min(x + max_size + 1, map.width) {
                    let distance = y_end.abs_diff(y) + x_end.abs_diff(x);
                    if distance <= max_size && map.grid[y_end][x_end] == Tile::Floor {
                        let end_score = scores[y_end][x_end];
                        if start_score + distance < end_score {
                            // We found a shortcut
                            *shortcuts.entry(end_score - start_score - distance).or_default() += 1;
                        }
                    }
                }
            }
        }
    }

    shortcuts.iter().filter(|(save, _)| **save >= map.save_target).map(|(_, count)| count).sum()
}

pub fn solve_part_one(map: &Map) -> usize {
    let scores = astar(map).unwrap();

    find_shortcuts(map, &scores, 2)
}

pub fn solve_part_two(map: &Map) -> usize {
    let scores = astar(map).unwrap();

    find_shortcuts(map, &scores, 20)
}
