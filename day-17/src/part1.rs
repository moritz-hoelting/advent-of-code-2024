use day_17::{parse_input, INPUT};

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &str) -> String {
    let mut device = parse_input(input);

    device
        .execute()
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use day_17::SAMPLE;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE), "4,6,3,5,6,3,5,2,1,0".to_string());
    }
}
