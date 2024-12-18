use std::collections::HashSet;

use day_18::{find_path, INPUT};

fn main() {
    let (x, y) = part_2(INPUT, 70);
    println!("Part 2: {x},{y}");
}

fn part_2(input: &str, max_coord: u32) -> (u32, u32) {
    let falling_bytes = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').expect("invalid input format");
            (
                x.parse::<u32>().expect("invalid number"),
                y.parse::<u32>().expect("invalid number"),
            )
        })
        .collect::<Vec<_>>();

    bin_search(&falling_bytes, 1, falling_bytes.len(), max_coord)
}

fn bin_search(
    falling_bytes: &[(u32, u32)],
    start: usize,
    end: usize,
    max_coord: u32,
) -> (u32, u32) {
    if start == end {
        return *falling_bytes.get(start - 1).unwrap();
    }

    let idx = start + (end - start) / 2;
    let obstacles = falling_bytes
        .iter()
        .take(idx)
        .copied()
        .collect::<HashSet<_>>();

    let path_len = find_path(max_coord, &obstacles);

    if path_len.is_some() {
        bin_search(falling_bytes, idx + 1, end, max_coord)
    } else {
        bin_search(falling_bytes, start, idx, max_coord)
    }
}

#[cfg(test)]
mod tests {

    use day_18::SAMPLE;

    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE, 6), (6, 1));
    }
}
