use day_21::{find_shortest_sequence, INPUT};

fn main() {
    println!("Part 2: {}", part_2(INPUT));
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line[..line.len() - 1].parse::<usize>().unwrap() * find_shortest_sequence(line, 25)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use day_21::SAMPLE;

    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE), 154_115_708_116_294);
    }
}
