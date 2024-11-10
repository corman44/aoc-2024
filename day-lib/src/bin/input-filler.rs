use nom::character::complete::digit0;
use nom::error::{ErrorKind, ParseError};
use reqwest::{self, blocking::Client, header::COOKIE};
use clap::{CommandFactory, Parser};
use nom::{
    bytes::complete::tag, character::complete,
    sequence::preceded, IResult,
};
use std::fmt::{format, Error};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// day is expected to be formatted as
    /// `day-01` to match all other commands in
    /// the repo
    #[clap(short, long)]
    day: String,
    /// a way to pass in the justfile directory
    /// so that we're always in the root without
    /// doing any shenanigans
    #[clap(long)]
    current_working_directory: PathBuf,
}

fn parse_day(input: &str) -> IResult<&str, u32> {
    preceded(tag("day-"), complete::u32)(input)
}

fn session_parser(input: &str) -> IResult<&str, &str>
{
    tag("SESSION=")(input)
}

fn get_session() -> Result<String, ErrorKind>
{
    let mut dir = std::env::current_dir().unwrap();
    dir.push(".env");
    let env_string = std::fs::read_to_string(dir).unwrap();

    for line in env_string.lines() {
        if line.contains("SESSION=") {
            let (first, _second) = session_parser(&line).unwrap();
            // println!("{}", format!("first: {first}, second: {second}"));
            return Ok(first.to_string());
        }
    }
    Ok(String::from("ERROR"))
}



fn main() -> Result<(), reqwest::Error> {
    if let Ok(session) = get_session() {
        println!("{}", format!("Session = {session}"));
        // let args = Args::parse();
        let url = "https://adventofcode.com/2020/day/1/input";
        let client = Client::new();
        let input_data = client
            .get(url)
            .header(COOKIE, format!("session={session}"))
            .send()?
            .text()?;

        println!("Text: \n{:?}", input_data);
    }
    Ok(())
}