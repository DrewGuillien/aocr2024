use aocr2024::read_file_to_string;

#[derive(Debug, PartialEq, Copy, Clone)]
struct FileBlock {
    id: usize,
    position: usize,
}

fn main() {
    // Part 1
    let blocks = parse_input("./src/bin/day9/input.txt");
    let compacted_blocks = compact_blocks(&blocks);
    let checksum = compute_checksum(compacted_blocks);
    println!("Checksum: {}", checksum);

    // Part 2
    let blocks = parse_input("./src/bin/day9/input.txt");
    let compacted_files = compact_files(&blocks);
    let checksum = compute_checksum(compacted_files);
    println!("Checksum: {}", checksum);
}

fn parse_input(file_path: &str) -> Vec<FileBlock> {
    let file_format = read_file_to_string(file_path);
    let mut blocks = Vec::new();
    let mut id = 0;
    let mut position = 0;
    let mut parsing_file = true;
    for char in file_format.chars() {
        if parsing_file {
            let size = char.to_digit(10).unwrap();
            for _ in 0..size {
                blocks.push(FileBlock { id, position });
                position += 1;
            }
            id += 1;
        } else {
            let space = char.to_digit(10).unwrap();
            position += space as usize;
        }
        parsing_file = !parsing_file;
    }
    blocks
}

fn compact_blocks(blocks: &Vec<FileBlock>) -> Vec<FileBlock> {
    let mut compacted_blocks = Vec::new();
    let mut front_index = 0;
    let mut back_index = blocks.len() - 1;
    let mut position = 0;
    while front_index < back_index + 1 {
        let front_block = blocks[front_index];
        let next_block = blocks[front_index + 1];
        compacted_blocks.push(FileBlock {
            id: front_block.id,
            position,
        });

        let distance = next_block.position - front_block.position;
        if distance > 1 {
            for _ in 1..distance {
                position += 1;
                let back_block = blocks[back_index];
                compacted_blocks.push(FileBlock {
                    id: back_block.id,
                    position,
                });
                back_index -= 1;
            }
        }
        position += 1;
        front_index += 1;
    }
    compacted_blocks
}

fn compact_files(blocks: &Vec<FileBlock>) -> Vec<FileBlock> {
    let mut compacted_files = blocks.clone();
    let mut current_id = blocks.last().map(|block| block.id);
    while let Some(id) = current_id {
        let file = compacted_files.iter()
            .enumerate()
            .filter(|(_, block)| block.id == id)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let gap = compacted_files
            .windows(2)
            .enumerate()
            .find(|(index, window)| {
                *index < file[0] &&
                window[1].position - window[0].position > file.len()
            })
            .map(|(index, block)| (index + 1, block[0].position + 1));
        
        if let Some((index, position)) = gap {
            for (i, old_index) in file.iter().enumerate() {
                let mut new_block = compacted_files.remove(*old_index);
                new_block.position = position + i;
                compacted_files.insert(index + i, new_block);
            }
        }

        current_id = if id > 0 { Some(id - 1) } else { None };
    }

    compacted_files
}

