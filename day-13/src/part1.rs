

#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let data = parse(input);
    Ok(0.to_string())
}

pub struct Buttons {
    pub a: IVec2,
    pub b: IVec2,
}

fn parse(input: &str) -> Vec<IVec2, Buttons> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input).unwrap());
    }
}