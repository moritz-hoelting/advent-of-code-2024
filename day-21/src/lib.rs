use std::{
    collections::{HashMap, VecDeque},
    sync::LazyLock,
};

use indoc::indoc;
use itertools::Itertools;

pub const SAMPLE: &str = indoc!(
    "
    029A
    980A
    179A
    456A
    379A
    "
);

pub const INPUT: &str = include_str!("../input.txt");

static NUM_SEQUENCES: LazyLock<HashMap<(char, char), Vec<String>>> = LazyLock::new(|| {
    let num_keypad = vec![
        vec![Some('7'), Some('8'), Some('9')],
        vec![Some('4'), Some('5'), Some('6')],
        vec![Some('1'), Some('2'), Some('3')],
        vec![None, Some('0'), Some('A')],
    ];

    compute_sequences(&num_keypad)
});

static DIR_SEQUENCES: LazyLock<HashMap<(char, char), Vec<String>>> = LazyLock::new(|| {
    let dir_keypad = vec![
        vec![None, Some('^'), Some('A')],
        vec![Some('<'), Some('v'), Some('>')],
    ];

    compute_sequences(&dir_keypad)
});

static DIR_SEQUENCES_LEN: LazyLock<HashMap<(char, char), usize>> = LazyLock::new(|| {
    DIR_SEQUENCES
        .iter()
        .map(|(&k, v)| (k, v.iter().map(|s| s.len()).min().unwrap()))
        .collect::<HashMap<_, _>>()
});

pub fn find_shortest_sequence(code: &str, depth: usize) -> usize {
    let mut cache = HashMap::new();
    solve(code, &NUM_SEQUENCES)
        .iter()
        .map(|input| compute_length(input, depth, &mut cache))
        .min()
        .unwrap()
}

fn solve(code: &str, sequences: &HashMap<(char, char), Vec<String>>) -> Vec<String> {
    let full_code = "A".to_string() + code;
    let steps = full_code.chars().tuple_windows::<(char, char)>();
    let possibilities_per_step = steps
        .map(|(a, b)| sequences.get(&(a, b)).unwrap().as_slice())
        .collect::<Vec<_>>();

    possibilities_per_step
        .iter()
        .copied()
        .multi_cartesian_product()
        .map(|v| v.into_iter().join(""))
        .collect::<Vec<_>>()
}

fn compute_length<'a>(
    code: &'a str,
    depth: usize,
    cache: &mut HashMap<(usize, &'a str), usize>,
) -> usize {
    if let Some(&res) = cache.get(&(depth, code)) {
        res
    } else {
        let seq = "A".to_string() + code;
        let steps = seq.chars().tuple_windows::<(char, char)>();
        let res = if depth == 1 {
            steps.map(|(a, b)| DIR_SEQUENCES_LEN[&(a, b)]).sum()
        } else {
            steps
                .map(|(a, b)| {
                    DIR_SEQUENCES[&(a, b)]
                        .iter()
                        .map(|subseq| compute_length(subseq, depth - 1, cache))
                        .min()
                        .unwrap()
                })
                .sum()
        };
        cache.insert((depth, code), res);
        res
    }
}

fn compute_sequences(keypad: &[Vec<Option<char>>]) -> HashMap<(char, char), Vec<String>> {
    let positions = keypad
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, &c)| c.map(|c| (c, (x as isize, y as isize))))
        })
        .collect::<HashMap<_, _>>();

    let max_x = keypad[0].len();
    let max_y = keypad.len();

    let mut sequences = HashMap::new();

    for ((&a, _), (&b, _)) in positions.iter().cartesian_product(positions.iter()) {
        if a == b {
            sequences.insert((a, b), vec!["A".to_string()]);
            continue;
        }

        let mut possibilities = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back((*positions.get(&a).unwrap(), String::new()));

        let mut optimal = usize::MAX;

        while let Some(((x, y), moves)) = queue.pop_front() {
            let mut found = false;
            for (nx, ny, nm) in [
                (x, y - 1, '^'),
                (x, y + 1, 'v'),
                (x - 1, y, '<'),
                (x + 1, y, '>'),
            ]
            .into_iter()
            {
                if ny < 0 || ny >= max_y as isize || nx < 0 || nx >= max_x as isize {
                    continue;
                }
                match keypad[ny as usize][nx as usize] {
                    Some(c) if c == b => {
                        if optimal < moves.len() + 1 {
                            found = true;
                            break;
                        }
                        optimal = moves.len() + 1;
                        let mut possibility = moves.clone();
                        possibility.push(nm);
                        possibility.push('A');
                        possibilities.push(possibility);
                    }
                    Some(_) => {
                        let mut next_moves = moves.clone();
                        next_moves.push(nm);
                        queue.push_back(((nx, ny), next_moves));
                    }
                    None => {}
                }
            }
            if found {
                break;
            }
        }
        sequences.insert((a, b), possibilities);
    }

    sequences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_shortest_sequence() {
        assert_eq!(
            find_shortest_sequence("029A", 2),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
        assert_eq!(
            find_shortest_sequence("980A", 2),
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len()
        );
        assert_eq!(
            find_shortest_sequence("179A", 2),
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
        assert_eq!(
            find_shortest_sequence("456A", 2),
            "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len()
        );
        assert_eq!(
            find_shortest_sequence("379A", 2),
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
    }
}
