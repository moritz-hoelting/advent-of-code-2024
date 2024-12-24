use std::collections::HashMap;

use itertools::Itertools as _;

use day_24::{parse_input, Operation, INPUT};

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &str) -> u64 {
    let (mut known, connections) = parse_input(input);

    let result = connections
        .keys()
        .filter(|k| k.starts_with('z'))
        .sorted()
        .map(|wire| {
            if calc_result(wire, &mut known, &connections) {
                '1'
            } else {
                '0'
            }
        })
        .rev()
        .collect::<String>();

    u64::from_str_radix(&result, 2).unwrap()
}

fn calc_result<'a>(
    wire: &'a str,
    known: &mut HashMap<&'a str, bool>,
    connections: &HashMap<&'a str, (Operation, &'a str, &'a str)>,
) -> bool {
    if let Some(&result) = known.get(wire) {
        return result;
    }

    let &(op, a, b) = connections.get(wire).unwrap();

    let a = calc_result(a, known, connections);
    let b = calc_result(b, known, connections);

    let res = match op {
        Operation::And => a & b,
        Operation::Or => a | b,
        Operation::Xor => a ^ b,
    };

    known.insert(wire, res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = indoc::indoc!(
        "
        x00: 1
        x01: 1
        x02: 1
        y00: 0
        y01: 1
        y02: 0
    
        x00 AND y00 -> z00
        x01 XOR y01 -> z01
        x02 OR y02 -> z02
        "
    );

    const SAMPLE_2: &str = indoc::indoc!(
        "
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1
    
        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
        "
    );

    #[test]
    fn test_part1_1() {
        assert_eq!(part_1(SAMPLE_1), 4);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part_1(SAMPLE_2), 2024);
    }
}
