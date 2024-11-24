use clap::{error::ErrorKind, CommandFactory, Parser};
use reqwest::{self, blocking::Client, header::COOKIE};
use nom::{
    bytes::complete::tag, character::complete,
    sequence::preceded, IResult,
};
use std::{fs::File, io::Write, path::PathBuf};

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
    cwd: PathBuf,
}

fn parse_day(input: &str) -> IResult<&str, u32> {
    preceded(tag("day-"), complete::u32)(input)
}

fn session_parser(input: &str) -> IResult<&str, &str>
{
    tag("SESSION=")(input)
}

fn get_session() -> Result<String, String>
{
    let mut dir = std::env::current_dir().unwrap();
    dir.push(".env");
    let env_string = std::fs::read_to_string(dir).expect("unable to find .env file");

    for line in env_string.lines() {
        if line.contains("SESSION=") {
            let (first, _second) = session_parser(&line).unwrap();
            return Ok(first.to_string());
        }
        else {
            return Err("SESSION key not found..".to_string())
        }
    }
    Err("shouldn't reach here..".to_owned())
}



fn main() -> Result<(), reqwest::Error> {
    if let Ok(session) = get_session() {
        let args = Args::parse();
        let Ok((_, day)) = parse_day(&args.day) else {
            let mut cmd = Args::command();
            cmd.error(
                ErrorKind::ValueValidation,
                format!(
                    "day '{}' must be formatted as 'day-01'",
                    args.day
                ),
            )
            .exit()
        };

        let url = format!("https://adventofcode.com/2024/day/{day}/input");
        let client = Client::new();
        let input_data = client
            .get(url)
            .header(COOKIE, format!("session={session}"))
            .send()?
            .text()?;

        for filename in ["input1.txt", "input2.txt"] {
            // println!("{:?}", args.cwd);
            let file_path = args
                .cwd
                .join(&args.day)
                .join(filename);
            // println!("{}", format!("filepath: {file_path:?}"));
            let mut file = File::create(&file_path)
                .expect("should be able to create a file");
            file.write_all(input_data.as_bytes())
                .expect("should be able to write to input file");
            // println!("wrote {}", file_path.display())
        }
    }
    Ok(())
}