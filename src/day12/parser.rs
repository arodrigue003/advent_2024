use crate::day12::models::MapWithBorder;

pub fn parse_input(input: String) -> MapWithBorder {
    MapWithBorder::new(input.lines().map(|line| line.chars().collect()).collect())
}
