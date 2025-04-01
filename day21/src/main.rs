use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};
mod keypads;

fn main() {
    println!("Advent of Code 2024 - day21");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> i32 {
    let mut lines = read_file(file_path).unwrap();

    let keypads = keypads::Keypad::new();
    println!("Keypad: {:?}", keypads);
    keypads.path_directional_to_numerical(&lines[0]);
    0
}

fn part2(file_path: &str) -> i32 {
    let mut lines = read_file(file_path).unwrap();
    0
}

fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Collect the lines into a vector
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test.txt"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }
}

