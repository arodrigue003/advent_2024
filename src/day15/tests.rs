use crate::common::{test_part_one_common, test_part_two_common};
use crate::day15::Day15;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day15");

#[test]
fn test_part_one() {
    test_part_one_common(Day15::default(), INPUT_EXAMPLE, 10092);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day15::default(), INPUT_EXAMPLE, 0);
}
