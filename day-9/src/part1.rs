
#[tracing::instrument]
pub fn process(
    input: &str,
) -> Result<String, String> {
    let answer = 0;
    let disk = parse(input);
    dbg!(disk);
    Ok(answer.to_string())
}

fn parse(input: &str) -> Vec<Block> {
    let mut acc = 0;
    input.chars()
        .enumerate()
        .map(move |(start_idx, c)| {
            if start_idx % 2 == 0 { // File
                let b = Block {
                    start_idx: acc,
                    file_id: Some((start_idx / 2) as u32),
                    len: c.to_digit(10).expect("not a digit"),
                    typ: BlockType::File,
                };
                acc += c.to_digit(10).unwrap();
                b
            } else { // Empty
                let b = Block {
                    start_idx: acc,
                    file_id: None,
                    len: c.to_digit(10).expect("not a digit"),
                    typ: BlockType::Free,
                };
                acc += c.to_digit(10).unwrap();
                b
            }
        })
        .collect()
}

#[derive(Debug)]
enum BlockType {
    Free,
    File
}

#[derive(Debug)]
struct Block {
    pub typ: BlockType,
    pub start_idx: u32,
    pub file_id: Option<u32>,
    pub len: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input).unwrap());
    }
}