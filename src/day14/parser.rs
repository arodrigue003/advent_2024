use nom::bytes::complete::tag;
use nom::character::complete::{i64 as i64_parser, line_ending};
use nom::combinator::{map, opt};
use nom::multi::{many1};
use nom::sequence::tuple;
use nom::{IResult, Parser};

use crate::day14::models::{Bathroom, Robot};

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    map(
        tuple((
            tag("p="),
            i64_parser,
            tag(","),
            i64_parser,
            tag(" v="),
            i64_parser,
            tag(","),
            i64_parser,
            opt(line_ending),
        )),
        |(_, x, _, y, _, vx, _, vy, _)| Robot { x, y, vx, vy },
    )
    .parse(input)
}

pub fn parse_input(input: String) -> Bathroom {
    let (res, robots) = many1(parse_robot).parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    // Compute bathroom size according to robot initial positions.
    // This allows us to detect test case from real case
    if robots.iter().all(|robot| robot.x < 11 && robot.y < 7) {
        Bathroom {
            robots,
            width: 11,
            height: 7,
        }
    } else {
        Bathroom {
            robots,
            width: 101,
            height: 103,
        }
    }
}
