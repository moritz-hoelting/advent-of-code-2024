use day_23::{connections, parse_input, INPUT};
use itertools::Itertools as _;

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &str) -> usize {
    let parsed_input = parse_input(input);
    let connections = connections(parsed_input.iter().map(|&(a, b)| (a, b)));

    parsed_input
        .iter()
        .flat_map(|&(a, b)| [a, b])
        .filter(|c| c.starts_with('t'))
        .flat_map(|c| {
            connections
                .get(c)
                .unwrap()
                .iter()
                .tuple_combinations::<(_, _)>()
                .filter_map(|(&a, &b)| {
                    if connections
                        .get(a)
                        .map(|n| n.contains(b))
                        .unwrap_or_default()
                    {
                        let mut x = [a, b, c];
                        x.sort();
                        let [a, b, c] = x;

                        Some((a, b, c))
                    } else {
                        None
                    }
                })
        })
        .unique()
        .count()
}

#[cfg(test)]
mod tests {
    use day_23::SAMPLE;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE), 7);
    }
}
