use std::collections::{HashMap, HashSet};

use crate::{simple_parse, Direction, Tile};


#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let mut data = simple_parse(&input);
    let mut direction = Direction::North;
    let mut path: HashSet<((i32, i32), Direction)> = HashSet::new();

    // get starting location
    let start_v: Vec<(i32, i32)> = data.iter()
        .filter(|(_, t)| *t == &Tile::Guard)
        .map(|((x,y), _)| (*x,*y))
        .collect();
    let guard_pos_start = *start_v.first().expect("no Guard Position found");
    let mut guard_pos = guard_pos_start.clone();
    path.insert((guard_pos, direction));

    let max_x: i32 = (input.lines().last().unwrap().chars().count() - 1).try_into().unwrap();
    let max_y: i32 = (input.lines().count() - 1).try_into().unwrap();

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
                        path.insert((guard_pos, direction));
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
                        path.insert((guard_pos, direction));
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
                        path.insert((guard_pos, direction));
                    }
                }},
            Direction::West =>                 // check out of bounds or turning
                if guard_pos.0 <= 0 {
                    keep_walking = false;
                } else { // move
                    guard_pos.0 -= 1;
                    // check if turn or not
                    if *data.get(&guard_pos).expect("No Position Found") == Tile::Object {
                        direction.turn();
                        guard_pos.0 += 1;
                    } else {
                        path.insert((guard_pos, direction));
                    }
            },
        }
    }

    // Start Checking For Loops from Object placements
    let mut loop_obj_pos: HashSet<(i32,i32)> = HashSet::new();
    for (count, (position, _dir)) in path.iter().enumerate() {
        let mut test_data = data.clone();
        test_data.insert(*position, Tile::Object);
        let mut visited: HashSet<((i32, i32), Direction)> = HashSet::new();
        let mut test_guard_pos = guard_pos_start.clone();
        keep_walking = true;
        direction = Direction::North;

        println!("{}", format!("try {count} / {}", path.clone().iter().count()));

        while keep_walking {
            // move in direction, check
            match direction {
                Direction::North => {
                    // check out of bounds or turning
                    if test_guard_pos.1 <= 0 {
                        keep_walking = false;
                    } else { // move
                        test_guard_pos.1 -= 1;
                        // check if turn or not
                        if *test_data.get(&test_guard_pos).expect("No Position Found") == Tile::Object {
                            direction.turn();
                            test_guard_pos.1 += 1;
                        } else {
                            let has_visited = visited.get(&(test_guard_pos, direction));
                            match has_visited {
                                Some(_) => {loop_obj_pos.insert(*position); keep_walking = false;},
                                None => {visited.insert((test_guard_pos, direction));}
                            }
                        }
                    }
                },
                Direction::East => {
                    // check out of bounds or turning
                    if test_guard_pos.0 >= max_x {
                        keep_walking = false;
                    } else { // move
                        test_guard_pos.0 += 1;
                        // check if turn or not
                        if *test_data.get(&test_guard_pos).expect("No Position Found") == Tile::Object {
                            direction.turn();
                            test_guard_pos.0 -= 1;
                        } else {
                            let has_visited = visited.get(&(test_guard_pos, direction));
                            match has_visited {
                                Some(_) => {loop_obj_pos.insert(*position); keep_walking = false;},
                                None => {visited.insert((test_guard_pos, direction));}
                            }
                        }
                    }},
                Direction::South => {                // check out of bounds or turning
                    if test_guard_pos.1 >= max_y {
                        keep_walking = false;
                    } else { // move
                        test_guard_pos.1 += 1;
                        // check if turn or not
                        if *test_data.get(&test_guard_pos).expect(&format!("No Position Found: {:?}",test_guard_pos)) == Tile::Object {
                            direction.turn();
                            test_guard_pos.1 -= 1;
                        } else {
                            let has_visited = visited.get(&(test_guard_pos, direction));
                            match has_visited {
                                Some(_) => {loop_obj_pos.insert(*position); keep_walking = false;},
                                None => {visited.insert((test_guard_pos, direction));}
                            }
                        }
                    }},
                Direction::West =>                 // check out of bounds or turning
                    if test_guard_pos.0 <= 0 {
                        keep_walking = false;
                    } else { // move
                        test_guard_pos.0 -= 1;
                        // check if turn or not
                        if *test_data.get(&test_guard_pos).expect("No Position Found") == Tile::Object {
                            direction.turn();
                            test_guard_pos.0 += 1;
                        } else {
                            let has_visited = visited.get(&(test_guard_pos, direction));
                            match has_visited {
                                Some(_) => {loop_obj_pos.insert(*position); keep_walking = false;},
                                None => {visited.insert((test_guard_pos, direction));}
                            }
                        }
                },
            }
        }
    }
    Ok(loop_obj_pos.iter().count().to_string())
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
        assert_eq!("6", process(input).unwrap());
    }
}