fn compute_checksum(blocks: Vec<FileBlock>) -> usize {
    blocks.iter().fold(0, |acc, block| acc + block.id * block.position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let expected = vec![
            FileBlock { id: 0, position: 0 },
            FileBlock { id: 0, position: 1 },
            FileBlock { id: 1, position: 5 },
            FileBlock { id: 1, position: 6 },
            FileBlock { id: 1, position: 7 },
            FileBlock { id: 2, position: 11 },
            FileBlock { id: 3, position: 15 },
            FileBlock { id: 3, position: 16 },
            FileBlock { id: 3, position: 17 },
            FileBlock { id: 4, position: 19 },
            FileBlock { id: 4, position: 20 },
            FileBlock { id: 5, position: 22 },
            FileBlock { id: 5, position: 23 },
            FileBlock { id: 5, position: 24 },
            FileBlock { id: 5, position: 25 },
            FileBlock { id: 6, position: 27 },
            FileBlock { id: 6, position: 28 },
            FileBlock { id: 6, position: 29 },
            FileBlock { id: 6, position: 30 },
            FileBlock { id: 7, position: 32 },
            FileBlock { id: 7, position: 33 },
            FileBlock { id: 7, position: 34 },
            FileBlock { id: 8, position: 36 },
            FileBlock { id: 8, position: 37 },
            FileBlock { id: 8, position: 38 },
            FileBlock { id: 8, position: 39 },
            FileBlock { id: 9, position: 40 },
            FileBlock { id: 9, position: 41 },
        ];
        assert_eq!(parse_input("./src/bin/day9/sample_input.txt"), expected);
    }

    #[test]
    fn test_compacting_blocks() {
        //00...111...2...333.44.5555.6666.777.888899
        let blocks = vec![
            FileBlock { id: 0, position: 0 },
            FileBlock { id: 0, position: 1 },
            FileBlock { id: 1, position: 5 },
            FileBlock { id: 1, position: 6 },
            FileBlock { id: 1, position: 7 },
            FileBlock { id: 2, position: 11 },
            FileBlock { id: 3, position: 15 },
            FileBlock { id: 3, position: 16 },
            FileBlock { id: 3, position: 17 },
            FileBlock { id: 4, position: 19 },
            FileBlock { id: 4, position: 20 },
            FileBlock { id: 5, position: 22 },
            FileBlock { id: 5, position: 23 },
            FileBlock { id: 5, position: 24 },
            FileBlock { id: 5, position: 25 },
            FileBlock { id: 6, position: 27 },
            FileBlock { id: 6, position: 28 },
            FileBlock { id: 6, position: 29 },
            FileBlock { id: 6, position: 30 },
            FileBlock { id: 7, position: 32 },
            FileBlock { id: 7, position: 33 },
            FileBlock { id: 7, position: 34 },
            FileBlock { id: 8, position: 36 },
            FileBlock { id: 8, position: 37 },
            FileBlock { id: 8, position: 38 },
            FileBlock { id: 8, position: 39 },
            FileBlock { id: 9, position: 40 },
            FileBlock { id: 9, position: 41 },
        ];
        let compacted_blocks = compact_blocks(&blocks);
        // 0099811188827773336446555566
        let expected = vec![
            FileBlock { id: 0, position: 0 },
            FileBlock { id: 0, position: 1 },
            FileBlock { id: 9, position: 2 },
            FileBlock { id: 9, position: 3 },
            FileBlock { id: 8, position: 4 },
            FileBlock { id: 1, position: 5 },
            FileBlock { id: 1, position: 6 },
            FileBlock { id: 1, position: 7 },
            FileBlock { id: 8, position: 8 },
            FileBlock { id: 8, position: 9 },
            FileBlock { id: 8, position: 10 },
            FileBlock { id: 2, position: 11 },
            FileBlock { id: 7, position: 12 },
            FileBlock { id: 7, position: 13 },
            FileBlock { id: 7, position: 14 },
            FileBlock { id: 3, position: 15 },
            FileBlock { id: 3, position: 16 },
            FileBlock { id: 3, position: 17 },
            FileBlock { id: 6, position: 18 },
            FileBlock { id: 4, position: 19 },
            FileBlock { id: 4, position: 20 },
            FileBlock { id: 6, position: 21 },
            FileBlock { id: 5, position: 22 },
            FileBlock { id: 5, position: 23 },
            FileBlock { id: 5, position: 24 },
            FileBlock { id: 5, position: 25 },
            FileBlock { id: 6, position: 26 },
            FileBlock { id: 6, position: 27 },
        ];
        assert_eq!(compacted_blocks, expected);
    }

    #[test]
    fn test_compacting_files() {
        //00...111...2...333.44.5555.6666.777.888899
        let blocks = vec![
            FileBlock { id: 0, position: 0 },
            FileBlock { id: 0, position: 1 },
            FileBlock { id: 1, position: 5 },
            FileBlock { id: 1, position: 6 },
            FileBlock { id: 1, position: 7 },
            FileBlock { id: 2, position: 11 },
            FileBlock { id: 3, position: 15 },
            FileBlock { id: 3, position: 16 },
            FileBlock { id: 3, position: 17 },
            FileBlock { id: 4, position: 19 },
            FileBlock { id: 4, position: 20 },
            FileBlock { id: 5, position: 22 },
            FileBlock { id: 5, position: 23 },
            FileBlock { id: 5, position: 24 },
            FileBlock { id: 5, position: 25 },
            FileBlock { id: 6, position: 27 },
            FileBlock { id: 6, position: 28 },
            FileBlock { id: 6, position: 29 },
            FileBlock { id: 6, position: 30 },
            FileBlock { id: 7, position: 32 },
            FileBlock { id: 7, position: 33 },
            FileBlock { id: 7, position: 34 },
            FileBlock { id: 8, position: 36 },
            FileBlock { id: 8, position: 37 },
            FileBlock { id: 8, position: 38 },
            FileBlock { id: 8, position: 39 },
            FileBlock { id: 9, position: 40 },
            FileBlock { id: 9, position: 41 },
        ];
        let compacted_files = compact_files(&blocks);
        // 00992111777.44.333....5555.6666.....8888..
        let expected = vec![
            FileBlock { id: 0, position: 0 },
            FileBlock { id: 0, position: 1 },
            FileBlock { id: 9, position: 2 },
            FileBlock { id: 9, position: 3 },
            FileBlock { id: 2, position: 4 },
            FileBlock { id: 1, position: 5 },
            FileBlock { id: 1, position: 6 },
            FileBlock { id: 1, position: 7 },
            FileBlock { id: 7, position: 8 },
            FileBlock { id: 7, position: 9 },
            FileBlock { id: 7, position: 10 },
            FileBlock { id: 4, position: 12 },
            FileBlock { id: 4, position: 13 },
            FileBlock { id: 3, position: 15 },
            FileBlock { id: 3, position: 16 },
            FileBlock { id: 3, position: 17 },
            FileBlock { id: 5, position: 22 },
            FileBlock { id: 5, position: 23 },
            FileBlock { id: 5, position: 24 },
            FileBlock { id: 5, position: 25 },
            FileBlock { id: 6, position: 27 },
            FileBlock { id: 6, position: 28 },
            FileBlock { id: 6, position: 29 },
            FileBlock { id: 6, position: 30 },
            FileBlock { id: 8, position: 36 },
            FileBlock { id: 8, position: 37 },
            FileBlock { id: 8, position: 38 },
            FileBlock { id: 8, position: 39 },
        ];
        assert_eq!(compacted_files, expected);
    }

    #[test]
    fn test_checksum() {
        let blocks = vec![
            FileBlock { id: 0, position: 0 },
            FileBlock { id: 0, position: 1 },
            FileBlock { id: 1, position: 5 },
            FileBlock { id: 1, position: 6 },
            FileBlock { id: 1, position: 7 },
            FileBlock { id: 2, position: 11 },
            FileBlock { id: 3, position: 15 },
            FileBlock { id: 3, position: 16 },
            FileBlock { id: 3, position: 17 },
            FileBlock { id: 4, position: 19 },
            FileBlock { id: 4, position: 20 },
            FileBlock { id: 5, position: 22 },
            FileBlock { id: 5, position: 23 },
            FileBlock { id: 5, position: 24 },
            FileBlock { id: 5, position: 25 },
            FileBlock { id: 6, position: 27 },
            FileBlock { id: 6, position: 28 },
            FileBlock { id: 6, position: 29 },
            FileBlock { id: 6, position: 30 },
            FileBlock { id: 7, position: 32 },
            FileBlock { id: 7, position: 33 },
            FileBlock { id: 7, position: 34 },
            FileBlock { id: 8, position: 36 },
            FileBlock { id: 8, position: 37 },
            FileBlock { id: 8, position: 38 },
            FileBlock { id: 8, position: 39 },
            FileBlock { id: 9, position: 40 },
            FileBlock { id: 9, position: 41 },
        ];
        let compacted_blocks = compact_blocks(&blocks);
        let checksum = compute_checksum(compacted_blocks);
        assert_eq!(checksum, 1928);
    }

    #[test]
    fn test_sample_input_part_1() {
        let blocks = parse_input("./src/bin/day9/sample_input.txt");
        let compacted_blocks = compact_blocks(&blocks);
        let checksum = compute_checksum(compacted_blocks);
        assert_eq!(checksum, 1928);
    }

    #[test]
    fn test_sample_input_part_2() {
        let blocks = parse_input("./src/bin/day9/sample_input.txt");
        let compacted_blocks = compact_files(&blocks);
        let checksum = compute_checksum(compacted_blocks);
        assert_eq!(checksum, 2858);
    }
}
