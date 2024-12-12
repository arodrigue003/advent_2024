use std::collections::HashMap;

fn get_stone_count_rec(value: usize, remaining_steps: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    // Check if the result was cached
    if let Some(result) = cache.get(&(value, remaining_steps)) {
        return *result;
    }

    // We are out of steps, return 1: the current stone
    if remaining_steps == 0 {
        return 1;
    }

    // We have a null value, set it to 1 and continue
    if value == 0 {
        return get_stone_count_rec(1, remaining_steps - 1, cache);
    }

    // Check the len of the integer
    let digit_count = value.ilog10() + 1;

    // Get the result from recursion
    let result = if digit_count % 2 == 0 {
        // we have an even number of digit, split it and return the result for each part
        get_stone_count_rec(value / 10usize.pow(digit_count / 2), remaining_steps - 1, cache)
            + get_stone_count_rec(value % 10usize.pow(digit_count / 2), remaining_steps - 1, cache)
    } else {
        get_stone_count_rec(value * 2024, remaining_steps - 1, cache)
    };

    // Update the cache
    cache.insert((value, remaining_steps), result);

    result
}

pub fn solve_part_one(data: &[usize]) -> usize {
    let mut cache = HashMap::new();

    data.iter()
        .map(|stone| get_stone_count_rec(*stone, 25, &mut cache))
        .sum()
}

pub fn solve_part_two(data: &[usize]) -> usize {
    let mut cache = HashMap::new();

    data.iter()
        .map(|stone| get_stone_count_rec(*stone, 75, &mut cache))
        .sum()
}
