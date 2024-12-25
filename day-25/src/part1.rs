use day_25::{parse_input, INPUT};
use itertools::Itertools as _;

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &str) -> usize {
    let (keys, locks) = parse_input(input);

    keys.iter()
        .cartesian_product(locks.iter())
        .filter(|&(key, lock)| key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 5))
        .count()
}

#[cfg(test)]
mod tests {
    use day_25::SAMPLE;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE), 3);
    }
}
