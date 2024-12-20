use day_20::{solve, INPUT};

fn main() {
    println!("Part 2: {}", part_2(INPUT, 100));
}

fn part_2(input: &str, min_save: usize) -> usize {
    solve(input, min_save, 20)
}

#[cfg(test)]
mod tests {
    use day_20::SAMPLE;

    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE, 50), 285);
    }
}
