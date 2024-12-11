use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, u128 as u128_parser};
use nom::combinator::{map, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::tuple;
use nom::{IResult, Parser};

use crate::day07::models::Equation;

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    map(
        tuple((u128_parser, tag(": "), separated_list1(tag(" "), u128_parser), opt(line_ending))),
        |(result, _, operands, _)| Equation { result, operands },
    )
    .parse(input)
}

pub fn parse_input(input: String) -> Vec<Equation> {
    let (res, games) = many1(parse_equation).parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    games
}
