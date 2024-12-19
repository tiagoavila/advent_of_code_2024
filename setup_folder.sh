#!/bin/bash

# Check if sub-folder name is provided
if [ -z "$1" ]; then
    echo "Error: Sub-folder name is required."
    echo "Usage: ./setup_files.sh <sub-folder-name>"
    exit 1
fi

# Assign sub-folder name
SUB_FOLDER="$1"

# Ensure the sub-folder exists
if [ ! -d "$SUB_FOLDER" ]; then
    mkdir -p "$SUB_FOLDER"
    echo "Created sub-folder: $SUB_FOLDER"
fi

# Content for main.rs
MAIN_RS_CONTENT="use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

fn main() {
    println!(\"Advent of Code 2024 - $SUB_FOLDER\");
    println!(\"Part 1: {}\", part1(\"challenge.txt\"));
    println!(\"Part 2: {}\", part2(\"challenge.txt\"));
}

fn part1(file_path: &str) -> i32 {
    0
}

fn part2(file_path: &str) -> i32 {
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
        assert_eq!(part1(\"test.txt\"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(\"test.txt\"), 0);
    }
}
"

# Update or create main.rs in the sub-folder
echo "$MAIN_RS_CONTENT" > "$SUB_FOLDER/src/main.rs"
echo "Updated $SUB_FOLDER/main.rs with predefined content."

# Create example.txt in the sub-folder
if [ ! -f "$SUB_FOLDER/test.txt" ]; then
    touch "$SUB_FOLDER/test.txt"
    echo "Created $SUB_FOLDER/test.txt."
else
    echo "$SUB_FOLDER/test.txt already exists."
fi

# Create challenge.txt in the sub-folder
if [ ! -f "$SUB_FOLDER/challenge.txt" ]; then
    touch "$SUB_FOLDER/challenge.txt"
    echo "Created $SUB_FOLDER/challenge.txt."
else
    echo "$SUB_FOLDER/challenge.txt already exists."
fi

