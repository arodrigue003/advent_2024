use crate::day17::models::Program;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{u64 as u64_parser, line_ending, u8 as u8_parser};
use nom::combinator::{map, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::tuple;
use nom::{IResult, Parser};

fn parse_register(input: &str) -> IResult<&str, u64> {
    map(
        tuple((
            tag("Register "),
            alt((tag("A"), tag("B"), tag("C"))),
            tag(": "),
            u64_parser,
            line_ending,
        )),
        |(_, _, _, value, _)| value,
    )
    .parse(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<u8>> {
    map(
        tuple((tag("Program: "), separated_list1(tag(","), u8_parser), opt(line_ending))),
        |(_, instructions, _)| instructions,
    )
    .parse(input)
}

fn parse_program(input: &str) -> IResult<&str, Program> {
    map(
        tuple((
            parse_register,
            parse_register,
            parse_register,
            many1(line_ending),
            parse_instructions,
        )),
        |(a, b, c, _, instructions)| Program::new(a, b, c, instructions),
    )
    .parse(input)
}

pub fn parse_input(input: String) -> Program {
    let (res, program) = parse_program.parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    program
}
