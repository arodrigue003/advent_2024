mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day17::logic::{solve_part_one, solve_part_two};
use crate::day17::models::Program;
use crate::day17::parser::parse_input;
use crate::models::AdventSolution;

#[derive(Default)]
pub struct Day17 {
    parsed_data: Option<Program>,
}

impl AdventSolution for Day17 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(parse_input(_data));
    }

    fn solve_part_one(&self) -> i128 {
        solve_part_one(self.parsed_data.as_ref().unwrap()) as i128
    }

    fn solve_part_two(&self) -> i128 {
        solve_part_two(self.parsed_data.as_ref().unwrap()) as i128
    }
}
