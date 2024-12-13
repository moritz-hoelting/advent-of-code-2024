use indoc::indoc;
use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending, one_of},
    combinator::{map, recognize},
    multi::{many1, separated_list1},
    sequence::{pair, tuple},
    IResult,
};

pub const SAMPLE: &str = indoc!(
    "
    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279
    "
);

pub const INPUT: &str = include_str!("../input.txt");

pub const COST_A: i64 = 3;
pub const COST_B: i64 = 1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClawMachine {
    pub button_a: Coords,
    pub button_b: Coords,
    pub prize: Coords,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords {
    pub x: i64,
    pub y: i64,
}

impl From<(i64, i64)> for Coords {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

pub fn parse_input(input: &str) -> Vec<ClawMachine> {
    separated_list1(
        pair(line_ending, line_ending),
        map(
            tuple((
                parse_button('A'),
                line_ending,
                parse_button('B'),
                line_ending,
                parse_prize,
            )),
            |(button_a, _, button_b, _, prize)| ClawMachine {
                button_a,
                button_b,
                prize,
            },
        ),
    )(input)
    .unwrap()
    .1
}

fn parse_button(button: char) -> impl Fn(&str) -> IResult<&str, Coords> {
    move |input| {
        map(
            tuple((
                tag("Button "),
                char::<&str, nom::error::Error<_>>(button),
                tag(": X+"),
                parse_int,
                tag(", Y+"),
                parse_int,
            )),
            |(_, _, _, x, _, y)| (x, y).into(),
        )(input)
    }
}

fn parse_prize(input: &str) -> IResult<&str, Coords> {
    map(
        tuple((tag("Prize: X="), parse_int, tag(", Y="), parse_int)),
        |(_, x, _, y)| (x, y).into(),
    )(input)
}

fn parse_int(input: &str) -> IResult<&str, i64> {
    map(recognize(many1(one_of("0123456789"))), |s: &str| {
        s.parse::<i64>().unwrap()
    })(input)
}

pub fn button_presses_required(
    ClawMachine {
        button_a,
        button_b,
        prize,
    }: ClawMachine,
) -> Option<(i64, i64)> {
    let numerator = (prize.x * button_b.y) - (prize.y * button_b.x);
    let denominator = button_a.x * button_b.y - button_a.y * button_b.x;

    if denominator == 0 {
        panic!("Divide by zero")
    } else if numerator % denominator == 0 {
        let a = numerator / denominator;
        let b_numerator = prize.x - (a * button_a.x);

        (b_numerator % button_b.x == 0).then_some((a, b_numerator / button_b.x))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        assert_eq!(parse_int("123"), Ok(("", 123)));
    }

    #[test]
    fn test_parse_prize() {
        assert_eq!(
            parse_prize("Prize: X=8400, Y=5400"),
            Ok(("", (8400, 5400).into()))
        );
    }

    #[test]
    fn test_parse_button() {
        assert_eq!(
            parse_button('A')("Button A: X+94, Y+34"),
            Ok(("", (94, 34).into()))
        );
        assert_eq!(
            parse_button('B')("Button B: X+22, Y+67"),
            Ok(("", (22, 67).into()))
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(indoc::indoc!(
                "
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176
            "
            )),
            vec![
                ClawMachine {
                    button_a: (94, 34).into(),
                    button_b: (22, 67).into(),
                    prize: (8400, 5400).into(),
                },
                ClawMachine {
                    button_a: (26, 66).into(),
                    button_b: (67, 21).into(),
                    prize: (12748, 12176).into(),
                },
            ]
        );
    }

    #[test]
    fn test_button_presses_required() {
        assert_eq!(
            button_presses_required(ClawMachine {
                button_a: (94, 34).into(),
                button_b: (22, 67).into(),
                prize: (8400, 5400).into(),
            }),
            Some((80, 40))
        );
    }
}
