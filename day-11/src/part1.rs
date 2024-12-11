
#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let mut stones: Vec<u64> = input.split(' ').into_iter().map(|num| num.parse::<u64>().expect("no u64 found")).collect();

    for _num in 0..25 {
        blink(&mut stones);
    }
    Ok(stones.len().to_string())
}

fn blink(mut stones: &mut Vec<u64>) {
    for i in 0..stones.len() {
        let num_dig: u64 = num_digits(stones[i]);
        if stones[i] == 0 {
            stones[i] = 1;
        } else if (num_dig % 2) == 1 { // odd idigits
            stones[i] = stones[i] * 2024;
        } else { // even digits
            let prev_stone_val = stones[i];
            stones[i] = stones[i] / 10u64.pow((num_dig/2).try_into().unwrap());
            // need to mask out the top half of digits
            stones.push(prev_stone_val - (stones[i] * 10u64.pow((num_dig/2).try_into().unwrap())));
        }
    }
}

pub fn num_digits(num: u64) -> u64 {
    let mut digits: u64 = 1;
    while num / (10u64.pow(digits.try_into().unwrap())) > 0 {
        digits += 1;
    }
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "125 17";
        assert_eq!("55312", process(input).unwrap());
    }

    #[test]
    fn test_num_digits() {
        assert_eq!(1, num_digits(9));
        assert_eq!(2, num_digits(99));
        assert_eq!(3, num_digits(999));
        assert_eq!(4, num_digits(9999));
    }
}