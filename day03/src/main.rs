use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    println!("Advent of Code 2024 - Day 03");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(input_path: &str) -> i32 {
    let input = read_file(input_path).unwrap();
    let pattern = r"mul\(\d+,\d+\)"; // Regex pattern

    // Create regex object
    let re = Regex::new(pattern).expect("Invalid regex pattern");

    // Find all matches
    re.find_iter(&input)
        .map(|m| parse_and_multiply(m.as_str()))
        .sum()
}

fn parse_and_multiply(m: &str) -> i32 {
    let result = m.replace("mul(", "").replace(")", "");
    let (x, y) = result.split_once(",").unwrap();
    x.parse::<i32>().unwrap_or(0) * y.parse::<i32>().unwrap_or(0)
}

fn part2(input_path: &str) -> i32 {
    let input = read_file(input_path).unwrap();
    let pattern = r"(mul\(\d+,\d+\)|do\(\)|don't\(\))"; // Regex pattern

    // Create regex object
    let re = Regex::new(pattern).expect("Invalid regex pattern");
    let mut matches: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();

    let mut mul_enabled = true;
    let mut sum = 0;
    for item in matches.iter_mut() {
        if mul_enabled {
            if item.contains("mul") {
                sum += parse_and_multiply(item);
            } else if *item == "don't()" {
                mul_enabled = false;
            }
        }

        if *item == "do()" {
            mul_enabled = true;
        }
    }
     
    sum
}

fn read_file(file_path: &str) -> io::Result<String> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Collect the lines into a vector
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    Ok(lines[0].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("example.txt"), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("example2.txt"), 48);
    }
}
