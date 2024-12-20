use crate::{operand_to_value, parse, Instruction, Registers};



#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    dbg!("Running Test...");
    let (registers, programs) = parse(input);
    let mut matched = false;
    let mut counter = registers.a + 1;

    let mut programs_vec = vec![];
    for (inst, prog) in &programs {
        programs_vec.push(*inst as u8);
        programs_vec.push(*prog);
    }
    dbg!(&programs_vec);

    let mut test_reg = Registers { 
        a: 0,
        b: 0,
        c: 0,
    };

    while !matched {
        let mut test_answer = vec![];
        test_reg.a = counter;
        // println!("{}",counter);
        test_answer = run_program(&mut test_reg, &programs);
        if counter == 117440 {
            dbg!(&test_answer);
            dbg!(&test_reg);
        }
        if test_answer == programs_vec {
            matched = true;
            dbg!(test_answer);
        } else {
            counter += 1;
        }
    }

    dbg!(&programs_vec);
    Ok(counter.to_string())
}

pub fn run_program(registers: &mut Registers, programs: &Vec<(Instruction, u8)>) -> Vec<u8> {
    let mut ip = 0;
    let mut answer:Vec<u8> = vec![];
    
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
            Instruction::out => answer.push((operand_to_value(&registers, *operand) % 8).try_into().unwrap()),
            Instruction::bdv => registers.b = registers.a / 2i32.pow(operand_to_value(&registers, *operand).try_into().unwrap()),
            Instruction::cdv => registers.c = registers.a / 2i32.pow(operand_to_value(&registers, *operand).try_into().unwrap()),
        }
        if !skip_ip_increase { ip += 1;}
    }

    return answer;
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
        assert_eq!("117440", process(input).unwrap());
        // Original: 4,6,3,5,6,3,5,2,1,0
    }
}