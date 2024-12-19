use crate::common::{test_part_one_common, test_part_two_common};
use crate::day18::Day18;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day18");

#[test]
fn test_part_one() {
    test_part_one_common(Day18::default(), INPUT_EXAMPLE, 0);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day18::default(), INPUT_EXAMPLE, 0);
}
