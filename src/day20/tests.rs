use crate::common::{test_part_one_common, test_part_two_common};
use crate::day20::Day20;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day20");

#[test]
fn test_part_one() {
    test_part_one_common(Day20::default(), INPUT_EXAMPLE, 5);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day20::default(), INPUT_EXAMPLE, 2268);
}
