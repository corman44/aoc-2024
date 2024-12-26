use std::collections::HashSet;

use crate::parse;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let pcs = parse(input);
    // dbg!(&pcs);

    // Check each Key for each combination of values take each key
    // combination and check the values of each of those keys and check
    // if they contain the first key and the other key. This is a match of 3.
    // Then sort the 3 and add entry to a matches hashset. Then count entries.
    let mut matches: HashSet<Vec<String>> = HashSet::new();
    for (k, v) in pcs.iter() {
        for i in 0..v.len() {
            for j in 0..v.len() {
                if i == j {
                    continue;
                }
                if let Some(e1) = pcs.get(&v[i]) {
                    if let Some(e2) = pcs.get(&v[j]) {
                        if e1.contains(&k) && e1.contains(&v[j]) && e2.contains(&k) && e2.contains(&v[i]) {
                            let mut m = vec![k.clone(), v[i].clone(), v[j].clone()];
                            m.sort();
                            matches.insert(m);
                        }
                    }
                }
            }
        }
    }

    let ans = matches.iter()
        .filter(|conns| {
            let mut t_found = false;
            for m in *conns {
                if m.starts_with("t") {
                    t_found = true;
                }
            }
            t_found
        }).collect::<HashSet<_>>();

    Ok(ans.len().to_string())
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
td-yn";
        assert_eq!("7", process(input).unwrap());
    }
}