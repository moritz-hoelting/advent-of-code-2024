use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, line_ending, space1},
    combinator::{map, value},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair, terminated, tuple},
    IResult,
};

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
    And,
    Or,
    Xor,
}

#[expect(clippy::type_complexity)]
pub fn parse_input(input: &str) -> (HashMap<&str, bool>, HashMap<&str, (Operation, &str, &str)>) {
    let (_, (known, connections)) = separated_pair(
        parse_known,
        pair(line_ending, line_ending),
        parse_connections,
    )(input)
    .unwrap();

    (known, connections)
}

fn parse_known(input: &str) -> IResult<&str, HashMap<&str, bool>> {
    map(
        separated_list1(
            line_ending,
            separated_pair(
                alphanumeric1,
                tag(": "),
                alt((value(true, char('1')), value(false, char('0')))),
            ),
        ),
        |list| list.into_iter().collect(),
    )(input)
}

fn parse_connections(input: &str) -> IResult<&str, HashMap<&str, (Operation, &str, &str)>> {
    map(
        separated_list1(
            line_ending,
            tuple((
                terminated(alphanumeric1::<&str, Error<_>>, space1),
                alt((
                    value(Operation::And, tag("AND")),
                    value(Operation::Or, tag("OR")),
                    value(Operation::Xor, tag("XOR")),
                )),
                delimited(space1, alphanumeric1, tag(" -> ")),
                alphanumeric1,
            )),
        ),
        |list| {
            list.into_iter()
                .map(|(a, op, b, c)| (c, (op, a, b)))
                .collect()
        },
    )(input)
}
