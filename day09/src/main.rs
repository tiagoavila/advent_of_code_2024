use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

enum BlockType {
    FreeSpace,
    File,
}

enum Block {
    FreeSpace(usize),
    File(usize, i32),
}

fn main() {
    println!("Advent of Code 2024 - day09");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> i128 {
    let input = read_file(file_path).unwrap();
    let input = display_blocks(input);
    let input = switch_blocks(input);
    calculate_checksum(input)
}

fn part2(file_path: &str) -> i128 {
    let input = read_file(file_path).unwrap();
    let blocks = parse_file_string_to_blocks_representation(input);
    let fragmented_blocks = switch_files(blocks);
    calculate_checksum_part2(fragmented_blocks)
}

fn display_blocks(file: String) -> Vec<String> {
    let mut block_id: i32 = 0;
    let mut block_type = BlockType::File;
    file.chars()
        .flat_map(|c| {
            let block_size = c.to_digit(10).unwrap() as usize;

            match block_type {
                BlockType::FreeSpace => {
                    block_type = BlockType::File;
                    return repeat_char(String::from("."), block_size);
                }
                BlockType::File => {
                    let file_representation = repeat_char(block_id.to_string(), block_size);
                    block_id += 1;
                    block_type = BlockType::FreeSpace;
                    return file_representation;
                }
            }
        }) // Repeat each character twice
        .collect()
}

fn repeat_char(
    character_to_repeat: String,
    repeat_count: usize,
) -> std::iter::Take<std::iter::Repeat<String>> {
    let file_representation = std::iter::repeat(character_to_repeat).take(repeat_count);
    file_representation
}

fn switch_blocks(mut input: Vec<String>) -> Vec<String> {
    let mut left = 0;
    let mut right = input.len() - 1;

    while left < right {
        if input[left] != "." {
            left += 1;
            continue;
        }

        if input[right] == "." {
            right -= 1;
            continue;
        }

        // Swap the characters at `left` and `right`
        input.swap(left, right);

        // Move the pointers inward
        left += 1;
        right -= 1;
    }

    input.into_iter().collect() // Convert Vec<char> back to String
}

fn calculate_checksum(input: Vec<String>) -> i128 {
    input
        .into_iter()
        .filter_map(|block| if block != "." { Some(block) } else { None })
        .enumerate()
        .fold(0, |acc, (i, c)| {
            acc + (i as i128) * c.parse::<i128>().unwrap()
        })
}

fn read_file(file_path: &str) -> io::Result<String> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Collect the input into a vector
    let input: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    Ok(input[0].to_string())
}

fn parse_file_string_to_blocks_representation(file: String) -> Vec<Block> {
    let mut block_id: i32 = 0;
    let mut block_type = BlockType::File;
    file.chars()
        .filter_map(|c| {
            let block_size = c.to_digit(10).unwrap() as usize;

            match block_type {
                BlockType::FreeSpace => {
                    block_type = BlockType::File;
                    if block_size == 0 {
                        return None;
                    }
                    return Some(Block::FreeSpace(block_size));
                }
                BlockType::File => {
                    let file_representation = Block::File(block_size, block_id);
                    block_id += 1;
                    block_type = BlockType::FreeSpace;
                    return Some(file_representation);
                }
            }
        }) // Repeat each character twice
        .collect()
}

fn switch_files(mut input: Vec<Block>) -> Vec<Block> {
    let mut right = input.len() - 1;
    let mut visited: HashSet<i32> = HashSet::new();
    while right > 0 {
        if let Block::File(file_size, id) = input[right] {
            if visited.contains(&id) {
                right -= 1;
                continue;
            }

            visited.insert(id);
            let mut swapped = false;
            for left in 0..right {
                if let Block::FreeSpace(free_space_size) = input[left] {
                    if free_space_size == file_size {
                        input.swap(right, left);
                        swapped = true;
                        break;
                    } else if free_space_size > file_size {
                        input[left] = Block::FreeSpace(free_space_size - file_size);
                        input[right] = Block::FreeSpace(file_size);
                        input.insert(left, Block::File(file_size, id));
                        swapped = true;
                        break;
                    }
                }
            }

            if !swapped {
                right -= 1;
            }
        } else {
            right -= 1;
        }
    }
    input
}

fn calculate_checksum_part2(input: Vec<Block>) -> i128 {
    let mut index_count = 0;

    input.into_iter().fold(0, |acc, c| match c {
        Block::File(file_size, id) => {
            let mut partial_acc = 0;
            for i in 0..file_size {
                partial_acc += (index_count as i128) * id as i128;
                index_count += 1;
            }
            return acc + partial_acc;
        }
        Block::FreeSpace(free_space) => {
            index_count += free_space;
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test.txt"), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 2858);
    }

    #[test]
    fn test_display_blocks() {
        let input = "2333133121414131402".to_string();
        let expected_output = "00...111...2...333.44.5555.6666.777.888899".to_string();
        let result = display_blocks(input).join("");
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_display_blocks_simple() {
        let input = "12345".to_string();
        let expected_output = "0..111....22222".to_string();
        let result = display_blocks(input).join("");
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_switch_blocks() {
        let input = "12345".to_string();
        let expected_output = "022111222......".to_string();
        let blocks = display_blocks(input);
        let result = switch_blocks(blocks).join("");
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_switch_blocks_complex() {
        let input = "2333133121414131402".to_string();
        let expected_output = "0099811188827773336446555566..............".to_string();
        let blocks = display_blocks(input);
        let result = switch_blocks(blocks).join("");
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_calculate_checksum() {
        let input = "2333133121414131402".to_string();
        let expected_output = 1928;
        let blocks = display_blocks(input);
        let result = switch_blocks(blocks);
        let checksum = calculate_checksum(result);
        assert_eq!(checksum, expected_output);
    }

    // #[test]
    // fn test_() {
    //     let input = "2333133121414131402".to_string();
    //     let expected_output = "00...111...2...333.44.5555.6666.777.888899".to_string();
    //     let result = parse_file_string_to_blocks_representation(input);
    //     let result = switch_files(result);
    //     let checksum = calculate_checksum_part2(result);
    //     // assert_eq!(result, expected_output);
    //     let test = true;
    //     assert_eq!(test, true);
    // }
}
