use day_21::{find_shortest_sequence, INPUT};

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line[..line.len() - 1].parse::<usize>().unwrap() * find_shortest_sequence(line, 2)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use day_21::SAMPLE;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE), 126384);
    }
}
