use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

enum BlockType {
    FreeSpace,
    File,
}

fn main() {
    println!("Advent of Code 2024 - day09");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> i128 {
    let input = read_file(file_path).unwrap();
    let input = display_blocks(input);
    let input = switch_characters(input);
    calculate_checksum(input)
}

fn part2(file_path: &str) -> i32 {
    0
}

fn display_blocks(file: String) -> String {
    let mut block_id: i32 = 0;
    let mut block_type = BlockType::File;
    let test = file
        .chars()
        .flat_map(|c| {
            let block_size = c.to_digit(10).unwrap() as usize;

            match block_type {
                BlockType::FreeSpace => {
                    block_type = BlockType::File;
                    return repeat_char(String::from("."), block_size);
                }
                BlockType::File => {
                    let file_representation =
                        repeat_char(block_id.to_string(), block_size);
                    block_id += 1;
                    block_type = BlockType::FreeSpace;
                    return file_representation;
                }
            }
        }) // Repeat each character twice
        .collect(); // Collect into a single `String`
    return test;
}

fn repeat_char(
    character_to_repeat: String,
    repeat_count: usize,
) -> std::iter::Take<std::iter::Repeat<String>> {
    let file_representation = std::iter::repeat(character_to_repeat).take(repeat_count);
    file_representation
}

fn switch_characters(input: String) -> String {
    let mut chars: Vec<char> = input.chars().collect(); // Convert string to mutable Vec<char>
    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
        if chars[left] != '.' {
            left += 1;
            continue;
        }	

        if chars[right] == '.' {
            right -= 1;
            continue;
        }

        // Swap the characters at `left` and `right`
        chars.swap(left, right);

        // Move the pointers inward
        left += 1;
        right -= 1;
    }

    chars.into_iter().collect() // Convert Vec<char> back to String
}

fn calculate_checksum(input: String) -> i128 {
    input.replace(".", "").chars().enumerate().fold(0, |acc, (i, c)| {
        acc + (i as i128) * c.to_digit(10).unwrap() as i128
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test.txt"), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }

    #[test]
    fn test_display_blocks() {
        let input = "2333133121414131402".to_string();
        let expected_output = "00...111...2...333.44.5555.6666.777.888899".to_string();
        let result = display_blocks(input);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_display_blocks_simple() {
        let input = "12345".to_string();
        let expected_output = "0..111....22222".to_string();
        let result = display_blocks(input);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_switch_characters() {
        let input = "0..111....22222".to_string();
        let expected_output = "022111222......".to_string();
        let result = switch_characters(input);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_switch_characters_complex() {
        let input = "00...111...2...333.44.5555.6666.777.888899".to_string();
        let expected_output = "0099811188827773336446555566..............".to_string();
        let result = switch_characters(input);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_calculate_checksum() {
        let input = "0099811188827773336446555566..............".to_string();
        let expected_output = 1928;
        let result = calculate_checksum(input);
        assert_eq!(result, expected_output);
    }
}
