use std::collections::{HashMap, HashSet};

use day_15::{parse_input, Coords, Direction, INPUT};

fn main() {
    println!("Part 2: {}", part_2(INPUT));
}

fn part_2(input: &str) -> u32 {
    let (mut robot, walls, boxes, directions) = parse_input(input);

    robot = Coords {
        x: 2 * robot.x,
        ..robot
    };

    let walls = walls
        .into_iter()
        .flat_map(|Coords { x, y }| [Coords { x: 2 * x, y }, Coords { x: 2 * x + 1, y }])
        .collect::<HashSet<Coords>>();

    let mut boxes = boxes
        .into_iter()
        .flat_map(|Coords { x, y }| {
            [
                (Coords { x: 2 * x, y }, Coords { x: 2 * x + 1, y }),
                (Coords { x: 2 * x + 1, y }, Coords { x: 2 * x, y }),
            ]
        })
        .collect::<HashMap<Coords, Coords>>();

    for direction in directions {
        let next_pos = robot + direction;
        if walls.contains(&next_pos) {
            continue;
        }
        if boxes.contains_key(&next_pos) {
            let can_move = can_move_box(next_pos, direction, &walls, &boxes);
            if can_move {
                move_box(next_pos, direction, &mut boxes);
                robot = next_pos;
            }
        } else {
            robot = next_pos;
        }
    }

    boxes
        .into_iter()
        .map(
            |(Coords { x: x_a, y: y_a }, Coords { x: x_b, .. })| {
                if x_a < x_b {
                    x_a + 100 * y_a
                } else {
                    0
                }
            },
        )
        .sum()
}

fn can_move_box(
    coord: Coords,
    direction: Direction,
    walls: &HashSet<Coords>,
    boxes: &HashMap<Coords, Coords>,
) -> bool {
    let new_coord = coord + direction;
    let vertical = matches!(direction, Direction::Up | Direction::Down);

    if !vertical {
        return !walls.contains(&new_coord)
            && (!boxes.contains_key(&new_coord)
                || can_move_box(new_coord, direction, walls, boxes));
    }

    let second_pos = *boxes.get(&coord).unwrap();
    let second_new_coord = second_pos + direction;

    if walls.contains(&new_coord) || walls.contains(&second_new_coord) {
        return false;
    }

    if let Some(partner) = boxes.get(&new_coord) {
        if partner == &second_new_coord {
            return can_move_box(new_coord, direction, walls, boxes);
        }
    }

    if boxes.contains_key(&new_coord) && !can_move_box(new_coord, direction, walls, boxes) {
        return false;
    }

    if boxes.contains_key(&second_new_coord)
        && !can_move_box(second_new_coord, direction, walls, boxes)
    {
        return false;
    }

    true
}

fn move_box(coord: Coords, direction: Direction, boxes: &mut HashMap<Coords, Coords>) {
    let new_coord = coord + direction;
    let vertical = matches!(direction, Direction::Up | Direction::Down);

    let second_coord = *boxes.get(&coord).unwrap();
    let second_new_coord = second_coord + direction;

    if !vertical {
        if boxes.contains_key(&new_coord) {
            move_box(new_coord, direction, boxes);
        }
    } else {
        match boxes.get(&new_coord) {
            Some(partner) if partner == &second_new_coord => {
                move_box(new_coord, direction, boxes);
            }
            _ => {
                if boxes.contains_key(&new_coord) {
                    move_box(new_coord, direction, boxes);
                }

                if boxes.contains_key(&second_new_coord) {
                    move_box(second_new_coord, direction, boxes);
                }
            }
        }
    }

    boxes.remove(&coord);
    boxes.remove(&second_coord);

    boxes.insert(new_coord, second_new_coord);
    boxes.insert(second_new_coord, new_coord);
}

#[cfg(test)]
mod tests {
    use day_15::SAMPLE;

    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE), 9021);
    }
}
