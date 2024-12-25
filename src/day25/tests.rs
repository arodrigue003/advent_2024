use crate::common::{test_part_one_common, test_part_two_common};
use crate::dayxx::DayXX;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day25");

#[test]
fn test_part_one() {
    test_part_one_common(DayXX::default(), INPUT_EXAMPLE, 3);
}

#[test]
fn test_part_two() {
    test_part_two_common(DayXX::default(), INPUT_EXAMPLE, 0);
}
