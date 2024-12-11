use std::collections::BTreeMap;

pub const SAMPLE: &str = "125 17";

pub const INPUT: &str = include_str!("../input.txt");

pub fn blink_n_times(input: &str, n: u32) -> BTreeMap<u64, u64> {
    let start = parse_input(input);

    (0..n).fold(start, |acc, _| blink(acc))
}

pub fn parse_input(input: &str) -> BTreeMap<u64, u64> {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().expect("invalid input"))
        .fold(BTreeMap::new(), |mut acc, cur| {
            acc.entry(cur).and_modify(|x| *x += 1).or_insert(1);
            acc
        })
}

pub fn blink(nums: BTreeMap<u64, u64>) -> BTreeMap<u64, u64> {
    nums.into_iter()
        .flat_map(|(num, amount)| {
            let len = ((num as f64).log10()).floor() as u32 + 1;
            match num {
                0 => vec![(1, amount)],
                _ if len % 2 == 0 => {
                    let div = 10_u64.pow(len / 2);
                    let first = num / div;
                    let last = num % div;
                    vec![(first, amount), (last, amount)]
                }
                _ => vec![(num * 2024, amount)],
            }
        })
        .fold(BTreeMap::new(), |mut acc, (num, amount)| {
            acc.entry(num)
                .and_modify(|x| *x += amount)
                .or_insert(amount);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink() {
        test_blink_helper("0 1 10 99 999", "1 2024 1 0 9 9 2021976");

        test_blink_helper("125 17", "253000 1 7");
        test_blink_helper("253000 1 7", "253 0 2024 14168");
        test_blink_helper("253 0 2024 14168", "512072 1 20 24 28676032");
        test_blink_helper("512072 1 20 24 28676032", "512 72 2024 2 0 2 4 2867 6032");
        test_blink_helper(
            "512 72 2024 2 0 2 4 2867 6032",
            "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32",
        );
        test_blink_helper(
            "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32",
            "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2",
        );
    }

    fn test_blink_helper(input: &str, expected: &str) {
        let input = parse_input(input);
        let expected = parse_input(expected);

        assert_eq!(blink(input), expected)
    }
}
