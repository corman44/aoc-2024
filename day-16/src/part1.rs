use pathfinding::prelude::*;

use crate::parse;


#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let (player, end, map) = parse(input);

    let res = dijkstra(&player, |p| p.routes(&map), |p| *p == end );
    // dbg!(&res);
    Ok(res.expect("no path..").1.to_string())
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
assert_eq!("7036", process(input).unwrap());
    }
}