use day_12::INPUT;

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    let height = lines.len();
    let width = lines[0].len();

    let mut regions = Vec::new();
    let mut used = vec![vec![false; height]; width];

    for (x, y) in (0..width).flat_map(|x| (0..height).map(move |y| (x, y))) {
        if used[x][y] {
            continue;
        }
        let mut q = vec![(x, y)];
        let mut area = 0;
        let mut perimeter = 0;
        while let Some((x, y)) = q.pop() {
            if used[x][y] {
                continue;
            }
            used[x][y] = true;
            area += 1;
            let c = lines[y].chars().nth(x).unwrap();

            for (dx, dy) in [(0, 1), (2, 1), (1, 0), (1, 2)].iter() {
                if let Some((nx, ny)) = (x + dx)
                    .checked_sub(1)
                    .and_then(|nx| (y + dy).checked_sub(1).map(|ny| (nx, ny)))
                {
                    if nx >= width || ny >= height {
                        perimeter += 1;
                        continue;
                    }
                    let nc = lines[ny].chars().nth(nx).unwrap();
                    if nc == c {
                        q.push((nx, ny));
                    } else {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }
            }
        }
        regions.push((area, perimeter));
    }

    regions.iter().map(|(a, p)| a * p).sum()
}

#[cfg(test)]
mod tests {
    use day_12::{SAMPLE_1, SAMPLE_2, SAMPLE_3};

    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(part_1(SAMPLE_1), 140);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part_1(SAMPLE_2), 772);
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(part_1(SAMPLE_3), 1930);
    }
}
