use std::collections::{HashMap, HashSet};

use indoc::indoc;
use itertools::Itertools as _;

pub const SAMPLE: &str = indoc!(
    "
    kh-tc
    qp-kh
    de-cg
    ka-co
    yn-aq
    qp-ub
    cg-tb
    vc-aq
    tb-ka
    wh-tc
    yn-cg
    kh-ub
    ta-co
    de-co
    tc-td
    tb-wq
    wh-td
    ta-ka
    td-qp
    aq-cg
    wq-ub
    ub-vc
    de-ta
    wq-aq
    wq-vc
    wh-yn
    ka-de
    kh-ta
    co-tc
    wh-qp
    tb-vc
    td-yn
    "
);

pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> HashSet<(&str, &str)> {
    input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect()
}

pub fn connections<'a>(
    pairs: impl Iterator<Item = (&'a str, &'a str)>,
) -> HashMap<&'a str, HashSet<&'a str>> {
    pairs
        .flat_map(|(a, b)| [(a, b), (b, a)])
        .sorted_by_key(|&(a, _)| a)
        .chunk_by(|&(a, _)| a)
        .into_iter()
        .map(|(v, neighbours)| (v, neighbours.map(|(_, b)| b).collect::<HashSet<_>>()))
        .collect()
}
