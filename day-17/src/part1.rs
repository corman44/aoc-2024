use crate::{operand_to_value, parse, Instruction};

#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    // dbg!("Running Test...");
    let (mut registers, programs) = parse(input);
    let mut ip = 0;
    let mut out_buf = String::new();
    
    while ip < programs.len() {
        let mut skip_ip_increase = false;
        let (instr, operand) = &programs[ip];

        match instr {
            Instruction::Adv => {
                let div =  2i32.pow(operand_to_value(&registers, *operand).try_into().unwrap());
                registers.a = registers.a / div;
            },
            Instruction::Bxl => registers.b = registers.b ^ *operand as i32,
            Instruction::Bst => registers.b = operand_to_value(&registers, *operand) & 7,
            Instruction::Jnz => {
                if registers.a != 0 {
                    skip_ip_increase = true;
                    ip = (*operand / 2) as usize;
                }
            },
            Instruction::bxc => registers.b ^= registers.c,
            Instruction::out => out_buf.push_str(&format!("{},",operand_to_value(&registers, *operand) % 8)),
            Instruction::bdv => registers.b = registers.a / 2i32.pow(operand_to_value(&registers, *operand).try_into().unwrap()),
            Instruction::cdv => registers.c = registers.a / 2i32.pow(operand_to_value(&registers, *operand).try_into().unwrap()),
        }
        if !skip_ip_increase { ip += 1;}
    }
    
    Ok(out_buf.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process() {
        let input = 
"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0,";
        assert_eq!("4,6,3,5,6,3,5,2,1,0,", process(input).unwrap());
    }
}