pub mod part1;
pub mod part2;

use nom::{
    bytes::complete::tag, character::complete::alpha1, multi::separated_list0, IResult, Parser,
};

pub fn validate_design(design: &Towels, towels: &Vec<Towels>) -> bool {
    towels.iter()
        .map(|t| {
            if design.0.starts_with(&t.0) {
                let new_design = Towels(design.0[t.0.len()..].to_vec());
                if new_design.0.is_empty() {
                    return true;
                } else {
                    return validate_design(&new_design, towels);
                }
            } else {
                return false;
            }
        })
        .any(|v| v)
}

pub fn validate_design2(design: &Towels, towels: &Vec<Towels>) -> Option<usize> {
    towels.iter()
        .flat_map(|t| {
            if design.0.starts_with(&t.0) {
                let new_design = Towels(design.0[t.0.len()..].to_vec());
                if new_design.0.is_empty() {
                    return Some(1);
                } else {
                    return validate_design2(&new_design, towels);
                }
            } else {
                return None;
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct Towels(pub Vec<char>);

/// Gets available towels and requested towels
pub fn parse(input: &str) -> Result<(Vec<Towels>, Vec<Towels>), ()> {
    // comma separated Vec of available towels
    let (spare, towels) = parse_available(input).expect("unable to parse available");
    let (_, requests) = parse_requests(spare.trim()).expect("unable to parse requests");
    Ok((towels, requests))
}

fn parse_requests(input: &str) -> IResult<&str, Vec<Towels>> {
    separated_list0(tag("\n"), alpha1.map(|t: &str| Towels(t.chars().collect())))(input)
}

fn parse_available(input: &str) -> IResult<&str, Vec<Towels>> {
    separated_list0(tag(", "), alpha1.map(|t: &str| Towels(t.chars().collect())))(input)
}