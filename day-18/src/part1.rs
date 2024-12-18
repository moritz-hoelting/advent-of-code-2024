use std::collections::HashSet;

use day_18::{find_path, INPUT};

fn main() {
    println!("Part 1: {}", part_1(INPUT, 70, 1024));
}

fn part_1(input: &str, max_coord: u32, after_time: usize) -> u32 {
    let obstacles = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').expect("invalid input format");
            (
                x.parse().expect("invalid number"),
                y.parse().expect("invalid number"),
            )
        })
        .take(after_time)
        .collect::<HashSet<(u32, u32)>>();

    find_path(max_coord, &obstacles).unwrap()
}

#[cfg(test)]
mod tests {
    use day_18::SAMPLE;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE, 6, 12), 22);
    }
}
