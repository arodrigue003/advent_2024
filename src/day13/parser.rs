use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{i64 as i64_parser, line_ending};
use nom::combinator::map;
use nom::multi::{many0, many1};
use nom::sequence::tuple;
use nom::{IResult, Parser};

use crate::day13::models::Machine;

fn parse_button(input: &str) -> IResult<&str, (i64, i64)> {
    map(
        tuple((
            alt((tag("Button A: X+"), tag("Button B: X+"))),
            i64_parser,
            tag(", Y+"),
            i64_parser,
            many0(line_ending),
        )),
        |(_, x, _, y, _)| (x, y),
    )
    .parse(input)
}

fn parse_prize(input: &str) -> IResult<&str, (i64, i64)> {
    map(
        tuple((
            tag("Prize: X="),
            i64_parser,
            tag(", Y="),
            i64_parser,
            many0(line_ending),
        )),
        |(_, x, _, y, _)| (x, y),
    )
    .parse(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    map(tuple((parse_button, parse_button, parse_prize)), |(a, b, target)| {
        Machine { a, b, target }
    })
    .parse(input)
}

pub fn parse_input(input: String) -> Vec<Machine> {
    let (res, games) = many1(parse_machine).parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    games
}
