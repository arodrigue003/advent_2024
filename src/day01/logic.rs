use std::collections::HashMap;

pub fn solve_part_one(data: &(Vec<i64>, Vec<i64>)) -> u64 {
    // Copy data
    let (mut left, mut right) = data.clone();

    // Sort vectors
    left.sort();
    right.sort();

    // Compute the result
    left.iter().zip(&right).map(|(l, r)| l.abs_diff(*r)).sum()
}

pub fn solve_part_two(data: &(Vec<i64>, Vec<i64>)) -> i64 {
    // Put the right list in a hashmap
    let mut right: HashMap<i64, i64> = HashMap::new();

    // fill it
    for l in &data.1 {
        *right.entry(*l).or_default() += 1;
    }

    // Perform the computation
    data.0.iter().map(|l| right.get(l).cloned().unwrap_or(0) * l).sum()
}
