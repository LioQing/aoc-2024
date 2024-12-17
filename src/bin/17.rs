use core::panic;

use aoc_2024::input;
use glam::*;
use itertools::{FoldWhile, Itertools};
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{digit1, multispace1},
    combinator::recognize,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<i64> for OpCode {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid opcode: {}", value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Operand {
    lit: i64,
    combo: ComboOperand,
}

impl From<i64> for Operand {
    fn from(value: i64) -> Self {
        Self {
            lit: value,
            combo: value.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ComboOperand {
    Lit(i64),
    Reg(usize),
}

impl From<i64> for ComboOperand {
    fn from(value: i64) -> Self {
        match value {
            0..=3 => Self::Lit(value),
            register @ 4..=6 => Self::Reg(register as usize - 4),
            _ => panic!("Invalid operand: {}", value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Computer {
    pub registers: [i64; 3],
    pub program: Vec<(OpCode, Operand)>,
}

impl Computer {
    pub fn compute(&self) -> Vec<i64> {
        std::iter::repeat(())
            .fold_while(
                (0, self.registers, Vec::new()),
                |(mut ptr, mut registers, mut out), _| {
                    if ptr >= self.program.len() {
                        return FoldWhile::Done((ptr, registers, out));
                    }

                    let (opcode, operand) = self.program.get(ptr).unwrap();
                    ptr += 1;

                    let lit_val = operand.lit;
                    let combo_val = match operand.combo {
                        ComboOperand::Lit(value) => value,
                        ComboOperand::Reg(register) => registers[register],
                    };

                    match opcode {
                        OpCode::Adv => registers[0] >>= combo_val,
                        OpCode::Bxl => registers[1] ^= lit_val,
                        OpCode::Bst => registers[1] = combo_val & 0b111,
                        OpCode::Jnz => match registers[0] {
                            0 => {}
                            _ => ptr = lit_val as usize / 2,
                        },
                        OpCode::Bxc => registers[1] ^= registers[2],
                        OpCode::Out => out.push(combo_val & 0b111),
                        OpCode::Bdv => registers[1] = registers[0] >> combo_val,
                        OpCode::Cdv => registers[2] = registers[0] >> combo_val,
                    }

                    FoldWhile::Continue((ptr, registers, out))
                },
            )
            .into_inner()
            .2
    }
}

pub fn parse_input(input: &str) -> (Computer, Vec<i32>) {
    fn i32(input: &str) -> IResult<&str, i32> {
        let (input, n) = recognize(digit1)(input)?;

        Ok((input, n.parse().unwrap()))
    }

    fn i64(input: &str) -> IResult<&str, i64> {
        let (input, n) = recognize(digit1)(input)?;

        Ok((input, n.parse().unwrap()))
    }

    fn register(input: &str) -> IResult<&str, i64> {
        let (input, _) = tag("Register ")(input)?;
        let (input, _) = take_while_m_n(1, 1, |c: char| c.is_alphabetic())(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, value) = i64(input)?;

        Ok((input, value))
    }

    fn stmt(input: &str) -> IResult<&str, (OpCode, Operand)> {
        let (input, (opcode, operand)) = separated_pair(i64, tag(","), i64)(input)?;

        Ok((input, (opcode.into(), operand.into())))
    }

    fn program(input: &str) -> IResult<&str, Vec<(OpCode, Operand)>> {
        let (input, _) = tag("Program: ")(input)?;
        let (input, program) = separated_list1(tag(","), stmt)(input)?;

        Ok((input, program))
    }

    fn computer(input: &str) -> IResult<&str, Computer> {
        let (input, registers) = separated_list1(multispace1, register)(input)?;
        let (input, _) = multispace1(input)?;
        let (input, program) = program(input)?;

        Ok((
            input,
            Computer {
                registers: registers.as_slice().try_into().unwrap(),
                program,
            },
        ))
    }

    fn src_code(input: &str) -> IResult<&str, Vec<i32>> {
        let (input, _) = separated_list1(multispace1, register)(input)?;
        let (input, _) = multispace1(input)?;
        let (input, _) = tag("Program: ")(input)?;
        let (input, src_code) = separated_list1(tag(","), i32)(input)?;

        Ok((input, src_code))
    }

    let (_, computer) = computer(input).unwrap();
    let (_, src_code) = src_code(input).unwrap();

    (computer, src_code)
}

pub fn solution_part_1(computer: &Computer) -> String {
    computer
        .compute()
        .into_iter()
        .map(|x| x.to_string())
        .join(",")
}

pub fn solution_part_2(computer: &Computer, src_code: &[i32]) -> i64 {
    src_code
        .iter()
        .map(|&code| code as i64)
        .rev()
        .fold(vec![0], |inits: Vec<i64>, code| {
            inits
                .into_iter()
                .flat_map(|init| {
                    (0..8).map(move |x| (init << 3) | x).filter(|&x| {
                        let out = Computer {
                            registers: [x, 0, 0],
                            program: computer.program.clone(),
                        }
                        .compute();

                        out.first() == Some(&code)
                    })
                })
                .collect_vec()
        })
        .into_iter()
        .min()
        .unwrap()
}

fn main() {
    let input = input(17);

    let (computer, src_code) = parse_input(&input);

    println!("Part 1 solution: {}", solution_part_1(&computer));
    println!("Part 2 solution: {}", solution_part_2(&computer, &src_code));
    println!(
        "Verify part 2: {}",
        solution_part_1(&Computer {
            registers: [solution_part_2(&computer, &src_code), 0, 0],
            program: computer.program,
        })
    );
}
