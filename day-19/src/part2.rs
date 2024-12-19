use std::{collections::HashMap, sync::RwLock};

use day_19::INPUT;

fn main() {
    println!("Part 2: {}", part_2(INPUT));
}

fn part_2(input: &str) -> u64 {
    let (towels, designs) = input.split_once("\n\n").unwrap();

    let towels = towels.split(", ").collect::<Vec<_>>();

    let cache = RwLock::new(HashMap::new());

    designs
        .lines()
        .map(|design| possibilities(design, &towels, &cache))
        .sum()
}

fn possibilities<'a>(
    design: &'a str,
    towels: &[&str],
    cache: &RwLock<HashMap<&'a str, u64>>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(ans) = cache.read().unwrap().get(design) {
        return *ans;
    }
    let ans = towels
        .iter()
        .map(|towel| {
            if !design.starts_with(towel) {
                0
            } else {
                possibilities(&design[towel.len()..], towels, cache)
            }
        })
        .sum();
    cache.write().unwrap().insert(design, ans);
    ans
}

#[cfg(test)]
mod tests {
    use day_19::SAMPLE;

    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE), 16);
    }
}
