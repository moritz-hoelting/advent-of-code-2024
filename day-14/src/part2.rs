use std::collections::HashSet;

use indicatif::ProgressIterator;

use day_14::{parse_input, Coords, Robot, INPUT};

fn main() {
    println!("Part 2: {}", part_2(INPUT, 101, 103));
}

fn part_2(input: &str, width: i64, height: i64) -> i64 {
    let dimensions = (width, height).into();
    let robots = parse_input(input);
    let (time, positions) = (1..100_000)
        .progress_count(100_000)
        .map(|s| {
            (
                s,
                robots
                    .iter()
                    .map(|Robot { position, velocity }| {
                        let mut pos = (*position + *velocity * s) % dimensions;
                        if pos.x < 0 {
                            pos.x += width;
                        }
                        if pos.y < 0 {
                            pos.y += height;
                        }
                        pos
                    })
                    .collect::<HashSet<_>>(),
            )
        })
        .find(|(_, positions)| {
            positions.iter().any(|Coords { x, y }| {
                (0..10).all(|i| positions.contains(&Coords { x: x + i, y: *y }))
            })
        })
        .unwrap();

    print_grid(&positions, width, height);

    time
}

fn print_grid(robots: &HashSet<Coords>, width: i64, height: i64) {
    for x in 0..width {
        for y in 0..height {
            let pos = Coords { x, y };
            if robots.contains(&pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
