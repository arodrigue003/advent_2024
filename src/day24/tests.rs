use crate::common::{test_part_one_common, test_part_two_common};
use crate::day24::Day24;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day24");

#[test]
fn test_part_one() {
    test_part_one_common(Day24::default(), INPUT_EXAMPLE, 2024);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day24::default(), INPUT_EXAMPLE, 0);
}
