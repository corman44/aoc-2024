use std::collections::HashMap;

pub mod part1;
pub mod part2;

pub fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let cons = line.split('-').map(|s| s.to_string()).collect::<Vec<String>>();
        connections.entry(cons[0].clone())
        .and_modify(|v| v.push(cons[1].clone()))
        .or_insert(vec![cons[1].clone()]);
        
        connections.entry(cons[1].clone())
        .and_modify(|v| v.push(cons[0].clone()))
        .or_insert(vec![cons[0].clone()]);
    }
    connections
}