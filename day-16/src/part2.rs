use day_16::{find_start_end, successors, Direction, INPUT};
use itertools::Itertools;
use pathfinding::prelude::astar_bag;

fn main() {
    println!("Part 2: {}", part_2(INPUT));
}

fn part_2(input: &str) -> usize {
    let ((start_x, start_y), (end_x, end_y)) = find_start_end(input);

    astar_bag(
        &(start_x, start_y, Direction::East),
        successors(input),
        |_| 0,
        |(x, y, _)| x == &end_x && y == &end_y,
    )
    .unwrap()
    .0
    .flat_map(|path| path.into_iter().map(|(x, y, _)| (x, y)).collect::<Vec<_>>())
    .unique()
    .count()
}

#[cfg(test)]
mod tests {
    use day_16::{SAMPLE_1, SAMPLE_2};

    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(part_2(SAMPLE_1), 45);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part_2(SAMPLE_2), 64);
    }
}
