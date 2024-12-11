
#[tracing::instrument]
pub fn process(
    _input: &str,
) -> Result<String, String> {
    todo!(" - part1");
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