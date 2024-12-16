use crate::day05::models::ManualUpdates;
use hashbrown::{HashMap, HashSet};
use nom::bytes::complete::tag;
use nom::character::complete::{i32 as i32_parser, line_ending};
use nom::combinator::{map};
use nom::multi::{many1, separated_list1};
use nom::sequence::tuple;
use nom::{IResult, Parser};

type ParsedManualUpdates = (Vec<(i32, i32)>, Vec<Vec<i32>>);

fn parse_rule(input: &str) -> IResult<&str, (i32, i32)> {
    map(
        tuple((i32_parser, tag("|"), i32_parser, line_ending)),
        |(left, _, right, _)| (left, right),
    )
    .parse(input)
}

fn parse_update(input: &str) -> IResult<&str, Vec<i32>> {
    map(
        tuple((separated_list1(tag(","), i32_parser), line_ending)),
        |(list, _)| list,
    )
    .parse(input)
}

fn parse_manual_updates(input: &str) -> IResult<&str, ParsedManualUpdates> {
    map(
        tuple((many1(parse_rule), line_ending, many1(parse_update))),
        |(rules, _, updates)| (rules, updates),
    )
    .parse(input)
}

pub fn parse_input(input: String) -> ManualUpdates {
    let (res, (rules_vec, updates)) = parse_manual_updates.parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    // Put rules in a hashmap for easier access later
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (before, after) in rules_vec {
        rules.entry(before).or_default().insert(after);
    }

    ManualUpdates { rules, updates }
}
