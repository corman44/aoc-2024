use std::collections::HashMap;

pub mod part1;
pub mod part2;


#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    #[default]
    Empty,
    Guard,
    Object,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction{
    pub fn turn(&mut self) {
        *self = match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

fn simple_parse(input: &str) -> HashMap<(i32, i32), Tile> {
    let mut x = 0;
    let mut y = 0;
    // let mut result: HashMap<(i32, i32), Tile> = HashMap::new();
    input.lines()
        .enumerate()
        .flat_map(|(y,line)| {
            line.chars()
                .enumerate()
                .map(move |(x,c)| {
                    if c == '.' {
                        ((x as i32,y as i32), Tile::Empty)
                    } else if c == '^' {
                        ((x as i32,y as i32), Tile::Guard)
                    } else {
                        ((x as i32,y as i32), Tile::Object)
                    }
                })
        })
        .collect::<HashMap<(i32, i32), Tile>>()
}