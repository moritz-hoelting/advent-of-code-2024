use day_16::{find_start_end, successors, Direction, INPUT};
use pathfinding::prelude::dijkstra;

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &str) -> u32 {
    let ((start_x, start_y), (end_x, end_y)) = find_start_end(input);

    dijkstra(
        &(start_x, start_y, Direction::East),
        successors(input),
        |(x, y, _)| x == &end_x && y == &end_y,
    )
    .unwrap()
    .1
}

#[cfg(test)]
mod tests {
    use day_16::{SAMPLE_1, SAMPLE_2};

    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(part_1(SAMPLE_1), 7036);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part_1(SAMPLE_2), 11048);
    }
}
