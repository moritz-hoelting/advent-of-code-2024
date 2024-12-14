use day_14::{parse_input, Coords, Robot, INPUT};

fn main() {
    println!("Part 1: {}", part_1(INPUT, 101, 103));
}

const SECONDS: i64 = 100;

fn part_1(input: &str, width: i64, height: i64) -> i64 {
    let dimensions = (width, height).into();
    let q_width = width / 2;
    let q_height = height / 2;
    parse_input(input)
        .into_iter()
        .map(|Robot { position, velocity }| {
            let mut pos = (position + velocity * SECONDS) % dimensions;
            if pos.x < 0 {
                pos.x += width;
            }
            if pos.y < 0 {
                pos.y += height;
            }
            pos
        })
        .fold([0; 4], |mut acc, Coords { x, y }| {
            if x < q_width && y < q_height {
                acc[0] += 1;
            } else if x > q_width && y < q_height {
                acc[1] += 1;
            } else if x < q_width && y > q_height {
                acc[2] += 1;
            } else if x > q_width && y > q_height {
                acc[3] += 1;
            }

            acc
        })
        .into_iter()
        .product()
}

#[cfg(test)]
mod tests {
    use day_14::SAMPLE;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE, 11, 7), 12);
    }
}
