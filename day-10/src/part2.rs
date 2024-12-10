use std::collections::HashSet;

use day_10::INPUT;

fn main() {
    println!("Part 2: {}", part_2(INPUT));
}

fn part_2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let height = lines.len();
    let width = lines[0].len();

    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(
                move |(x, c)| {
                    if c == '0' {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        })
        .map(|(tx, ty)| {
            let mut trails = HashSet::new();
            let mut stack = vec![(tx, ty, vec![(tx, ty)])];
            while let Some((px, py, prev)) = stack.pop() {
                let pn = lines[py]
                    .chars()
                    .nth(px)
                    .and_then(|c| c.to_digit(10))
                    .unwrap();

                let a = py.checked_sub(1).map(|y| (px, y));
                let b = py.checked_add(1).map(|y| (px, y));
                let c = px.checked_sub(1).map(|x| (x, py));
                let d = px.checked_add(1).map(|x| (x, py));

                [a, b, c, d]
                    .into_iter()
                    .flatten()
                    .filter_map(|(x, y)| {
                        let valid_pos = (0..width).contains(&x) && (0..height).contains(&y);
                        if valid_pos {
                            let n = lines[y]
                                .chars()
                                .nth(x)
                                .and_then(|c| c.to_digit(10))
                                .unwrap();
                            let valid_height = n == pn + 1;
                            valid_height.then_some((x, y, n))
                        } else {
                            None
                        }
                    })
                    .for_each(|(x, y, n)| {
                        let mut path = prev.clone();
                        path.push((x, y));
                        if n == 9 {
                            trails.insert(path);
                        } else {
                            stack.push((x, y, path));
                        }
                    });
            }
            trails.len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use day_10::SAMPLE;

    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE), 81);
    }
}
