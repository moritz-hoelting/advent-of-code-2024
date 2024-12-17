use day_17::{parse_input, INPUT};

fn main() {
    println!("Part 2: {}", part_2(INPUT));
}

// fn part_2(input: &str) -> u64 {
//     let device = parse_input(input);

//     (0..u64::MAX)
//         .into_par_iter()
//         .progress_count(u64::MAX)
//         .find_first(|i| {
//             let mut copy = device.clone();
//             copy.set_reg_a(*i);
//             copy.execute() == device.instructions()
//         })
//         .unwrap()
// }

fn part_2(input: &str) -> u64 {
    let device = parse_input(input);
    let program = device.instructions.as_slice();

    assert_eq!(
        &program[program.len() - 2..],
        &[3, 0],
        "Program does not end with JNZ 0"
    );

    find(program, 0, program).unwrap()
}

fn find(target: &[u8], ans: u64, program: &[u8]) -> Option<u64> {
    if target.is_empty() {
        return Some(ans);
    }

    for t in 0..8 {
        let mut a = (ans << 3) | t;
        let mut b = 0;
        let mut c = 0;
        let mut output = None;
        let mut adv3 = false;

        for pointer in 0..((program.len() - 2) / 2) {
            let instruction = program[pointer * 2];
            let operand = program[pointer * 2 + 1];
            match instruction {
                0 => {
                    assert!(!adv3, "multiple ADVs");
                    assert_eq!(operand, 3, "ADV with non-3 operand");
                    adv3 = true;
                    if target.len() == 1 && program.first() == Some(&0) {
                        a = a << 3 | t;
                    }
                }
                1 => b ^= operand as u64,
                2 => b = combo_operand(operand, a, b, c) & 0b111,
                3 => panic!("JNZ in inner loop: {}", pointer * 2),
                4 => b ^= c,
                5 => {
                    assert_eq!(output, None, "multiple outs");
                    output = Some(combo_operand(operand, a, b, c) as u8 & 0b111);
                }
                6 => b = a >> combo_operand(operand, a, b, c),
                7 => c = a >> combo_operand(operand, a, b, c),
                _ => panic!("Unsupported opcode: {instruction}"),
            }
            if output == target.last().copied() {
                let sub = find(&target[0..(target.len() - 1)], a, program);
                if sub.is_none() {
                    continue;
                }
                return sub;
            }
        }
    }

    None
}

fn combo_operand(operand: u8, a: u64, b: u64, c: u64) -> u64 {
    match operand {
        0..=3 => operand as u64,
        4 => a,
        5 => b,
        6 => c,
        7 => panic!("Reserved combo operand 7"),
        _ => panic!("Invalid combo operand {operand}"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE: &str = indoc::indoc!(
        "
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
        "
    );

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE), 117440);
    }
}
