use std::collections::{HashMap, HashSet};

use crate::{simple_parse, Direction, Tile};

#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let data = simple_parse(&input);
    let mut direction = Direction::North;
    let mut path: HashSet<(i32, i32)> = HashSet::new();

    // get starting location
    let start_v: Vec<(i32, i32)> = data.iter()
        .filter(|(_, t)| *t == &Tile::Guard)
        .map(|((x,y), _)| (*x,*y))
        .collect();    
    let mut guard_pos = *start_v.first().expect("no Guard Position found");
    path.insert(guard_pos);

    let max_x: i32 = (input.lines().last().unwrap().chars().count() - 1).try_into().unwrap();
    let max_y: i32 = (input.lines().count() - 1).try_into().unwrap();
    // println!("Max x: {max_x}\nMax y: {max_y}");

    let mut keep_walking = true;
    while keep_walking {
        // move in direction, check
        match direction {
            Direction::North => {
                // check out of bounds or turning
                if guard_pos.1 <= 0 {
                    keep_walking = false;
                } else { // move
                    guard_pos.1 -= 1;
                    // check if turn or not
                    if *data.get(&guard_pos).expect("No Position Found") == Tile::Object {
                        direction.turn();
                        guard_pos.1 += 1;
                    } else {
                        path.insert(guard_pos);
                    }
                }
            },
            Direction::East =>
                // check out of bounds or turning
                if guard_pos.0 >= max_x {
                    keep_walking = false;
                } else { // move
                    guard_pos.0 += 1;
                    // check if turn or not
                    if *data.get(&guard_pos).expect("No Position Found") == Tile::Object {
                        direction.turn();
                        guard_pos.0 -= 1;
                    } else {
                        path.insert(guard_pos);
                    }
                },
            Direction::South => {                // check out of bounds or turning
                if guard_pos.1 >= max_y {
                    keep_walking = false;
                } else { // move
                    guard_pos.1 += 1;
                    // check if turn or not
                    if *data.get(&guard_pos).expect(&format!("No Position Found: {:?}",guard_pos)) == Tile::Object {
                        direction.turn();
                        guard_pos.1 -= 1;
                    } else {
                        path.insert(guard_pos);
                    }
                }},
            Direction::West =>                 // check out of bounds or turning
            if guard_pos.0 <= 0 {
                keep_walking = false;
            } else { // move
                guard_pos.0 -= 1;
                // check if turn or not
                println!("guard_pos: {guard_pos:?}");
                if *data.get(&guard_pos).expect("No Position Found") == Tile::Object {
                    direction.turn();
                    guard_pos.0 += 1;
                } else {
                    path.insert(guard_pos);
                }
            },
        }
        // println!("CurrPos: {guard_pos:?}");
        // println!("Direction: {direction:?}");
    }
    // println!("{path:?}");
    Ok(path.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = 
"....#.....
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

    #[test]
    fn test_turn() {
        let mut dir = Direction::North;
        dir.turn();
        assert_eq!(Direction::East, dir);
        dir.turn();
        assert_eq!(Direction::South, dir);
        dir.turn();
        assert_eq!(Direction::West, dir);
    }
}

// 4768