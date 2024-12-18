mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day09::logic::{solve_part_one, solve_part_two};
use crate::day09::parser::parse_input;
use crate::models::AdventSolution;

#[derive(Default)]
pub struct Day09 {
    parsed_data: Vec<usize>,
}

impl AdventSolution for Day09 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = parse_input(_data);
    }

    fn solve_part_one(&self) -> i128 {
        solve_part_one(&self.parsed_data) as i128
    }

    fn solve_part_two(&self) -> i128 {
        solve_part_two(&self.parsed_data) as i128
    }
}
