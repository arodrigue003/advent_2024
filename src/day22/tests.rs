use crate::common::{test_part_one_common, test_part_two_common};
use crate::day22::Day22;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day22");

#[test]
fn test_part_one() {
    test_part_one_common(Day22::default(), INPUT_EXAMPLE, 37327623);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day22::default(), INPUT_EXAMPLE, 0);
}
