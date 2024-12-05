use std::collections::HashMap;

use nom::{branch::alt, bytes::complete::tag, character::complete, multi::many0, sequence::{separated_pair, terminated}, IResult};

#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    Ok("".to_string())
}

fn parser(input: &str) {
    let mut splitted = input.split("\n\n").into_iter();
    let rules_raw = splitted.next().unwrap();
    rules_raw.to_string().push('\n');
    let produce_raw = splitted.next().unwrap();
    println!("rules: {rules_raw}");
    // println!("produce: {produce_raw}");
    
    let x = parse_rules(rules_raw);
    println!("x: {x:?}");
}

fn parse_rules(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many0(
        terminated(
            separated_pair(
                complete::i32,
                tag("|"),
                complete::i32
            ),
            alt((
                tag("\n"),
                tag("\n\n")
            ))
        )
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        parser(input);
        assert_eq!("143", process(input).unwrap());
    }
}