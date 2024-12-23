use crate::parse;


#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let (available, requested) = parse(input).expect("unable to parse.");

    

    Ok(0.to_string())
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
bbrgwb
";
        assert_eq!("16", process(input).unwrap());
    }
}