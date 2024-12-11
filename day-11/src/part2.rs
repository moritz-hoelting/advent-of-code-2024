
use day_11::{blink_n_times, INPUT};

fn main() {
    println!("Part 2: {}", part_2(INPUT));
}

fn part_2(input: &str) -> u64 {
    blink_n_times(input, 75).into_values().sum()
}
