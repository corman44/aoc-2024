use crate::{parse, validate_design};



#[tracing::instrument]
pub fn process(input: &str) -> Result<String, String> {
    let (mut available, requested) = parse(input).expect("parsing error:");

    available.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    let count = requested.iter()
        .filter(|design| validate_design(design, &available))
        .count();

    Ok(count.to_string())
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
