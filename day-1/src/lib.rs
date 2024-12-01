use nom::{branch::alt, character::complete::{digit0, multispace0}, combinator::map_res, sequence::{delimited, separated_pair}, IResult};

pub mod part1;
pub mod part2;

pub fn ws_parser(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(parse_u32, multispace0, parse_u32)(input)
}

fn parse_u32(input: &str) -> IResult<&str, u32>{
    map_res(digit0, str::parse)(input)
}