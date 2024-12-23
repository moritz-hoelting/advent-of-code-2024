use std::collections::{BTreeSet, HashSet, VecDeque};

use day_23::{connections, parse_input, INPUT};
use itertools::Itertools;

fn main() {
    println!("Part 2: {}", part_2(INPUT));
}

fn part_2(input: &str) -> String {
    let parsed_input = parse_input(input);
    let connections = connections(parsed_input.iter().map(|&(a, b)| (a, b)));

    let mut computers = parsed_input
        .iter()
        .flat_map(|&(a, b)| [a, b])
        .collect::<BTreeSet<_>>();

    let mut connected_parts = Vec::new();

    while let Some(&start) = computers.first() {
        let mut connected_part = HashSet::new();
        let mut queue = VecDeque::from([start]);

        while let Some(next) = queue.pop_front() {
            if connected_part.contains(&next) {
                continue;
            }

            if connected_part
                .iter()
                .any(|&c| !connections.get(c).unwrap().contains(&next))
            {
                continue;
            }

            computers.remove(next);
            connected_part.insert(next);

            let next_connections = connections.get(next).unwrap();
            queue.extend(next_connections.iter().copied());
        }

        connected_parts.push(connected_part);
    }

    connected_parts
        .iter()
        .max_by_key(|connected| connected.len())
        .unwrap()
        .iter()
        .sorted()
        .join(",")
}

#[cfg(test)]
mod tests {
    use day_23::SAMPLE;

    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE), "co,de,ka,ta");
    }
}
