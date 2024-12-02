use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    println!("Advent of Code 2024 - Day 02");
    println!("Part 1: {}", part1("challenge_input.txt"));
    println!("Part 2: {}", part2("challenge_input.txt"));
}

fn part1(input_path: &str) -> i32 {
    read_file(input_path)
        .unwrap()
        .iter()
        .filter(|line| report_is_valid(line))
        .count() as i32
}

fn part2(input_path: &str) -> i32 {
    read_file(input_path)
        .unwrap()
        .iter()
        .filter(|line| report_is_valid_using_problem_dampener(line))
        .count() as i32
}

fn report_is_valid(line: &str) -> bool {
    let parts: Vec<i32> = line
        .split(" ")
        .map(|element| element.parse::<i32>().unwrap())
        .collect();

    let first_number = parts[0];
    let second_number = parts[1];

    if first_number == second_number {
        return false;
    }

    if first_number < second_number {
        validate_increasing(parts)
    } else {
        validate_decreasing(parts)
    }
}

fn validate_increasing(parts: Vec<i32>) -> bool {
    let mut iter = parts.into_iter();
    if let Some(mut previous_number) = iter.next() {
        for number in iter {
            if number <= previous_number || number - previous_number > 3 {
                return false;
            }
            previous_number = number;
        }
    }
    true
}

fn validate_decreasing(parts: Vec<i32>) -> bool {
    let mut iter = parts.into_iter();
    if let Some(mut previous_number) = iter.next() {
        for number in iter {
            if number >= previous_number || previous_number - number > 3 {
                return false;
            }
            previous_number = number;
        }
    }
    true
}

fn report_is_valid_using_problem_dampener(line: &str) -> bool {
    let parts: Vec<i32> = line
        .split(" ")
        .map(|element| element.parse::<i32>().unwrap())
        .collect();

    let first_number = parts[0];
    let second_number = parts[1];

    if first_number == second_number {
        let slice = parse_parts_to_string(parts[1..].to_vec());

        return report_is_valid(&slice);
    }

    if first_number < second_number {
        validate_increasing_using_problem_dampener(parts)
    } else {
        validate_decreasing_using_problem_dampener(parts)
    }
}

fn validate_increasing_using_problem_dampener(mut parts: Vec<i32>) -> bool {
    let mut iter = parts.iter();
    if let Some(mut previous_number) = iter.next() {
        for (index, number) in iter.enumerate() {
            if number <= previous_number || number - previous_number > 3 {
                parts.remove(index + 1);
                let line = parse_parts_to_string(parts);
                return report_is_valid(&line);
            }
            previous_number = number;
        }
    }
    true
}

fn validate_decreasing_using_problem_dampener(mut parts: Vec<i32>) -> bool {
    let mut iter = parts.iter();
    if let Some(mut previous_number) = iter.next() {
        for (index, number) in iter.enumerate() {
            if number >= previous_number || previous_number - number > 3 {
                parts.remove(index + 1);
                let line = parse_parts_to_string(parts);
                return report_is_valid(&line);
            }
            previous_number = number;
        }
    }
    true
}

fn parse_parts_to_string(parts: Vec<i32>) -> String {
    parts
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join(" ")
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
    fn test_validate_report_cases() {
        let cases = vec![
            (
                "7 6 4 2 1",
                true,
                "Safe because the levels are all decreasing by 1 or 2.",
            ),
            (
                "1 2 7 8 9",
                false,
                "Unsafe because 2 7 is an increase of 5.",
            ),
            ("9 7 6 2 1", false, "Unsafe because 6 2 is a decrease of 4."),
            (
                "1 3 2 4 5",
                false,
                "Unsafe because 1 3 is increasing but 3 2 is decreasing.",
            ),
            (
                "8 6 4 4 1",
                false,
                "Unsafe because 4 4 is neither an increase or a decrease.",
            ),
            (
                "1 3 6 7 9",
                true,
                "Safe because the levels are all increasing by 1, 2, or 3.",
            ),
        ];

        for (line, expected, reason) in cases {
            assert_eq!(
                report_is_valid(line),
                expected,
                "Failed on input '{}': {}",
                line,
                reason
            );
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("test_input.txt"), 2);
    }

    #[test]
    fn test_report_is_valid_using_problem_dampener() {
        let line = "1 3 2 4 5";
        assert_eq!(report_is_valid_using_problem_dampener(line), true);
    }

    #[test]
    fn test_validate_report_using_problem_dampener_cases() {
        let cases = vec![
            (
                "7 6 4 2 1",
                true,
                "Safe because the levels are all decreasing by 1 or 2.",
            ),
            (
                "1 2 7 8 9",
                false,
                "Unsafe because 2 7 is an increase of 5.",
            ),
            (
                "9 7 6 2 1", 
                false, 
                "Unsafe because 6 2 is a decrease of 4."
            ),
            (
                "1 3 2 4 5",
                true,
                "Unsafe because 1 3 is increasing but 3 2 is decreasing.",
            ),
            (
                "8 6 4 4 1",
                true,
                "Unsafe because 4 4 is neither an increase or a decrease.",
            ),
            (
                "1 3 6 7 9",
                true,
                "Safe because the levels are all increasing by 1, 2, or 3.",
            ),
        ];

        for (line, expected, reason) in cases {
            assert_eq!(
                report_is_valid_using_problem_dampener(line),
                expected,
                "Failed on input '{}': {}",
                line,
                reason
            );
        }
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test_input.txt"), 4);
    }
}
