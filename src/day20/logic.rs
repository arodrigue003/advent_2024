use crate::day20::models::{Map, Tile};
use std::cmp::min;

#[allow(clippy::needless_range_loop)]
pub fn find_shortcuts(map: &Map, scores: &[Vec<usize>], max_size: usize) -> usize {
    // Store found shortcuts
    let mut result = 0;

    // For each point in the grind, find it's shortcuts
    for y in 0..map.height {
        for x in 0..map.width {
            // Only considers floor tiles
            if map.grid[y][x] == Tile::Wall {
                continue;
            }

            let start_score = scores[y][x];

            // For every reachable point that has a max distance of max_size
            let y_min = y.saturating_sub(max_size);
            let x_min = x.saturating_sub(max_size);
            for y_end in y_min..min(y + max_size + 1, map.height) {
                for x_end in x_min..min(x + max_size + 1, map.width) {
                    let distance = y_end.abs_diff(y) + x_end.abs_diff(x);
                    let end_score = scores[y_end][x_end];
                    if distance <= max_size
                        && map.grid[y_end][x_end] == Tile::Floor
                        && map.save_target + start_score + distance <= end_score
                    {
                        // We found a shortcut we are interested in
                        result += 1;
                    }
                }
            }
        }
    }

    result
}
