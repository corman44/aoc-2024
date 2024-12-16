pub mod part1;
pub mod part2;

use glam::IVec2;

pub fn parse(input: &str) -> (Player, IVec2, Vec<Vec<Tile>>){
    let map = input.lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '.' {
                        Tile::Empty
                    } else if c == '#' {
                        Tile::Wall
                    } else if c == 'S' {
                        Tile::Player
                    } else {
                        Tile::End
                    }
                })
                .collect()
        })
        .collect::<Vec<Vec<Tile>>>();

    let p = map.iter().enumerate().flat_map(|(y,v)| {
            v.iter().enumerate().filter(|(_, t)| **t == Tile::Player).map(|(x,_)| {
                Player { loc: IVec2::new(x as i32,y as i32), dir: IVec2::X }
            }).collect::<Vec<_>>()
        })
        .next()
        .expect("no player found");

    let e = map.iter().enumerate().flat_map(|(y,v)| {
        v.iter().enumerate().filter(|(_, t)| **t == Tile::End).map(|(x,_)| {
            IVec2::new(x as i32,y as i32)
        }).collect::<Vec<_>>()
    })
    .next()
    .expect("no player found");

    (p, e, map)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    End,
    Player,
    Wall,
}

trait Turnable{
    fn turn_right(&self) -> Self;
    fn turn_left(&self) -> Self;
}

impl Turnable for IVec2 {
    fn turn_right(&self) -> Self {
        if *self == IVec2::X {
            IVec2::Y
        } else if *self == IVec2::NEG_X {
            IVec2::NEG_Y
        } else if *self == IVec2::NEG_Y {
            IVec2::X
        } else {
            IVec2::NEG_X
        }
    }

    fn turn_left(&self) -> Self {
        if *self == IVec2::X {
            IVec2::NEG_Y
        } else if *self == IVec2::NEG_X {
            IVec2::Y
        } else if *self == IVec2::NEG_Y {
            IVec2::NEG_X
        } else {
            IVec2::X
        }
    }

}

#[derive(Debug, Clone, Copy, Default, Hash, Eq)]
pub struct Player {
    loc: IVec2,
    dir: IVec2,
}

impl PartialEq<IVec2> for Player {
    fn eq(&self, other: &IVec2) -> bool {
        self.loc == *other
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc
    }
}

impl Player {
    pub fn new(location: IVec2, direction: IVec2) -> Self {
        Self {
            dir: direction,
            loc: location,
        }
    }

    pub fn routes(&self, map: &Vec<Vec<Tile>>) -> Vec<(Self, usize)> {
        let mut v: Vec<(Self, usize)> = vec![];

        // Check dirs and push to vec
        if let Some(straight) = self.check_straight(&map) {
            v.push(
                (Self {
                    loc: straight,
                    dir: self.dir,
                },
                1)
            );
        }
        if let Some(right) = self.check_right(&map) {
            v.push(
                (Self {
                    loc: right,
                    dir: self.dir.turn_right(),
                },
                1001)
            );
        }
        if let Some(left) = self.check_left(&map) {
            v.push(
                (Self {
                    loc: left,
                    dir: self.dir.turn_left(),
                },
                1001)
            );
        }
        v
    }

    fn check_straight(&self, map: &Vec<Vec<Tile>>) -> Option<IVec2> {
        let dir_check = self.loc + self.dir;
        if map[dir_check.y as usize][dir_check.x as usize] == Tile::End ||
            map[dir_check.y as usize][dir_check.x as usize] == Tile::Empty {
            Some(dir_check)
        } else {
            None
        }
    }

    fn check_left(&self, map: &Vec<Vec<Tile>>) -> Option<IVec2> {
        let dir_check = self.loc + self.dir.turn_left();
        if map[dir_check.y as usize][dir_check.x as usize] == Tile::End ||
            map[dir_check.y as usize][dir_check.x as usize] == Tile::Empty {
            Some(dir_check)
        } else {
            None
        }
    }

    fn check_right(&self, map: &Vec<Vec<Tile>>) -> Option<IVec2>{
        let dir_check = self.loc + self.dir.turn_right();
        if map[dir_check.y as usize][dir_check.x as usize] == Tile::End ||
            map[dir_check.y as usize][dir_check.x as usize] == Tile::Empty {
            Some(dir_check)
        } else {
            None
        }
    }
}
