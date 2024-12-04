use nom::InputIter;

/* Process:
    - Recursively search through each of the Paths and spin off a new recursive loop until XMAS have all b
    - 
 */


#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let keys = vec!['M', 'A', 'S'];
    let puzzle = parse(input);
    let ans: i32 = puzzle
        .iter()
        .enumerate()
        .map(|(row,line)| {
            line.iter()
                .enumerate()
                .map(|(col, c)| {
                    if *c == 'X' {
                        // TODO recursivley check all direction and return sum
                        check_recursive(puzzle.clone(), &keys, col, row) as i32
                    } else { 0 }
                }).sum::<i32>()
        })
        .sum();
    Ok(ans.to_string())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    N, NE, E, SE, S, SW, W, NW
}

fn check_recursive(puzzle: Vec<Vec<char>>, keys: &Vec<char>, x: usize, y: usize) -> usize {

}

fn check_all_dir(puzzle: Vec<Vec<char>>, key: &char, x: usize, y: usize) -> Result<Vec<Direction>, ()> {
    // TODO handle min and max value for X and Y

    let mut res: Vec<Direction> = vec![];
    if puzzle[y-1][x] == *key {res.push(Direction::N)}
    if puzzle[y-1][x+1] == *key {res.push(Direction::NE)}
    if puzzle[y][x+1] == *key {res.push(Direction::E)}
    if puzzle[y+1][x+1] == *key {res.push(Direction::SE)}
    if puzzle[y+1][x] == *key {res.push(Direction::S)}
    if puzzle[y+1][x-1] == *key {res.push(Direction::SW)}
    if puzzle[y][x-1] == *key {res.push(Direction::W)}
    if puzzle[y-1][x-1] == *key {res.push(Direction::NW)}
    if res.is_empty() { return Err(())}
    else { return Ok(res)}
}

fn check_new_dir(dir: Direction) -> (i32, i32) {
    match dir {
        Direction::N => (-1, 0),
        Direction::NE => (-1, 1),
        Direction::E => (0, 1),
        Direction::SE => (1, 1),
        Direction::S => (1, 0),
        Direction::SW => (1, -1),
        Direction::W => (0, -1),
        Direction::NW => (-1, -1),
    }
}

/// not sure if needed but whatever..
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == 'X' || c == 'M' || c == 'A' || c == 'S' {
                        c
                    } else {
                        '.'
                    }
                }
            ).collect()
        }
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        assert_eq!("18", process(input).unwrap());
    }

    #[test]
    fn test_check_all_dir() {
        let input = 
        "MSMSMSMSMSMSM\nMSXSMSMSMSMSS\nMSMSMSMSMSMSM";
        let puzzle = parse(input);
        println!("{puzzle:?}");
        let res = check_all_dir(puzzle, &'M', 2, 1);
        let ans = vec![
            Direction::N,
            Direction::S,
        ];
        assert_eq!(ans, res.unwrap());
    }
}