use nom::bytes::complete::tag;
use nom::character::complete::{i64 as i64_parser, line_ending};
use nom::combinator::{map, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::tuple;
use nom::{IResult, Parser};

fn parse_locations(input: &str) -> IResult<&str, Vec<i64>> {
    map(
        tuple((separated_list1(tag(" "), i64_parser), opt(line_ending))),
        |(list, _)| list,
    )
    .parse(input)
}

pub fn parse_input(input: String) -> Vec<Vec<i64>> {
    let (res, reports) = many1(parse_locations).parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    reports
}
