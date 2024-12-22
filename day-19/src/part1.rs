use nom::{
    bytes::complete::tag, character::complete::alpha1, multi::separated_list0, IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, String> {
    let (mut available, requested) = parse(input).expect("parsing error:");

    // TODO sort available towels by longest first
    available.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    let count = requested.iter()
        .filter(|design| validate_design(design, &available))
        .count();

    Ok(count.to_string())
}

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
        .any(|v|v)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!("6", process(input).unwrap());
    }
}
