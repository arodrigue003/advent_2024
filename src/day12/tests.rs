use crate::common::{test_part_one_common, test_part_two_common};
use crate::day12::Day12;

static INPUT_EXAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

#[test]
fn test_part_one() {
    test_part_one_common(Day12::default(), INPUT_EXAMPLE, 1930);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day12::default(), INPUT_EXAMPLE, 0);
}
