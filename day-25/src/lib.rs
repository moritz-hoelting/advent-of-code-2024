use indoc::indoc;

pub const SAMPLE: &str = indoc!(
    "
    #####
    .####
    .####
    .####
    .#.#.
    .#...
    .....

    #####
    ##.##
    .#.##
    ...##
    ...#.
    ...#.
    .....

    .....
    #....
    #....
    #...#
    #.#.#
    #.###
    #####

    .....
    .....
    #.#..
    ###..
    ###.#
    ###.#
    #####

    .....
    .....
    .....
    #....
    #.#..
    #.#.#
    #####
    "
);

pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> (Vec<[u32; 5]>, Vec<[u32; 5]>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for part in input.split("\n\n") {
        if part.starts_with('#') {
            let mut lock = [0; 5];

            for (i, lock_entry) in lock.iter_mut().enumerate() {
                let mut lines = part.lines().enumerate();
                while let Some((j, '#')) = lines
                    .next()
                    .and_then(|(j, l)| l.chars().nth(i).map(|c| (j, c)))
                {
                    *lock_entry = j as u32;
                }
            }

            locks.push(lock);
        } else {
            let mut key = [0; 5];

            for (i, key_entry) in key.iter_mut().enumerate() {
                let mut lines = part.lines().rev().enumerate();
                while let Some((j, '#')) = lines
                    .next()
                    .and_then(|(j, l)| l.chars().nth(i).map(|c| (j, c)))
                {
                    *key_entry = j as u32;
                }
            }

            keys.push(key);
        }
    }
    (keys, locks)
}
