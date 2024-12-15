use day_15::{parse_input, Coords, INPUT};

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &str) -> u32 {
    let (mut robot, walls, mut boxes, directions) = parse_input(input);

    for direction in directions {
        let next_pos = robot + direction;
        let mut final_pos = next_pos;
        while boxes.contains(&final_pos) {
            final_pos = final_pos + direction;
        }
        if walls.contains(&final_pos) {
            continue;
        }
        robot = next_pos;
        if boxes.remove(&next_pos) {
            boxes.insert(final_pos);
        }
    }

    boxes.into_iter().map(|Coords { x, y }| x + 100 * y).sum()
}

#[cfg(test)]
mod tests {
    use day_15::SAMPLE as SAMPLE_1;

    use indoc::indoc;

    use super::*;

    const SAMPLE_2: &str = indoc!(
        "
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########
    
        <^^>>>vv<v>>v<<
        "
    );

    #[test]
    fn test_part1_1() {
        assert_eq!(part_1(SAMPLE_1), 10092);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part_1(SAMPLE_2), 2028);
    }
}
