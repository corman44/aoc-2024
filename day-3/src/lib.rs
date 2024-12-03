use nom::{ bytes::{complete::tag, streaming::take_until}, character::complete::{self, anychar}, multi::{many0, many_till}, sequence::{delimited, separated_pair}, IResult, Parser};

pub mod part1;
pub mod part2;

pub fn parser(input: &str) -> IResult<&str, Vec<(u32,u32)>> {
    many0(
        many_till(
            anychar,
            parse_pair
        ).map(|(_dis, res)| res),
    )(input)
}

pub fn parse_pair(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, _) = tag("mul")(input)?;
    // let (input, pair) = 
    delimited(
        tag("("),
        separated_pair(
            complete::u32,
            tag(","),
            complete::u32
        ),
        tag(")")
    )(input)
}

pub fn parse_do_dont(input: &str) -> IResult<&str, &str> {
    delimited(
        tag("do()"), 
        take_until("don't"),
        tag("don't()") 
    )(input)
}

fn parse_til_dont(input: &str) -> IResult<&str, &str> {
    take_until("don't")(input)
}

pub fn parser2(input: &str) -> IResult<&str, Vec<Vec<(u32,u32)>>> {
    let (_, start) = parse_til_dont(input.clone()).expect("unable to get first text before Don't");
    let (_, mut v1) = parser(start).unwrap();
    println!("v1: {v1:?}");

    let (s, mut v2) = many0(
        many_till(
            anychar,
            parse_do_dont
        ).map(|(_dis, do_it)| {
            if let Ok((_x, res)) = parser(do_it) {res}
            else {
                vec![(0,1)]
            }
        }),
    )(input).expect("unable to parse dos and don't");

    v2.extend(vec![v1]);
    
    return Ok((s,v2));
}