use std::ops::Add;

use nom::InputIter;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let keys = parse(input);
    // dbg!(&keys);

    let keyholes: Vec<Key> = keys.clone().into_iter().filter(|k| k.style == KeyType::KeyHole).collect();
    let keys: Vec<Key> = keys.clone().into_iter().filter(|k| k.style == KeyType::Key).collect();

    let mut key_fits = 0;
    for key in keys {
        key_fits += keyholes.iter().filter(|kh| kh.fits(&key)).count();
    }

    Ok(key_fits.to_string())
}

const MAX_X: usize = 5;
const MAX_Y: usize = 7;

pub fn parse(input: &str) -> Vec<Key> {
    input.split("\n\n")
        .map(|schem| {
            let mut out: Vec<u8> = vec![0; schem.lines().next().unwrap().len()];
            for each in schem.lines() {
                for (idx, ch) in each.iter_elements().enumerate() {
                    if ch == '#' { out[idx] += 1}
                }
            }
            if schem.lines().next().unwrap().iter_elements().all(|c| c=='#') {
                Key {
                    schematic: out,
                    style: KeyType::Key,
                }
            } else {
                Key {
                    schematic: out,
                    style: KeyType::KeyHole,
                }
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum KeyType {
    Key,
    KeyHole,
}

impl Key {
    pub fn fits(&self, key: &Key) -> bool {
        let sum: Vec<u8> = self.schematic.clone()
            .iter()
            .enumerate()
            .map(|(idx, v)| {
                v + key.schematic[idx]
            })
            .collect();
        sum.iter().all(|v| *v <= MAX_Y as u8)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    schematic: Vec<u8>,
    style: KeyType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";
        assert_eq!("3", process(input).unwrap());
    }
}