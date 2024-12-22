pub const INPUT: &str = include_str!("../input.txt");

pub fn generate_secret_numbers(start: u64) -> impl Iterator<Item = u64> + Clone {
    const PRUNE: u64 = 2_u64.pow(24) - 1;
    std::iter::successors(Some(start), |&n| {
        let a = n << 6;
        let n = (a ^ n) & PRUNE;
        let b = n >> 5;
        let n = (b ^ n) & PRUNE;
        let c = n << 11;
        Some((c ^ n) & PRUNE)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secret_numbers() {
        assert_eq!(
            generate_secret_numbers(123)
                .skip(1)
                .take(10)
                .collect::<Vec<_>>(),
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254,
            ]
        );
    }
}
