use crate::day18::astar::astar;
use crate::day18::models::Corruption;

pub fn solve_part_one(corruption_map: &Corruption) -> usize {
    astar(corruption_map, corruption_map.to_simulate).unwrap()
}

pub fn solve_part_two(corruption_map: &Corruption) -> usize {
    // We now that the corruption_map.to_simulate is valid, so we can use that as the start of our
    // dichotomy.
    let mut start = corruption_map.to_simulate;

    // We don't have to simulate more steps than there are damaged bytes in the list
    let mut end = corruption_map.bytes.len();

    while start + 1 != end {
        // Compute the middle value
        let middle = (start + end) / 2;

        // Try the middle value
        let is_possible = astar(corruption_map, middle).is_some();

        // Update the value depending on the result
        if is_possible {
            start = middle;
        } else {
            end = middle;
        }
    }

    // We take start that is equal to end - 1 because if we need to simulate i step to get a
    // failure, this is the i-1th byte that failed
    corruption_map.bytes[start].0 * 100 + corruption_map.bytes[start].1
}
