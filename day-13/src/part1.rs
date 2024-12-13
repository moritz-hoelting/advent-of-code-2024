use day_13::{button_presses_required, parse_input, COST_A, COST_B, INPUT};

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &str) -> i64 {
    parse_input(input)
        .into_iter()
        .filter_map(button_presses_required)
        .map(|(a, b)| a * COST_A + b * COST_B)
        .sum()
}

#[cfg(test)]
mod tests {
    use day_13::SAMPLE;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE), 480);
    }
}
