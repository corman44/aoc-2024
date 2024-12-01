use itertools::Itertools;

use crate::ws_parser;


#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let nums: Vec<(u32,u32)> = input.lines()
        .map(|l| {
            let (_text, out) = ws_parser(l).unwrap();
            out
        })
        .collect();

    let (mut v1, mut v2) = nums.iter()
    .map(|n| {
        (n.0,n.1)
    })
    .collect::<(Vec<u32>, Vec<u32>)>();

    let sum_val:u32 = v1.iter()
        .map(|v| {
            v2.iter()
                .filter(|x| *x == v)
                .count() as u32
                * v
        })
        .sum();

    Ok(sum_val.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}