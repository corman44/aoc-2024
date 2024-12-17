use nom::{bytes::complete::tag, character::complete, combinator::value, multi::many0, sequence::{preceded, separated_pair, tuple}, IResult, Parser};
use num_enum::TryFromPrimitive;


#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let (mut registers, programs) = parse(input);
    let mut ip = 0;
    let mut out_buf = String::new();

    dbg!(&registers);
    dbg!(&programs);

    while ip < programs.len() {
        let mut skip_ip_increase = false;
        let (instr, operand) = &programs[ip];

        match instr {
            Instruction::Adv => registers.a = registers.a / operand_to_value(&registers, *operand)^2,
            Instruction::Bxl => registers.b = registers.b | *operand as u32,
            Instruction::Bst => registers.b = operand_to_value(&registers, *operand) & 7,
            Instruction::Jnz => {
                if registers.a != 0 {
                    skip_ip_increase = true;
                    ip = (*operand / 2) as usize;
                }
            },
            Instruction::bxc => registers.b |= registers.c,
            Instruction::out => out_buf.push_str(&format!("{},",operand_to_value(&registers, *operand))),
            Instruction::bdv => registers.b = registers.a / operand_to_value(&registers, *operand)^2,
            Instruction::cdv => registers.c = registers.a / operand_to_value(&registers, *operand)^2,
        }
        if !skip_ip_increase { ip += 1;}
    }
    dbg!(out_buf);
    
    Ok(0.to_string())
}

pub fn operand_to_value(regs: &Registers, op: u8) -> u32 {
    match op {
        x if x <= 3 => x as u32,
        4 => regs.a,
        5 => regs.b,
        6 => regs.c,
        _ => panic!("not recognized operand..")
    }
}

fn parse(input: &str) -> (Registers, Vec<(Instruction,u8)>) {
    let (left, a) = parse_a(input).expect("unable to parse_a");
    let (left, b) = parse_b(left.trim()).expect("unable to parse_b");
    let (left, c) = parse_c(left.trim()).expect("unable to parse_c");
    let reg = Registers {
        a: a,
        b: b,
        c: c,
    };

    let (left, program) = parse_program(left.trim()).expect("unable to parse program");
    (reg, program)
}

fn parse_a(input: &str) -> IResult<&str, u32> {
    preceded(
        tag("Register A: "),
        complete::u32,
    )(input)
}

fn parse_b(input: &str) -> IResult<&str, u32> {
    preceded(
        tag("Register B: "),
        complete::u32,
    )(input)
}

fn parse_c(input: &str) -> IResult<&str, u32> {
    preceded(
        tag("Register C: "),
        complete::u32,
    )(input)
}

// FIXME: get comma separated instructions properly...
fn parse_program(input: &str) -> IResult<&str, Vec<(Instruction,u8)>> {
    preceded(
        tag("Program: "),
        many0(
            separated_pair(
                complete::u8,
                tag(","),
                complete::u8
            ).map(|(instr, op)| {
                (
                    Instruction::try_from(instr).expect("Instruction conversion failed.."),
                    op
                )
            })
        )
    )(input)
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Registers {
    pub a: u32,
    pub b: u32,
    pub c: u32
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Instruction {
    /// division A / (operand)^2 "round down" and write to A
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", process(input).unwrap());
    }
}