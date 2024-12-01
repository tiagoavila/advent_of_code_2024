use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;

pub fn part1(input_path: &str) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = parse_input(input_path);

    left.sort();
    right.sort();

    zip(left.iter(), right.iter())
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>()
}

pub fn part2(input_path: &str) -> i32 {
    let (left, right): (Vec<i32>, Vec<i32>) = parse_input(input_path);
    let mut right_ocurrences: HashMap<i32, i32> = HashMap::new();

    left.iter()
        .map(|&e| {
            // Dereference e immediately when using it
            if right_ocurrences.contains_key(&e) {
                right_ocurrences.get(&e).unwrap() * e // Dereference properly
            } else {
                let count: i32 = right.iter().filter(|&&x| x == e).count() as i32;
                right_ocurrences.insert(e, count); // Use e directly (it is an i32 now)
                count * e // No need for dereferencing
            }
        })
        .sum() // Sum up the results from the map
}

fn parse_input(input_path: &str) -> (Vec<i32>, Vec<i32>) {
    let lines: Vec<String> = read_file(input_path).unwrap();
    lines
        .iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let first = parts.next().unwrap().parse::<i32>().unwrap();
            let second = parts.next().unwrap().parse::<i32>().unwrap();
            (first, second)
        })
        .unzip()
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
        assert_eq!(
            result, expected_result,
            "Expected {}, but got {}",
            expected_result, result
        );
    }

    #[test]
    fn test_part2() {
        // Arrange: The test input path is already set up with the test file
        let test_input_path = "inputs/day1/test_input.txt";

        // Act: Call the part2 function
        let result = part2(test_input_path);

        // Assert: Verify the result
        let expected_result = 31;
        assert_eq!(
            result, expected_result,
            "Expected {}, but got {}",
            expected_result, result
        );
    }
}
