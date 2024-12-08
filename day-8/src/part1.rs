use std::collections::HashMap;
use glam::IVec2;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let antennas = parse(input);

    let max_coords = IVec2::new(
        input.lines().next().unwrap().len() as i32,
        input.lines().count() as i32
    );
    // dbg!(max_coords);

    let anti_antennas = antennas.iter()
        .flat_map(|(_c, coords)|{
            let mut cords = coords.clone();
            cords.sort_by(|a,b| a.y.cmp(&b.y));
            cords.iter()
                .combinations(2)
                .flat_map(|x| {
                    vec![
                        IVec2::new(x[0].x*2 - x[1].x, x[0].y*2 - x[1].y),
                        IVec2::new(x[1].x*2 - x[0].x, x[1].y*2 - x[0].y)
                        ]
                }).collect::<Vec<IVec2>>()
        })
        .filter(|a| {
            // dbg!(a);
            check_bounds(*a, max_coords) 
            // && !antenna_loc(&antennas, a)
        });
    let ans = anti_antennas.unique().count();
    Ok(ans.to_string())
}

fn antenna_loc(antennas: &HashMap<char, Vec<IVec2>>, location: &IVec2) -> bool {
    for (_key, ant) in antennas.iter() {
        if ant.contains(location) {
            // dbg!(location);
            return true
        }
    }
    false
}

fn check_bounds(coord: IVec2, max: IVec2) -> bool {
    coord.x >= 0 && coord.x < max.x && coord.y >= 0 && coord.y < max.y
}

fn parse(input: &str) -> HashMap<char, Vec<IVec2>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c != '.')
                .map(move |(x, c)| (c, (x as i32, y as i32)))
        })
        .fold(HashMap::new(), |mut acc, (c, coord)| {
            acc.entry(c).or_insert_with(Vec::new).push(IVec2::new(coord.0, coord.1));
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input).unwrap());
    }
}

/* try: 
452
440
417
392
 */