use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;

pub fn part1(input_path: &str) -> i32 {
    let lines: Vec<String> = read_file(input_path).unwrap();
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = lines
        .iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let first = parts.next().unwrap().parse::<i32>().unwrap();
            let second = parts.next().unwrap().parse::<i32>().unwrap();
            (first, second)
        })
        .unzip();

    left.sort();
    right.sort();

    zip(left.iter(), right.iter())
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>()
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
        // Arrange: The test input path is already set up with the test file
        let test_input_path = "inputs/day1/test_input.txt";

        // Act: Call the function
        let result = part1(test_input_path);

        // Assert: Verify the result
        let expected_result = 11;
        assert_eq!(result, expected_result, "Expected {}, but got {}", expected_result, result);
    }
}
