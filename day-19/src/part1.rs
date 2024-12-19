use day_19::INPUT;

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &str) -> usize {
    let (towels, designs) = input.split_once("\n\n").unwrap();

    let towels = towels.split(", ").collect::<Vec<_>>();
    designs
        .lines()
        .filter(|design| test_design(design, &towels))
        .count()
}

fn test_design(design: &str, towels: &[&str]) -> bool {
    if design.is_empty() {
        return true;
    }
    towels
        .iter()
        .any(|towel| design.starts_with(towel) && test_design(&design[towel.len()..], towels))
}

#[cfg(test)]
mod tests {
    use day_19::SAMPLE;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE), 6);
    }
}
