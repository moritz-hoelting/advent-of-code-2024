use std::collections::{BTreeSet, HashMap, VecDeque};

use indoc::indoc;
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};

pub const SAMPLE: &str = indoc!(
    "
    ###############
    #...#...#.....#
    #.#.#.#.#.###.#
    #S#...#.#.#...#
    #######.#.#.###
    #######.#.#...#
    #######.#.###.#
    ###..E#...#...#
    ###.#######.###
    #...###...#...#
    #.#####.#.###.#
    #.#...#.#.#...#
    #.#.#.#.#.#.###
    #...#...#...###
    ###############
    "
);

pub const INPUT: &str = include_str!("../input.txt");

type Coords = (usize, usize);

pub fn solve(input: &str, min_save: usize, cheat_time: usize) -> usize {
    let start = input
        .lines()
        .enumerate()
        .find_map(|(y, l)| l.contains('S').then(|| (l.find('S').unwrap(), y)))
        .unwrap();
    let end = input
        .lines()
        .enumerate()
        .find_map(|(y, l)| l.contains('E').then(|| (l.find('E').unwrap(), y)))
        .unwrap();

    let (no_cheat_path, distances_to_end) = find_regular_path(input, end, start);

    no_cheat_path
        .into_par_iter()
        .map(|cheat_at| check_cheat_at(cheat_at, cheat_time, &distances_to_end, min_save))
        .sum()
}

fn find_regular_path(
    input: &str,
    (start_x, start_y): (usize, usize),
    (end_x, end_y): (usize, usize),
) -> (Vec<Coords>, HashMap<Coords, usize>) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut queue = VecDeque::new();
    queue.push_back(((start_x, start_y), 0, (start_x, start_y)));

    let mut predecessors = HashMap::new();
    let mut times = HashMap::new();

    let mut found = false;

    while let Some(((x, y), t, pred)) = queue.pop_front() {
        if predecessors.contains_key(&(x, y)) {
            continue;
        }
        predecessors.insert((x, y), pred);
        assert!(times.insert((x, y), t).is_none());
        if (x, y) == (end_x, end_y) {
            found = true;
            break;
        }

        let next = [
            x.checked_sub(1).map(|xn| (xn, y)),
            y.checked_sub(1).map(|yn| (x, yn)),
            (x < width).then_some((x + 1, y)),
            (y < height).then_some((x, y + 1)),
        ];

        queue.extend(next.into_iter().flatten().filter_map(|(nx, ny)| {
            if input.lines().nth(ny).and_then(|l| l.chars().nth(nx)) != Some('#') {
                Some(((nx, ny), t + 1, (x, y)))
            } else {
                None
            }
        }));
    }

    if found {
        let mut path = vec![(end_x, end_y)];
        let mut current = (end_x, end_y);
        while current != (start_x, start_y) {
            current = *predecessors.get(&current).unwrap();
            path.push(current);
        }
        path.reverse();
        (path, times)
    } else {
        panic!("No path found")
    }
}

fn check_cheat_at(
    (x, y): (usize, usize),
    cheat_time: usize,
    distances_to_end: &HashMap<Coords, usize>,
    min_save: usize,
) -> usize {
    let distance_cheat_start = distances_to_end[&(x, y)];

    let targets = coords_with_manhattan_distance((x, y), cheat_time);

    targets
        .into_iter()
        .filter_map(|((x, y), cheat_dist)| {
            distances_to_end
                .get(&(x, y))
                .and_then(|&d| distance_cheat_start.checked_sub(d + cheat_dist))
        })
        .filter(|&d| d >= min_save)
        .count()
}

fn coords_with_manhattan_distance((x, y): Coords, distance: usize) -> BTreeSet<(Coords, usize)> {
    let mut result = BTreeSet::new();
    for dx in 0..=distance {
        for dy in 0..=(distance - dx) {
            let distance = dx + dy;
            result.insert(((x + dx, y + dy), distance));
            if let Some(yn) = y.checked_sub(dy) {
                result.insert(((x + dx, yn), distance));
            }
            if let Some(xn) = x.checked_sub(dx) {
                result.insert(((xn, y + dy), distance));

                if let Some(yn) = y.checked_sub(dy) {
                    result.insert(((xn, yn), distance));
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_manhattan_distance_1() {
        let coords = (3, 3);
        let distance = 1;
        let expected = vec![
            (coords, 0),
            ((2, 3), 1),
            ((3, 2), 1),
            ((4, 3), 1),
            ((3, 4), 1),
        ]
        .into_iter()
        .collect::<HashSet<_>>();
        let result = coords_with_manhattan_distance(coords, distance)
            .into_iter()
            .collect::<HashSet<_>>();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_manhattan_distance_2() {
        let coords = (3, 3);
        let distance = 2;
        let expected = vec![
            (coords, 0),
            ((2, 3), 1),
            ((3, 2), 1),
            ((4, 3), 1),
            ((3, 4), 1),
            ((2, 2), 2),
            ((1, 3), 2),
            ((2, 4), 2),
            ((3, 5), 2),
            ((4, 4), 2),
            ((5, 3), 2),
            ((4, 2), 2),
            ((3, 1), 2),
        ]
        .into_iter()
        .collect::<HashSet<_>>();
        let result = coords_with_manhattan_distance(coords, distance)
            .into_iter()
            .collect::<HashSet<_>>();
        assert_eq!(result, expected);
    }
}
