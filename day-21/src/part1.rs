use glam::IVec2;
use pathfinding::prelude::*;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {

    let numeric_keypad = vec![
        vec![7,8,9],
        vec![4,5,6],
        vec![1,2,3],
        vec![15,0,10],
    ];

    let directional_keypad = vec![
        vec![Keypad::None, Keypad::Up,Keypad::Action],
        vec![Keypad::Left,Keypad::Left,Keypad::Right],
    ];

    let codes = parse(input);
    // dbg!(&codes);

    let nums = codes.iter()
        .map(|c| {
            let mut s = c.clone();
            s.pop();
            let x = s.iter().collect::<String>();
            x.parse::<i32>().expect("unable to parse to i32")
        }).collect::<Vec<i32>>();
    // dbg!(&nums);

    let robot1 = Robot::new(2,0); // directional keypad
    let robot2 = Robot::new(2,0); // directional keypad
    let robot3 = Robot::new(2,3); // numeric keypad     
    let me = Robot::new(2,0);

    // main logic
    let mut sum = 0;
    for (idx, code) in codes.iter().enumerate() {
        let mut my_path: Vec<Keypad> = vec![];
        // TODO pathfind to the first number
        // TODO pathfind to those moves for robot 2
        // TODO pathfind to those moves for robot 3
        // TODO pathfind to those moves for me
        // TODO length of moves for me * numeric part of the code and add to sum
        sum += my_path.len() as i32 * nums[idx];
    }
    Ok(sum.to_string())
}

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c
                }).collect()
        }).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keypad {
    Up,
    Down,
    Left,
    Right,
    Action,
    None,
}

const MAX_X_KEYPAD: usize = 2;
const MAX_Y_KEYPAD: usize = 1;
const MAX_X_NUMERIC: usize = 2;
const MAX_Y_NUMERIC: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Robot {
    position: IVec2,
}

impl Robot {

    pub fn new(x: i32, y: i32) -> Self {
        Robot { position: IVec2::new(x,y) }
    }

    pub fn routes(&self) -> Vec<(Self,usize)> {
        let mut v: Vec<(Self, usize)> = vec![];

        if let Some(left) = self.check_left() {
            v.push((Robot {position: left}, 1));
        }
        if let Some(right) = self.check_right() {
            v.push((Robot {position: right}, 1));
        }
        if let Some(up) = self.check_up() {
            v.push((Robot {position: up}, 1));
        }
        if let Some(down) = self.check_down() {
            v.push((Robot {position: down}, 1));
        }
        v
    }

    fn check_left(&self) -> Option<IVec2> {
        if self.position.x == 0 { None }
        else { Some(self.position + IVec2::NEG_X) }
    }

    pub fn check_right(&self) -> Option<IVec2> {
        if self.position.x >= self.max_x { None }
        else { Some(self.position + IVec2::X) }
    }

    pub fn check_up(&self) -> Option<IVec2> {
        if self.position.y == 0 { None }
        else { Some(self.position + IVec2::NEG_Y) }
    }

    pub fn check_down(&self) -> Option<IVec2> {
        if self.position.y >= self.max_y { None }
        else { Some(self.position + IVec2::Y) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "029A
980A
179A
456A
379A";
        assert_eq!("126384", process(input).unwrap());
    }
}