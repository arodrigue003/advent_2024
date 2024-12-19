use crate::day19::models::Onsen;
use hashbrown::HashSet;
use itertools::Itertools;
use regex::Regex;
use std::cmp::min;

pub fn solve_part_one(onsen: &Onsen) -> usize {
    // Create a regex that can match
    let towels_regex = Regex::new(&format!(r"^(?:{})*$", onsen.available_towels.iter().join("|"))).unwrap();

    // Match designs
    onsen
        .target_designs
        .iter()
        .filter(|target| towels_regex.is_match(target))
        .count()
}

pub fn count_combinations_rec(
    available_towels: &HashSet<String>,
    design: &str,
    min_size: usize,
    max_size: usize,
    cache: &mut [usize],
    pos: usize,
) -> usize {
    // Return the default value if we are done with the recursion
    if pos >= design.len() {
        return 1;
    }

    // Return the cached value if available
    if cache[pos] != usize::MAX {
        return cache[pos];
    }

    // Store the result
    let mut result = 0;

    // Iter over towels to see if one of them can be used
    for i in min_size..=min(max_size, design.len() - pos) {
        if available_towels.contains(&design[pos..pos + i]) {
            result += count_combinations_rec(available_towels, design, min_size, max_size, cache, pos + i);
        }
    }

    // Store the result
    cache[pos] = result;

    result
}

pub fn count_combinations(available_towels: &HashSet<String>, design: &str, min_size: usize, max_size: usize) -> usize {
    // Initialize the cache
    let mut cache = vec![usize::MAX; design.len()];

    count_combinations_rec(available_towels, design, min_size, max_size, &mut cache, 0)
}

pub fn solve_part_two(onsen: &Onsen) -> usize {
    let min_size = onsen.available_towels.iter().map(|a| a.len()).min().unwrap();
    let max_size = onsen.available_towels.iter().map(|a| a.len()).max().unwrap();

    onsen
        .target_designs
        .iter()
        .map(|design| count_combinations(&onsen.available_towels_set, design, min_size, max_size))
        .sum()
}
