
#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let mut sum = 0;

    for line in input.lines() {
        let mut ascending: Option<bool> = None;
        let mut mistakes = 0;
        let nums = parse_line(line);
        let mut _skip_next = false;
        println!("nums: {nums:?}");

        for idx in 0..nums.len()-1 {
            let diff = nums[idx+1] - nums[idx];

            // Check rules
            if diff.abs() > 3 || diff == 0 {
                if ascending.is_none() { // check 1st 3rd and 2nd 3rd
                    let diff2 = nums[idx+2] - nums[idx];
                    if diff2.abs() > 3 || diff2 == 0 {
                        let diff3 = nums[idx+2] - nums[idx+1];
                        if diff3.abs() > 3 || diff3 == 0 { break; }
                        ascending = Some(diff3.is_positive());
                        mistakes += 1;
                    } else {
                        ascending = Some(diff2.is_positive());
                        mistakes += 1;
                    }
                } else if ascending.unwrap() == diff.is_positive() { // Check idx - 1 and idx + 1
                    if mistakes > 0 { break; }
                    let diff2 = nums[idx+1] - nums[idx-1];
                    if (diff2.abs() > 3 || diff2 == 0 || ascending.unwrap() != diff2.is_positive()) && idx+2 < nums.len() {
                        let diff3 = nums[idx+2] - nums[idx];
                        if diff3.abs() > 3 || diff3 == 0 || ascending.unwrap() != diff3.is_positive() {
                            // println!("Broke!");
                            break;
                        }
                    }
                }
            } else { // need to check if ascending matches
                if ascending.is_none() {
                    ascending = Some(diff.is_positive());
                } 
            }

            println!("Ascending: {ascending:?}");
            if mistakes > 1 {
                println!("Broke!");
                break;
            }

            if idx+2 == nums.len() {
                sum += 1;
                println!("Passed!");
            }
        }
    }

    Ok(sum.to_string())
}

fn parse_line(input: &str) -> Vec<i32> {
    input.split(" ").into_iter().map(|x| x.parse::<i32>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
7 7 6 5 4
7 7 6 5 1 4
7 9 8 5 4 3
70 66 64 62 60 63 63";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}

/* Tries:
- 822
- 823
- 711
 */