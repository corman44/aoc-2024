use crate::parse;

const MAX_X: i32 = 100;
const MAX_Y: i32 = 102;
const SIM_RUNS: i32 = 100;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let mut robots = parse(input);
    // dbg!(&robots);

    // run simulation step
    for i in 0..SIM_RUNS {
        for r in robots.iter_mut() {
            r.step_forward();
        }
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

    dbg!(quadrants.clone()); // should have 1, 3, 4, 1
    let ans = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];

    Ok(ans.to_string())
}

#[cfg(test)]
mod tests {
    use glam::IVec2;

    use crate::Robot;

    use super::*;

    #[test]
    fn test_process() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
        assert_eq!("12", process(input).unwrap());
    }

    #[test]
    fn test_robot_movement() {
        let mut r1 = Robot {
            position: IVec2::new(1, MAX_Y - 1),
            velocity: IVec2::new(-1, 1),
        };

        r1.step_forward();
        assert_eq!(
            IVec2::new(0, MAX_Y),
            r1.position
        );

        r1.step_forward();
        assert_eq!(
            IVec2::new(MAX_X, 0),
            r1.position
        );
    }
}

/* Tries
7458228 - too low
 */