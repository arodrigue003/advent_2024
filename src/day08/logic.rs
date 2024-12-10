use std::collections::HashSet;

use itertools::Itertools;

use crate::day08::models::Map;

pub fn solve_part_one(map: &Map) -> usize {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for antennas in map.antennas.values() {
        for combination in antennas.iter().combinations(2) {
            let left = combination[0];
            let right = combination[1];

            // Compute antinodes
            // 1. the positive one
            let a1_x = 2 * right.x - left.x;
            let a1_y = 2 * right.y - left.y;
            if a1_x >= 0 && a1_x < map.width && a1_y >= 0 && a1_y < map.height {
                antinodes.insert((a1_x, a1_y));
            }
            // 2. the negative one
            let a2_x = 2 * left.x - right.x;
            let a2_y = 2 * left.y - right.y;
            if a2_x >= 0 && a2_x < map.width && a2_y >= 0 && a2_y < map.height {
                antinodes.insert((a2_x, a2_y));
            }
        }
    }

    antinodes.len()
}

pub fn solve_part_two(map: &Map) -> usize {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for antennas in map.antennas.values() {
        for combination in antennas.iter().combinations(2) {
            let left = combination[0];
            let right = combination[1];

            // Compute left->right vector
            let v_x = right.x - left.x;
            let v_y = right.y - left.y;

            // Check every positive antinode
            let mut antinode = (right.x, right.y);
            while map.is_inside(&antinode) {
                antinodes.insert(antinode);
                antinode = (antinode.0 + v_x, antinode.1 + v_y);
            }

            // Check every negative antinode
            let mut antinode = (left.x, left.y);
            while map.is_inside(&antinode) {
                antinodes.insert(antinode);
                antinode = (antinode.0 - v_x, antinode.1 - v_y);
            }
        }
    }

    antinodes.len()
}
