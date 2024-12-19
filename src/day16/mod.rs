mod logic;
mod models;
mod parser;

mod astar;
#[cfg(test)]
mod tests;

use hashbrown::HashSet;

use crate::day16::logic::prepare_data;
use crate::day16::models::Map;
use crate::day16::parser::parse_input;
use crate::models::AdventSolution;

#[derive(Default)]
pub struct Day16 {
    parsed_data: Option<Map>,
    prepared_data: Option<(i32, HashSet<(usize, usize)>)>,
}

impl AdventSolution for Day16 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(parse_input(_data));
    }

    fn prepare(&mut self) {
        self.prepared_data = Some(prepare_data(self.parsed_data.as_ref().unwrap()));
    }

    fn solve_part_one(&self) -> i128 {
        self.prepared_data.as_ref().unwrap().0 as i128
    }

    fn solve_part_two(&self) -> i128 {
        self.prepared_data.as_ref().unwrap().1.len() as i128
    }
}
