use day_20::{solve, INPUT};

fn main() {
    println!("Part 1: {}", part_1(INPUT, 100));
}

fn part_1(input: &str, min_save: usize) -> usize {
    solve(input, min_save, 2)
}

#[cfg(test)]
mod tests {
    use day_20::SAMPLE;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE, 10), 10);
    }
}
