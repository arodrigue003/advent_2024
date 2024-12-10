use crate::day10::models::MapWithBorder;
use std::collections::HashSet;

// fn compute_trail_score(map: &MapWithBorder, cache: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
//     // First check if we have a result in the cache
//     if cache[y][x] != u32::MAX {
//         return cache[y][x];
//     }
//
//     // Get the map current value
//     let value = map.grid[y][x];
//
//     // If we are on a 9, we are done, set the cache and return 1
//     if value == 9 {
//         cache[y][x] = 1;
//         return 1;
//     };
//
//     // Get the result from every direction that respect the hiking condition
//     let mut result = 0;
//     // look up
//     if map.grid[y - 1][x] == value + 1 {
//         result += compute_trail_score(map, cache, x, y - 1);
//     }
//     // Look right
//     if map.grid[y][x + 1] == value + 1 {
//         result += compute_trail_score(map, cache, x + 1, y);
//     }
//     // Look down
//     if map.grid[y + 1][x] == value + 1 {
//         result += compute_trail_score(map, cache, x, y + 1);
//     }
//     // Look left
//     if map.grid[y][x - 1] == value + 1 {
//         result += compute_trail_score(map, cache, x - 1, y);
//     }
//
//     // Set the cache with the result
//     cache[y][x] = result;
//
//     result
// }

fn compute_trail_score(
    map: &MapWithBorder,
    cache: &mut Vec<Vec<Option<HashSet<(usize, usize)>>>>,
    x: usize,
    y: usize,
) -> usize {
    // First check if we have a result in the cache
    if let Some(cached_value) = &cache[y][x] {
        return cached_value.len();
    }

    // Get the map current value
    let value = map.grid[y][x];

    // If we are on a 9, we are done, set the cache and return 1
    if value == 9 {
        cache[y][x] = Some([(x, y)].into_iter().collect());
        return 1;
    };

    // Get the result from every direction that respect the hiking condition
    let mut end_set = HashSet::new();
    // look up
    if map.grid[y - 1][x] == value + 1 {
        compute_trail_score(map, cache, x, y - 1);
        // We know that the hashset for the neighbor is now defined by construction of the algorithm
        end_set = end_set.union(cache[y-1][x].as_ref().unwrap()).cloned().collect();
    }
    // Look right
    if map.grid[y][x + 1] == value + 1 {
        compute_trail_score(map, cache, x + 1, y);
        // We know that the hashset for the neighbor is now defined by construction of the algorithm
        end_set = end_set.union(cache[y][x+1].as_ref().unwrap()).cloned().collect();
    }
    // Look down
    if map.grid[y + 1][x] == value + 1 {
        compute_trail_score(map, cache, x, y + 1);
        // We know that the hashset for the neighbor is now defined by construction of the algorithm
        end_set = end_set.union(cache[y+1][x].as_ref().unwrap()).cloned().collect();
    }
    // Look left
    if map.grid[y][x - 1] == value + 1 {
        compute_trail_score(map, cache, x - 1, y);
        // We know that the hashset for the neighbor is now defined by construction of the algorithm
        end_set = end_set.union(cache[y][x-1].as_ref().unwrap()).cloned().collect();
    }

    // Set the cache with the result
    let result = end_set.len();
    cache[y][x] = Some(end_set);

    // Return the result
    result
}

pub fn solve_part_one(map: &MapWithBorder) -> usize {
    // Create a cache for the parkour
    let mut cache = vec![vec![None; map.width]; map.height];

    // Iterate over the inner map to compute the total score
    let mut result = 0;
    for y in 1..(map.width - 1) {
        for x in 1..(map.height - 1) {
            if map.grid[y][x] == 0 {
                result += compute_trail_score(&map, &mut cache, x, y);
            }
        }
    }

    result
}

pub fn solve_part_two(data: &MapWithBorder) -> usize {
    0
}
