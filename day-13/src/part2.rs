use day_13::{button_presses_required, parse_input, ClawMachine, Coords, COST_A, COST_B, INPUT};

fn main() {
    println!("Part 2: {}", part_2(INPUT));
}

const OFFSET: i64 = 10_000_000_000_000;

fn part_2(input: &str) -> i64 {
    parse_input(input)
        .into_iter()
        .map(|m| ClawMachine {
            prize: Coords {
                x: m.prize.x + OFFSET,
                y: m.prize.y + OFFSET,
            },
            ..m
        })
        .filter_map(button_presses_required)
        .map(|(a, b)| (a * COST_A) + (b * COST_B))
        .sum()
}

#[cfg(test)]
mod tests {
    use day_13::SAMPLE;

    use super::*;

    #[test]
    fn test_part2() {
        let possible = parse_input(SAMPLE)
            .into_iter()
            .map(|m| ClawMachine {
                prize: Coords {
                    x: m.prize.x + OFFSET,
                    y: m.prize.y + OFFSET,
                },
                ..m
            })
            .map(|m| button_presses_required(m).is_some())
            .collect::<Vec<_>>();
        assert_eq!(possible, vec![false, true, false, true]);
    }
}
