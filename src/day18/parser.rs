use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, u128 as u128_parser};
use nom::combinator::{map};
use nom::multi::{many1};
use nom::sequence::tuple;
use nom::{IResult, Parser};

use crate::day18::models::Corruption;

fn parse_bytes(input: &str) -> IResult<&str, (usize, usize)> {
    map(
        tuple((u128_parser, tag(","), u128_parser, line_ending)),
        |(x, _, y, _)| (x as usize, y as usize),
    )
    .parse(input)
}

pub fn parse_input(input: String) -> Corruption {
    let (res, bytes) = many1(parse_bytes).parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    Corruption::new(bytes)
}
