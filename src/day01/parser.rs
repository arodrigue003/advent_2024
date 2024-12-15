use nom::character::complete::{i64 as i64_parser, line_ending, space1};
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::tuple;
use nom::{IResult, Parser};

fn parse_locations(input: &str) -> IResult<&str, (i64, i64)> {
    map(
        tuple((i64_parser, space1, i64_parser, opt(line_ending))),
        |(left, _, right, _)| (left, right),
    )
    .parse(input)
}

pub fn parse_input(input: String) -> (Vec<i64>, Vec<i64>) {
    let (res, locations) = many1(parse_locations).parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    locations.into_iter().unzip()
}
