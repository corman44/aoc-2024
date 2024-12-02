use tracing::info;



#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let mut sum = 0;

    for line in input.lines() {
        let mut last_up: Option<bool> = None;
        let mut up: bool = false;
        let nums = parse_line(line);

        for idx in 0..nums.len()-1 {
            // println!("idx: {idx}\nsum: {sum}");
            let diff = nums[idx+1] - nums[idx];
            if diff == 0 || diff.abs() > 3 {
                break;
            }
            let up = diff.is_positive();
            if last_up.is_none() {
                last_up = Some(up);
            } else if last_up.unwrap() != up {
                break;
            }

            // breaker
            if idx == nums.len()-2 {
                sum += 1;
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
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}

/* Answer: 680
 */