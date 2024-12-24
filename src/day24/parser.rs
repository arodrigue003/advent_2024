use crate::day24::models::{Gate, Operation, System, Wire};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1, line_ending};
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::tuple;
use nom::{IResult, Parser};

fn parse_wire(input: &str) -> IResult<&str, Wire> {
    map(
        tuple((alphanumeric1, tag(": "), digit1, opt(line_ending))),
        |(name, _, value, _): (&str, _, &str, _)| Wire {
            name: name.to_string(),
            value: if value == "1" { true } else { false },
        },
    )
    .parse(input)
}

fn parse_gate(input: &str) -> IResult<&str, Gate> {
    map(
        tuple((
            alphanumeric1,
            tag(" "),
            alt((tag("AND"), tag("XOR"), tag("OR"))),
            tag(" "),
            alphanumeric1,
            tag(" -> "),
            alphanumeric1,
            opt(line_ending),
        )),
        |(left, _, operation, _, right, _, result, _): (&str, _, &str, _, &str, _, &str, _)| Gate {
            left: left.to_string(),
            operation: Operation::from(operation),
            right: right.to_string(),
            result: result.to_string(),
        },
    )
    .parse(input)
}

fn parse_system(input: &str) -> IResult<&str, System> {
    map(
        tuple((many1(parse_wire), many1(line_ending), many1(parse_gate))),
        |(wires, _, gates)| System { wires, gates },
    )
    .parse(input)
}

pub fn parse_input(input: String) -> System {
    let (res, system) = parse_system.parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    system
}
