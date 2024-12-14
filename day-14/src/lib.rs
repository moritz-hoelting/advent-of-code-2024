use std::ops::{Add, Mul, Rem};

use indoc::indoc;
use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub const SAMPLE: &str = indoc!(
    "
    p=0,4 v=3,-3
    p=6,3 v=-1,-3
    p=10,3 v=-1,2
    p=2,0 v=2,-1
    p=0,0 v=1,3
    p=3,0 v=-2,-2
    p=7,6 v=-1,-3
    p=3,0 v=-1,-2
    p=9,3 v=2,3
    p=7,3 v=-1,2
    p=2,4 v=2,-3
    p=9,5 v=-3,-3
    "
);

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Robot {
    pub position: Coords,
    pub velocity: Coords,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords {
    pub x: i64,
    pub y: i64,
}

impl From<(i64, i64)> for Coords {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coords {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i64> for Coords {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Coords {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Rem for Coords {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Coords {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Robot> {
    separated_list1(
        line_ending,
        map(
            tuple((tag("p="), parse_coords, tag(" v="), parse_coords)),
            |(_, position, _, velocity)| Robot { position, velocity },
        ),
    )(input)
    .unwrap()
    .1
}

fn parse_coords(input: &str) -> IResult<&str, Coords> {
    map(
        tuple((
            nom::character::complete::i64,
            char(','),
            nom::character::complete::i64,
        )),
        |(x, _, y)| Coords { x, y },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coords() {
        assert_eq!(parse_coords("0,4"), Ok(("", Coords { x: 0, y: 4 })));
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(&SAMPLE.lines().take(2).collect::<Vec<_>>().join("\n")),
            vec![
                Robot {
                    position: Coords { x: 0, y: 4 },
                    velocity: Coords { x: 3, y: -3 }
                },
                Robot {
                    position: Coords { x: 6, y: 3 },
                    velocity: Coords { x: -1, y: -3 }
                }
            ]
        );
    }
}
