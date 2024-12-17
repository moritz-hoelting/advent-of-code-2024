use indoc::indoc;
use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

pub const SAMPLE: &str = indoc!(
    "
    Register A: 729
    Register B: 0
    Register C: 0

    Program: 0,1,5,4,3,0
    "
);

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Device {
    pub register_a: u64,
    pub register_b: u64,
    pub register_c: u64,

    pub instruction_ptr: usize,
    pub instructions: Vec<u8>,

    pub output: Vec<u8>,
}

impl Device {
    pub fn execute(&mut self) -> &[u8] {
        while self.clock() {}
        &self.output
    }

    fn combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => panic!("Reserved combo operand 7"),
            _ => panic!("Invalid combo operand {operand}"),
        }
    }

    fn clock(&mut self) -> bool {
        if let Some((instruction, operand)) =
            self.instructions
                .get(self.instruction_ptr)
                .and_then(|instruction| {
                    self.instructions
                        .get(self.instruction_ptr + 1)
                        .map(|operand| (*instruction, *operand))
                })
        {
            match instruction {
                0 => self.register_a >>= self.combo_operand(operand),
                1 => self.register_b ^= operand as u64,
                2 => self.register_b = self.combo_operand(operand) & 0b111,
                3 => {
                    if self.register_a != 0 {
                        self.instruction_ptr = operand as usize;
                        return true;
                    }
                }
                4 => {
                    self.register_b ^= self.register_c;
                }
                5 => {
                    self.output
                        .push((self.combo_operand(operand) & 0b111) as u8);
                }
                6 => self.register_b = self.register_a >> self.combo_operand(operand),
                7 => self.register_c = self.register_a >> self.combo_operand(operand),
                _ => panic!("Unsupported opcode: {instruction}"),
            }
            self.instruction_ptr += 2;
            true
        } else {
            false
        }
    }
}

pub fn parse_input(input: &str) -> Device {
    let (_, ((register_a, register_b, register_c), instructions)) = all_consuming(separated_pair(
        tuple((
            terminated(parse_register('A'), line_ending),
            terminated(parse_register('B'), line_ending),
            terminated(parse_register('C'), line_ending),
        )),
        line_ending,
        terminated(parse_program, opt(line_ending)),
    ))(input)
    .unwrap();

    Device {
        register_a,
        register_b,
        register_c,
        instruction_ptr: 0,
        instructions,
        output: Vec::new(),
    }
}

fn parse_register(reg: char) -> impl Fn(&str) -> IResult<&str, u64> {
    move |input| {
        preceded(
            tuple((tag("Register "), char(reg), tag(": "))),
            nom::character::complete::u64,
        )(input)
    }
}

fn parse_program(input: &str) -> IResult<&str, Vec<u8>> {
    preceded(
        tag("Program: "),
        separated_list1(char(','), nom::character::complete::u8),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(SAMPLE),
            Device {
                register_a: 729,
                register_b: 0,
                register_c: 0,
                instruction_ptr: 0,
                instructions: vec![0, 1, 5, 4, 3, 0],
                output: Vec::new(),
            }
        );
    }

    #[test]
    fn test_execute_1() {
        let mut device = Device {
            register_c: 9,
            instructions: vec![2, 6],
            ..Default::default()
        };
        device.execute();

        assert_eq!(device.register_b, 1);
    }

    #[test]
    fn test_execute_2() {
        let mut device = Device {
            register_a: 10,
            instructions: vec![5, 0, 5, 1, 5, 4],
            ..Default::default()
        };

        assert_eq!(device.execute(), &[0, 1, 2]);
    }

    #[test]
    fn test_execute_3() {
        let mut device = Device {
            register_a: 2024,
            instructions: vec![0, 1, 5, 4, 3, 0],
            ..Default::default()
        };

        assert_eq!(device.execute(), &[4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(device.register_a, 0);
    }

    #[test]
    fn test_execute_4() {
        let mut device = Device {
            register_b: 29,
            instructions: vec![1, 7],
            ..Default::default()
        };

        device.execute();

        assert_eq!(device.register_b, 26);
    }

    #[test]
    fn test_execute_5() {
        let mut device = Device {
            register_b: 2024,
            register_c: 43690,
            instructions: vec![4, 0],
            ..Default::default()
        };

        device.execute();

        assert_eq!(device.register_b, 44354);
    }
}
