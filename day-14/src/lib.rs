pub mod part1;
pub mod part2;

use glam::IVec2;
use nom::{bytes::complete::tag, character::complete::{self, newline, space0}, multi::many0, sequence::{preceded, separated_pair, terminated, tuple}, IResult, Parser};

const MAX_X: i32 = 100;
const MAX_Y: i32 = 102;
const SIM_RUNS: i32 = 100;

fn parse_p(input: &str) -> IResult<&str, IVec2> {
    preceded(
        tag("p="),
        separated_pair(
            complete::i32, 
            tag(","),
            complete::i32,
        ).map(|(x,y)| IVec2::new(x,y))
    )(input)
}

fn parse_v(input: &str) -> IResult<&str, IVec2> {
    preceded(
        tag("v="),
        separated_pair(
            complete::i32, 
            tag(","),
            complete::i32,
        ).map(|(x,y)| IVec2::new(x,y))
    )(input)
}

pub fn parse(input: &str) -> Vec<Robot> {
    many0(
        terminated(
            separated_pair(
                parse_p, 
                space0,
                parse_v,
            ),
            newline
        ).map(|(p,v)| {
            Robot {
                position: p,
                velocity: v,
            }
        })
    )(input).expect("Robot parsing err..").1
}

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    pub position: IVec2,
    pub velocity: IVec2,
}

impl Robot {
    pub fn step_forward(&mut self) {
        self.position += self.velocity;

        // check limits and wrap
        if self.position.x > MAX_X {
            self.position.x = (self.position.x % MAX_X) - 1;
        } else if self.position.x < 0 {
            self.position.x = self.position.x + MAX_X + 1;
        }

        if self.position.y > MAX_Y {
            self.position.y = (self.position.y % MAX_Y) - 1;
        } else if self.position.y < 0 {
            self.position.y = self.position.y + MAX_Y + 1;
        }
    }
}