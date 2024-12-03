use crate::parser2;


#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let (_, pairs) = parser2(input).unwrap();

    let output: u32 = pairs.iter().map(|v1| {
        v1.iter().map(|v2| {
            v2.0 * v2.1
        }).sum::<u32>()
    }).sum();
    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use crate::parse_do_dont;

    use super::*;

    #[test]
    fn test_process() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input).unwrap());
    }

    #[test]
    fn test_do_dont() {
        let input = "do()asdfmul(3,4)asdfdon't()asdf";
        let (_dis, res) = parse_do_dont(input).unwrap();
        assert_eq!("asdfmul(3,4)asdf", res);
    }
}

/* answer: 71668682
 */