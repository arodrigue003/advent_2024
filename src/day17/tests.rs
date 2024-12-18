use crate::common::{test_part_one_common, test_part_two_common};
use crate::day17::Day17;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day17");

#[test]
fn test_part_one() {
    test_part_one_common(Day17::default(), INPUT_EXAMPLE, 0);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day17::default(), INPUT_EXAMPLE, 0);
}
