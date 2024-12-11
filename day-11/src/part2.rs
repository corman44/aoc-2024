use std::collections::HashMap;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let mut cache: Vec<u64> = input.split(' ').into_iter().map(|num| num.parse::<u64>().expect("no u64 found")).collect();

    let mut stones: HashMap<u64, u64> = HashMap::new();
    for num in cache {
        stones.entry(num)
            .and_modify(|v| *v+=1)
            .or_insert(1);
    }
    // dbg!(stones.clone());

    for num in 0..75 {
        let mut new_stones: HashMap<u64, u64> = HashMap::new();
        for (num, count) in stones {
            let num_dig = num_digits(num);
            if num == 0 {
                new_stones.entry(1)
                    .and_modify(|v| *v+=count)
                    .or_insert(count);
            } else if num_dig % 2 == 0 {
                let div = 10u64.pow((num_dig/2).try_into().unwrap());
                new_stones.entry(num / div)
                    .and_modify(|v| *v+=count)
                    .or_insert(count);

                new_stones.entry(num - (num/div) * div)
                    .and_modify(|v| *v+=count)
                    .or_insert(count);
                
            } else {
                new_stones.entry(num*2024)
                    .and_modify(|v| *v+=count)
                    .or_insert(count);
            }
        }
        stones = new_stones;
    }
    Ok(stones.values().sum::<u64>().to_string())
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