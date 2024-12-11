use day_11::{blink_n_times, INPUT};

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &str) -> u64 {
    blink_n_times(input, 25).into_values().sum()
}

#[cfg(test)]
mod tests {
    use day_11::SAMPLE;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE), 55312);
    }
}
