use crate::parser;


#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let (_dis, pairs) = parser(input).unwrap();
    let total: u32 = pairs.iter().map(|(v1,v2)| {
        v1*v2
    }).sum();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{parse_pair, parser};

    use super::*;

    #[test]
    fn test_process() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input).unwrap());
    }

    #[test]
    fn test_parser_multi() {
        let input = "mul(2,3)asdfmul(3,3)adf";
        assert_eq!(("adf", vec![(2,3),(3,3)]), parser(input).unwrap());
    }

    #[test]
    fn test_parser_pair() {
        let input = "mul(2,3)asdf";
        assert_eq!(("asdf", (2,3)), parse_pair(input).unwrap());
    }


}