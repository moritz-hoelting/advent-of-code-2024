use day_22::{generate_secret_numbers, INPUT};
use itertools::Itertools as _;

fn main() {
    println!("Part 2: {}", part_2(INPUT));
}

fn part_2(input: &str) -> u64 {
    input
        .lines()
        .flat_map(|line| {
            let secrets = generate_secret_numbers(line.parse().unwrap())
                .take(2000)
                .map(|n| (n % 10) as i8);

            changes(secrets)
                .tuple_windows::<(_, _, _, _)>()
                .map(|((_, da), (_, db), (_, dc), (d, dd))| ((da, db, dc, dd), d))
                .unique_by(|&(seq, _)| seq)
        })
        .sorted_by_key(|&(seq, _)| seq)
        .chunk_by(|&(seq, _)| seq)
        .into_iter()
        .map(|(_, group)| group.map(|(_, d)| d as u64).sum::<u64>())
        .max()
        .unwrap()
}

fn changes(iter: impl Iterator<Item = i8>) -> impl Iterator<Item = (i8, i8)> {
    iter.tuple_windows::<(_, _)>().map(|(a, b)| (b, b - a))
}

#[cfg(test)]
mod tests {
    use day_22::generate_secret_numbers;

    use super::*;

    pub const SAMPLE: &str = indoc::indoc!(
        "
        1
        2
        3
        2024
        "
    );

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE), 23);
    }

    #[test]
    fn test_ones_digit() {
        assert_eq!(
            generate_secret_numbers(123)
                .map(|n| n % 10)
                .take(10)
                .collect::<Vec<_>>(),
            vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2,]
        )
    }

    #[test]
    fn test_changes() {
        let secrets = generate_secret_numbers(123).map(|n| (n % 10) as i8);
        assert_eq!(
            changes(secrets).take(9).collect::<Vec<_>>(),
            vec![
                (0, -3),
                (6, 6),
                (5, -1),
                (4, -1),
                (4, 0),
                (6, 2),
                (4, -2),
                (4, 0),
                (2, -2)
            ]
        )
    }
}
