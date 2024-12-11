use glam::IVec2;

const NORTH: IVec2 = IVec2::new(0, -1);
const SOUTH: IVec2 = IVec2::new(0, 1);
const EAST: IVec2 = IVec2::new(1, 0);
const WEST: IVec2 = IVec2::new(-1, 0);

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let topo_map = parse(input);
    let all_dirs = vec![NORTH, SOUTH, EAST, WEST];

    let trailheads: Vec<(usize, usize)> = topo_map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, h)| if *h == 0 {
                    Some((x,y))
                } else {
                    None
                }
            )
        }).collect();

    // For each Trailhaed in trailheads ..
    //      find_paths
    for th in trailheads.iter() {

    }

    Ok(0.to_string())
}

fn recur_path(map: &Vec<Vec<u8>>, location: (usize, usize)) -> Option<Vec<(u8, u8)>> {

}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("36", process(input).unwrap());
    }
}