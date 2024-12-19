use indoc::indoc;

pub const SAMPLE: &str = indoc!(
    "
    r, wr, b, g, bwu, rb, gb, br

    brwrr
    bggr
    gbbr
    rrbgbr
    ubwu
    bwurrg
    brgr
    bbrgwb
    "
);

pub const INPUT: &str = include_str!("../input.txt");
