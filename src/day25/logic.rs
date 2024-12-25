pub fn solve_part_one(data: &(Vec<u64>, Vec<u64>)) -> usize {
    data.0
        .iter()
        .map(|key| data.1.iter().filter(|lock| key & *lock == 0).count())
        .sum()
}

pub fn solve_part_two(_data: &(Vec<u64>, Vec<u64>)) -> u32 {
    0
}
