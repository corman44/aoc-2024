use std::collections::HashMap;
use itertools::Itertools;

use crate::parse;


#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let connections = parse(&input);

    let answer = connections.iter()
        .map(|(k, v)| {
            // check if all values are contained within each other key
        });

    Ok(0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";
        assert_eq!("co,de,ka,ta", process(input).unwrap());
    }
}