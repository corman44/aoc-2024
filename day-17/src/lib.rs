pub mod part1;
pub mod part2;

use num_enum::TryFromPrimitive;
use nom::{bytes::complete::tag, character::complete, multi::many0, sequence::{preceded, separated_pair, terminated}, IResult, Parser};


pub fn operand_to_value(regs: &Registers, op: u8) -> i32 {
    match op {
        x if x <= 3 => x as i32,
        4 => regs.a,
        5 => regs.b,
        6 => regs.c,
        _ => panic!("not recognized operand..")
    }
}

pub fn parse(input: &str) -> (Registers, Vec<(Instruction,u8)>) {
    let (left, a) = parse_a(input).expect("unable to parse_a");
    let (left, b) = parse_b(left.trim()).expect("unable to parse_b");
    let (left, c) = parse_c(left.trim()).expect("unable to parse_c");
    let reg = Registers {
        a,
        b,
        c,
    };

    let (_left, program) = parse_program(left.trim()).expect("unable to parse program");
    (reg, program)
}

fn parse_a(input: &str) -> IResult<&str, i32> {
    preceded(
        tag("Register A: "),
        complete::i32,
    )(input)
}

fn parse_b(input: &str) -> IResult<&str, i32> {
    preceded(
        tag("Register B: "),
        complete::i32,
    )(input)
}

fn parse_c(input: &str) -> IResult<&str, i32> {
    preceded(
        tag("Register C: "),
        complete::i32,
    )(input)
}

fn parse_program(input: &str) -> IResult<&str, Vec<(Instruction,u8)>> {
    preceded(
        tag("Program: "),
        many0(
            terminated(
                separated_pair(
                    complete::u8,
                    tag(","),
                    complete::u8
                ).map(|(instr, op)| {
                    (
                        Instruction::try_from(instr).expect("Instruction conversion failed.."),
                        op
                    )
                }),
                tag(",")
            )
        )
    )(input)
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Registers {
    pub a: i32,
    pub b: i32,
    pub c: i32
}

#[derive(Debug, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum Instruction {
    /// division A / 2^(operand) "round down" and write to A
    Adv,
    /// B XOR operand literal
    Bxl, 
    /// writes Combo % 8 to B
    Bst, 
    /// nothing if A == 0, set IP to literal operand IP not increased by 2 if jump
    Jnz, 
    /// writes B XOR C to B
    bxc, 
    /// Combo % 8 and outputs (outputs are comma seperated)
    out, 
    /// same as Adv but write to B
    bdv, 
    /// same as Adv but write to C
    cdv, 
}