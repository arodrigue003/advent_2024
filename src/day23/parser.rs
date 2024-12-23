use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending,};
use nom::combinator::{map, opt};
use nom::multi::{many1,};
use nom::sequence::tuple;
use nom::{IResult, Parser};

use crate::day23::models::ConnectionMap;

fn parse_connection(input: &str) -> IResult<&str, (String, String)> {
    map(
        tuple((alpha1, tag("-"), alpha1, opt(line_ending))),
        |(left, _, right, _): (&str, _, &str, _)| (left.to_string(), right.to_string()),
    )
    .parse(input)
}

pub fn parse_input(input: String) -> ConnectionMap {
    let (res, connections) = many1(parse_connection).parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    ConnectionMap { connections }
}
