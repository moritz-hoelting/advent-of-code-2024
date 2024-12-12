use indoc::indoc;

pub const SAMPLE_1: &str = indoc!(
    "
    AAAA
    BBCD
    BBCC
    EEEC
    "
);

pub const SAMPLE_2: &str = indoc!(
    "
    OOOOO
    OXOXO
    OOOOO
    OXOXO
    OOOOO
    "
);

pub const SAMPLE_3: &str = indoc!(
    "
    RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE
    "
);

pub const INPUT: &str = include_str!("../input.txt");
