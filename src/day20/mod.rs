mod logic;
mod models;
mod parser;

mod astar;
#[cfg(test)]
mod tests;

use crate::day20::astar::astar;
use crate::day20::logic::{find_shortcuts};
use crate::day20::models::Map;
use crate::day20::parser::parse_input;
use crate::models::AdventSolution;

#[derive(Default)]
pub struct Day20 {
    parsed_data: Option<Map>,
    scores: Option<Vec<Vec<usize>>>,
}

impl AdventSolution for Day20 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(parse_input(_data));
    }

    fn prepare(&mut self) {
        self.scores = astar(self.parsed_data.as_ref().unwrap())
    }

    fn solve_part_one(&self) -> i128 {
        find_shortcuts(self.parsed_data.as_ref().unwrap(), self.scores.as_ref().unwrap(), 2) as i128
    }

    fn solve_part_two(&self) -> i128 {
        find_shortcuts(self.parsed_data.as_ref().unwrap(), self.scores.as_ref().unwrap(), 20) as i128
    }
}
