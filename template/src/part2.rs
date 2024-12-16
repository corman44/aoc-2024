
#[tracing::instrument]
pub fn process(
    _input: &str,
) -> Result<String, String> {
    todo!(" - part2");
    Ok(0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input).unwrap());
    }
}