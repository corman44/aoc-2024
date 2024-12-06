use std::collections::{HashMap, HashSet};

#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let data = simple_parse(&input);
    let mut direction = Direction::North;
    let path: HashSet<(usize, usize)> = HashSet::new();

    // get starting location
    let start_v: Vec<(&usize, &usize)> = data.iter()
        .filter(|(_, t)| *t == &Tile::Guard)
        .map(|((x,y), _)| (x,y))
        .collect();
    let guard_pos = start_v.first().expect("no Guard Position found");
    println!("Guard Start Pos: {guard_pos:?}");

    let mut cont = true;
    // while cont {

    // }

    Ok(0.to_string())
}

#[derive(Debug, Default, PartialEq, Eq)]
enum Tile {
    #[default]
    Empty,
    Guard,
    Object,
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn simple_parse(input: &str) -> HashMap<(usize, usize), Tile> {
    let mut x = 0;
    let mut y = 0;
    // let mut result: HashMap<(usize, usize), Tile> = HashMap::new();
    input.lines()
        .enumerate()
        .flat_map(|(y,line)| {
            line.chars()
                .enumerate()
                .map(move |(x,c)| {
                    if c == '.' {
                        ((x,y), Tile::Empty)
                    } else if c == '^' {
                        ((x,y), Tile::Guard)
                    } else {
                        ((x,y), Tile::Object)
                    }
                })
        })
        .collect::<HashMap<(usize, usize), Tile>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let data = simple_parse(&input);
        // println!("{data:?}");
        assert_eq!("41", process(input).unwrap());
    }
}