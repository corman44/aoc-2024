use std::{fs::File, io::Write};

use crate::{parse, Robot};

const MAX_X: i32 = 100;
const MAX_Y: i32 = 102;
const SIM_RUNS: i32 = 10000;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let mut robots = parse(input);
    let mut outfile = File::create("robot_positions.txt").expect("unable to open file");
    outfile.flush();
    // dbg!(&robots);

    // run simulation step
    for i in 0..SIM_RUNS {
        for r in robots.iter_mut() {
            r.step_forward();
        }
        // println!("{i}");
        outfile.write(format!("\n-------------------------------------------------  {i}  ----------------------------------------------\n").as_bytes());
        show_robots(&robots, &mut outfile);
    }

    // count in each quadrant
    let mut quadrants: Vec<i32> = vec![0; 4];
    for robot in robots {
        if robot.position.x < MAX_X / 2 && robot.position.y < MAX_Y / 2 {
            quadrants[0] += 1;
        } else if robot.position.x < MAX_X / 2 && robot.position.y > MAX_Y / 2 {
            quadrants[1] += 1;
        } else if robot.position.x > MAX_X / 2 && robot.position.y < MAX_Y / 2 {
            quadrants[2] += 1;
        } else if robot.position.x > MAX_X / 2 && robot.position.y > MAX_Y / 2 {
            quadrants[3] += 1;
        }
    }

    Ok(0.to_string())
}

pub fn show_robots(robots: &Vec<Robot>, outfile: &mut File) {
    let mut map: Vec<Vec<i32>> = vec![vec![0; (MAX_X + 1) as usize]; (MAX_Y + 1) as usize];
    for robot in robots {
        map[robot.position.y as usize][robot.position.x as usize] += 1;
    }

    let mut buffer = String::new();
    // print the view
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                // print!(".");
                buffer.push(' ');
            } else {
                // print!("{}", map[y][x]);
                // buffer.push_str(map[y][x].to_string().as_str());
                buffer.push_str("#");
            }
        }
        // println!();
        buffer.push('\n');
    }
    // buffer.push('\n');
    outfile.write(buffer.as_bytes()).expect("unable to write to file");
}

// 7371 XD