use std::collections::HashSet;

use glam::IVec2;
use pathfinding::prelude::*;

use crate::parse;


#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let (player, end, map) = parse(input);

    let res = astar_bag(&player, |p| p.routes(&map),|_| 0,  |p| *p == end );
    let mut set: HashSet<IVec2> = HashSet::new();
    for path in res.expect("No path found").0.into_iter() {
        for p in path.iter() {
            set.insert(p.loc);
        }
    }
    Ok(set.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
assert_eq!("45", process(input).unwrap());
    }
}