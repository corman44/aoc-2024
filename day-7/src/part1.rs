
#[tracing::instrument]
pub fn process(input: &str) -> Result<String, String> {
    let calibration_eqs = parse(&input);
    println!("{calibration_eqs:?}");

    Ok(0.to_string())
}

fn parse(input: &str) -> Vec<(i32, Vec<i32>)> {
    input
        .lines()
        .map(|lines| {
            let mut it = lines.split(":");
            (
                it.next().unwrap().parse::<i32>().expect("parse calc err }"),
                it.next()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .map(|num| {
                        num.parse::<i32>()
                            .expect(&format!("val parse wrong: {num}"))
                    })
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("3749", process(input).unwrap());
    }
}
