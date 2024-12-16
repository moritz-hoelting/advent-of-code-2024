use indoc::indoc;

pub const SAMPLE_1: &str = indoc!(
    "
    ###############
    #.......#....E#
    #.#.###.#.###.#
    #.....#.#...#.#
    #.###.#####.#.#
    #.#.#.......#.#
    #.#.#####.###.#
    #...........#.#
    ###.#.#####.#.#
    #...#.....#.#.#
    #.#.#.###.#.#.#
    #.....#...#.#.#
    #.###.#.#.#.#.#
    #S..#.....#...#
    ###############
    "
);

pub const SAMPLE_2: &str = indoc!(
    "
    #################
    #...#...#...#..E#
    #.#.#.#.#.#.#.#.#
    #.#.#.#...#...#.#
    #.#.#.#.###.#.#.#
    #...#.#.#.....#.#
    #.#.#.#.#.#####.#
    #.#...#.#.#.....#
    #.#.#####.#.###.#
    #.#.#.......#...#
    #.#.###.#####.###
    #.#.#...#.....#.#
    #.#.#.#####.###.#
    #.#.#.........#.#
    #.#.#.#########.#
    #S#.............#
    #################
    "
);

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn rotate(self) -> [Self; 2] {
        match self {
            Self::North | Self::South => [Self::West, Self::East],
            Self::West | Self::East => [Self::North, Self::South],
        }
    }

    pub fn step(self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Self::North => y.checked_sub(1).map(|y| (x, y)),
            Self::South => y.checked_add(1).map(|y| (x, y)),
            Self::West => x.checked_sub(1).map(|x| (x, y)),
            Self::East => x.checked_add(1).map(|x| (x, y)),
        }
    }
}

pub fn find_start_end(input: &str) -> ((usize, usize), (usize, usize)) {
    let start_y = input
        .lines()
        .enumerate()
        .find(|(_, line)| line.contains('S'))
        .unwrap()
        .0;
    let start_x = input.lines().nth(start_y).unwrap().find('S').unwrap();

    let end_y = input
        .lines()
        .enumerate()
        .find(|(_, line)| line.contains('E'))
        .unwrap()
        .0;
    let end_x = input.lines().nth(end_y).unwrap().find('E').unwrap();

    ((start_x, start_y), (end_x, end_y))
}

pub fn successors(
    input: &str,
) -> impl use<'_> + Fn(&(usize, usize, Direction)) -> Vec<((usize, usize, Direction), u32)> {
    move |(x, y, direction)| {
        let x = *x;
        let y = *y;
        let direction = *direction;

        let step = direction.step(x, y).and_then(|(nx, ny)| {
            if input
                .lines()
                .nth(ny)
                .and_then(|line| line.chars().nth(nx))
                .is_some_and(|c| c == '.' || c == 'E')
            {
                Some(((nx, ny, direction), 1))
            } else {
                None
            }
        });

        direction
            .rotate()
            .into_iter()
            .map(move |direction| ((x, y, direction), 1000))
            .chain(step)
            .collect()
    }
}
