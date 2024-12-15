use glam::IVec2;
use nom::InputIter;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let (mut warehouse, moves) = parse(input);
    let mut robot_loc: IVec2 = warehouse.iter().enumerate().flat_map(|(y,v)| {
        v.iter().enumerate().filter(|(_, t)| **t == Tile::Robot).map(|(x,_)| IVec2::new(x as i32,y as i32)).collect::<Vec<_>>()
    }).next().expect("No Robot Found");

    // Small test of is_dir_possible
    // let test_start = IVec2::new(4,7);
    // let test_moves = is_dir_possible(&warehouse, test_start, LEFT, 0);
    // dbg!(test_moves);

    // dbg!(&moves);
    // dbg!(&warehouse);

    // run through the moves
    for dir in moves.iter() {
        // display_map(&warehouse);
        match dir {
            Direction::Left => {
                if let Some(box_moves) = is_dir_possible(&warehouse, robot_loc, LEFT, 0) {
                    let new_loc = robot_loc + LEFT;
                    warehouse[new_loc.y as usize][new_loc.x as usize - box_moves] = Tile::Box;
                    warehouse[new_loc.y as usize][new_loc.x as usize] = Tile::Robot;
                    warehouse[robot_loc.y as usize][robot_loc.x as usize] = Tile::Empty;

                    robot_loc = new_loc;
                } // else do nothing since can't move in that direction
            },
            Direction::Up => {
                if let Some(box_moves) = is_dir_possible(&warehouse, robot_loc, UP, 0) {
                    let new_loc = robot_loc + UP;
                    warehouse[new_loc.y as usize - box_moves][new_loc.x as usize] = Tile::Box;
                    warehouse[new_loc.y as usize][new_loc.x as usize] = Tile::Robot;
                    warehouse[robot_loc.y as usize][robot_loc.x as usize] = Tile::Empty;

                    robot_loc = new_loc;
                } // else do nothing since can't move in that direction
            },
            Direction::Right => {
                if let Some(box_moves) = is_dir_possible(&warehouse, robot_loc, RIGHT, 0) {
                    let new_loc = robot_loc + RIGHT;
                    warehouse[new_loc.y as usize][new_loc.x as usize + box_moves] = Tile::Box;
                    warehouse[new_loc.y as usize][new_loc.x as usize] = Tile::Robot;
                    warehouse[robot_loc.y as usize][robot_loc.x as usize] = Tile::Empty;

                    robot_loc = new_loc;
                } // else do nothing since can't move in that direction
            },
            Direction::Down => {
                if let Some(box_moves) = is_dir_possible(&warehouse, robot_loc, DOWN, 0) {
                    let new_loc = robot_loc + DOWN;
                    warehouse[new_loc.y as usize + box_moves][new_loc.x as usize] = Tile::Box;
                    warehouse[new_loc.y as usize][new_loc.x as usize] = Tile::Robot;
                    warehouse[robot_loc.y as usize][robot_loc.x as usize] = Tile::Empty;

                    robot_loc = new_loc;
                } // else do nothing since can't move in that direction
            },
        }
    }

    // display_map(&warehouse);

    let boxes: Vec<IVec2> = warehouse.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x,c)| {
                    match *c {
                        Tile::Box => Some(IVec2::new(x as i32,y as i32)),
                        _ => None,
                    }
                }).collect::<Vec<_>>()
        }).collect();
    let answer: i32 = boxes.iter()
        .map(|b| {
            b.y * 100 + b.x
        })
        .sum();

    Ok(answer.to_string())
}

pub fn display_map(wh: &Vec<Vec<Tile>>) {
    for y in 0..wh.len() {
        for x in 0..wh.iter().next().unwrap().len() {
            match wh[y][x] {
                Tile::Edge => print!("#"),
                Tile::Empty => print!("."),
                Tile::Box => print!("O"),
                Tile::Robot => print!("@"),
            }
        }
        println!();
    }
}

/// if not possible: None, if possible, Some(num) of number of boxes to move
pub fn is_dir_possible(wh: &Vec<Vec<Tile>>, start: IVec2, dir: IVec2, prev_count: usize) -> Option<usize> {
    match wh[(start.y + dir.y) as usize][(start.x + dir.x) as usize] {
        Tile::Empty => Some(prev_count),
        Tile::Box => is_dir_possible(wh, start + dir, dir, prev_count+1),
        _ => None,
    }
}

const LEFT: IVec2 = IVec2::new(-1,0);
const RIGHT: IVec2 = IVec2::new(1,0);
const UP: IVec2 = IVec2::new(0,-1);
const DOWN: IVec2 = IVec2::new(0,1);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Edge,
    Empty,
    Box,
    Robot,
}

pub fn parse(input: &str) -> (Vec<Vec<Tile>>, Vec<Direction>){
    let mut first_split = input.split("\n\n");
    let raw_wh = first_split.next().expect("no warehouse found");
    let raw_moves = first_split.next().expect("no moves found");

    let wh: Vec<Vec<Tile>> = raw_wh.lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '#' {
                        Tile::Edge
                    } else if c == '.' {
                        Tile::Empty
                    } else if c == 'O' {
                        Tile::Box
                    } else {
                        Tile::Robot
                    }
                }).collect()
        }).collect();

    let moves: Vec<Direction> = raw_moves.iter_elements()
        .filter(|c| *c != '\n')
        .map(|m| {
            if m == '<' { Direction::Left } 
            else if m == '^' { Direction::Up }
            else if m == 'v' { Direction::Down }
            else { Direction::Right }
        })
        .collect();

    (wh, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
        assert_eq!("10092", process(input).unwrap());
    }
}