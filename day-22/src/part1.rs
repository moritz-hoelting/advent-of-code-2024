use day_22::{generate_secret_numbers, INPUT};

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            generate_secret_numbers(line.parse().unwrap())
                .nth(2000)
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const SAMPLE: &str = indoc::indoc!(
        "
        1
        10
        100
        2024
        "
    );

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE), 37_327_623);
    }
}
