use crate::common::{test_part_one_common, test_part_two_common};
use crate::dayxx::DayXX;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day03");
static INPUT_EXAMPLE_2: &str = include_str!("../../input_examples/day03_2");

#[test]
fn test_part_one() {
    test_part_one_common(DayXX::default(), INPUT_EXAMPLE, 161);
}

#[test]
fn test_part_two() {
    test_part_two_common(DayXX::default(), INPUT_EXAMPLE, 48);
}
