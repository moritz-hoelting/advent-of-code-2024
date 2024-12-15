use std::{collections::HashSet, ops::Add};

use indoc::indoc;
use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::value,
    multi::{many1, separated_list1},
    sequence::{pair, separated_pair},
    IResult,
};

pub const SAMPLE: &str = indoc!(
    "
    ##########
    #..O..O.O#
    #......O.#
    #.OO..O.O#
    #..O@..O.#
    #O#..O...#
    #O..O..O.#
    #.OO.O.OO#
    #....O...#
    ##########

    <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "
);

pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> (Coords, HashSet<Coords>, HashSet<Coords>, Vec<Direction>) {
    let (_, ((robot, walls, boxes), directions)) =
        separated_pair(parse_grid, pair(line_ending, line_ending), parse_directions)(input)
            .unwrap();

    (robot, walls, boxes, directions)
}

fn parse_grid(input: &str) -> IResult<&str, (Coords, HashSet<Coords>, HashSet<Coords>)> {
    separated_list1(
        line_ending,
        many1(alt((
            value(Some(MapType::Robot), char('@')),
            value(Some(MapType::Box), char('O')),
            value(Some(MapType::Wall), char('#')),
            value(None, char('.')),
        ))),
    )(input)
    .map(|(rest, map)| {
        let mut robot = None;
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();

        for (y, row) in map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Some(MapType::Robot) => {
                        robot = Some(Coords {
                            x: x as u32,
                            y: y as u32,
                        })
                    }
                    Some(MapType::Wall) => {
                        walls.insert(Coords {
                            x: x as u32,
                            y: y as u32,
                        });
                    }
                    Some(MapType::Box) => {
                        boxes.insert(Coords {
                            x: x as u32,
                            y: y as u32,
                        });
                    }
                    _ => {}
                }
            }
        }

        (rest, (robot.unwrap(), walls, boxes))
    })
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(alt((
        value(Some(Direction::Down), char('v')),
        value(Some(Direction::Up), char('^')),
        value(Some(Direction::Left), char('<')),
        value(Some(Direction::Right), char('>')),
        value(None, line_ending),
    )))(input)
    .map(|(rest, directions)| (rest, directions.into_iter().flatten().collect()))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords {
    pub x: u32,
    pub y: u32,
}

impl From<(u32, u32)> for Coords {
    fn from((x, y): (u32, u32)) -> Self {
        Self { x, y }
    }
}

impl Add<Direction> for Coords {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Coords {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Coords {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Coords {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Coords {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MapType {
    Robot,
    Wall,
    Box,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_grid() {
        assert!(parse_grid(indoc!(
            "
            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########
            "
        ))
        .is_ok());
    }

    #[test]
    fn test_parse_directions() {
        assert!(parse_directions(indoc!(
            "
            <^^>>>vv<v>>v<<
            "
        ))
        .is_ok());
    }
}
