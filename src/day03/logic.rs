use regex::Regex;

pub fn solve_part_one(data: &str) -> u32
{
    let mut score: u32 = 0;

    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for line in data.lines() {
        for (_, [left, right]) in mul_regex.captures_iter(line).map(|c|c.extract()) {
            score += left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
        }
    }

    score
}

pub fn solve_part_two(_data: &str) -> u32 {
    let mut score: u32 = 0;
    let mut is_enabled = true;

    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    for line in _data.lines() {
        for capture in mul_regex.captures_iter(line) {
            match &capture[0] {
                "do()" => is_enabled = true,
                "don't()" => is_enabled = false,
                _ => if is_enabled {
                    score += capture[1].parse::<u32>().unwrap() * capture[2].parse::<u32>().unwrap();
                }
            }
        }
    }

    score
}
