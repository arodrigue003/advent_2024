use crate::common::{test_part_one_common, test_part_two_common};
use crate::day10::Day10;

static INPUT_EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

#[test]
fn test_part_one() {
    test_part_one_common(Day10::default(), INPUT_EXAMPLE, 36);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day10::default(), INPUT_EXAMPLE, 81);
}
