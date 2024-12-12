use std::collections::BTreeSet;

use day_12::INPUT;

fn main() {
    println!("Part 2: {}", part_2(INPUT));
}

fn part_2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let height = lines.len() as isize;
    let width = lines[0].len() as isize;

    let mut regions = Vec::new();
    let mut used = vec![vec![false; height as usize]; width as usize];

    for (x, y) in (0..width).flat_map(|x| (0..height).map(move |y| (x, y))) {
        if used[x as usize][y as usize] {
            continue;
        }
        let mut q = vec![(x, y)];
        // let mut area = 0;
        let mut region = BTreeSet::new();
        // let mut edges = HashSet::new();
        let c = lines[y as usize].chars().nth(x as usize).unwrap();
        while let Some((x, y)) = q.pop() {
            if used[x as usize][y as usize] {
                continue;
            }
            region.insert((x, y));
            used[x as usize][y as usize] = true;
            // area += 1;
            let c = lines[y as usize].chars().nth(x as usize).unwrap();

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                let (nx, ny) = (x + dx, y + dy);
                if !(0..width).contains(&nx) || !(0..height).contains(&ny) {
                    continue;
                }
                let nc = lines[ny as usize].chars().nth(nx as usize).unwrap();
                if nc == c {
                    q.push((nx, ny));
                }
            }
        }
        regions.push((c, region));
    }
    regions
        .iter()
        .map(|(_, region)| region.len() * edge_amount(region))
        .sum()
}

fn edge_amount(region: &BTreeSet<(isize, isize)>) -> usize {
    let corner_candidates = region
        .iter()
        .flat_map(|(x, y)| {
            [
                (2 * x, 2 * y),
                (2 * x + 2, 2 * y),
                (2 * x + 2, 2 * y + 2),
                (2 * x, 2 * y + 2),
            ]
        })
        .collect::<BTreeSet<_>>();

    let mut corners = 0;

    for (cx, cy) in corner_candidates.into_iter() {
        let config = [
            (cx / 2 - 1, cy / 2 - 1),
            (cx / 2, cy / 2 - 1),
            (cx / 2, cy / 2),
            (cx / 2 - 1, cy / 2),
        ]
        .iter()
        .map(|(sx, sy)| region.contains(&(*sx, *sy)))
        .enumerate()
        .fold([false; 4], |mut acc, (i, val)| {
            acc[i] = val;
            acc
        });

        let number = config.iter().filter(|b| **b).count();

        if number == 1 || number == 3 {
            corners += 1;
        } else if number == 2
            && (config == [true, false, true, false] || config == [false, true, false, true])
        {
            corners += 2;
        }
    }
    corners
}

#[cfg(test)]
mod tests {
    use day_12::{SAMPLE_1, SAMPLE_2, SAMPLE_3};
    use indoc::indoc;

    use super::*;

    const SAMPLE_4: &str = indoc!(
        "
        EEEEE
        EXXXX
        EEEEE
        EXXXX
        EEEEE
        "
    );

    const SAMPLE_5: &str = indoc!(
        "
        AAAAAA
        AAABBA
        AAABBA
        ABBAAA
        ABBAAA
        AAAAAA
        "
    );

    #[test]
    fn test_part2_1() {
        assert_eq!(part_2(SAMPLE_1), 80);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part_2(SAMPLE_2), 436);
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(part_2(SAMPLE_3), 1206);
    }

    #[test]
    fn test_part2_4() {
        assert_eq!(part_2(SAMPLE_4), 236);
    }

    #[test]
    fn test_part2_5() {
        assert_eq!(part_2(SAMPLE_5), 368);
    }
}
