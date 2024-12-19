use crate::day19::models::Onsen;
use itertools::Itertools;
use regex::Regex;

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

pub fn count_combinations_rec(available_towels: &[String], design: &str, cache: &mut [usize], pos: usize) -> usize {
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
    for towel in available_towels {
        if pos + towel.len() <= design.len() {
            if &design[pos..pos + towel.len()] == towel {
                result += count_combinations_rec(available_towels, design, cache, pos + towel.len());
            }
        }
    }

    // Store the result
    cache[pos] = result;

    result
}

pub fn count_combinations(available_towels: &[String], design: &str) -> usize {
    // Initialize the cache
    let mut cache = vec![usize::MAX; design.len()];

    count_combinations_rec(available_towels, design, &mut cache, 0)
}

pub fn solve_part_two(onsen: &Onsen) -> usize {
    onsen
        .target_designs
        .iter()
        .map(|design| count_combinations(&onsen.available_towels, design))
        .sum()
}
