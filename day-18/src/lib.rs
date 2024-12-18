use std::collections::{HashSet, VecDeque};

use indoc::indoc;

pub const SAMPLE: &str = indoc!(
    "
    5,4
    4,2
    4,5
    3,0
    2,1
    6,3
    2,4
    1,5
    0,6
    3,3
    2,6
    5,1
    1,2
    5,5
    2,5
    6,5
    1,4
    0,4
    6,4
    1,1
    6,1
    1,0
    0,5
    1,6
    2,0
    "
);

pub const INPUT: &str = include_str!("../input.txt");

pub fn find_path(max_coord: u32, obstacles: &HashSet<(u32, u32)>) -> Option<u32> {
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));
    let mut seen = HashSet::new();

    while let Some(((x, y), t)) = queue.pop_front() {
        if seen.contains(&(x, y)) {
            continue;
        }

        seen.insert((x, y));
        if (x, y) == (max_coord, max_coord) {
            return Some(t);
        }

        let next = [
            x.checked_sub(1).map(|xn| (xn, y)),
            y.checked_sub(1).map(|yn| (x, yn)),
            (x < max_coord).then_some((x + 1, y)),
            (y < max_coord).then_some((x, y + 1)),
        ];

        queue.extend(
            next.into_iter()
                .flatten()
                .filter(|coords| !obstacles.contains(coords))
                .map(|coords| (coords, t + 1)),
        );
    }

    None
}
