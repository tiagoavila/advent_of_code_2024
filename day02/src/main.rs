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
        .filter(|line_without_current| report_is_valid(line_without_current))
        .count() as i32
}

fn part2(input_path: &str) -> i32 {
    read_file(input_path)
        .unwrap()
        .iter()
        .filter(|line_without_current| report_is_valid_using_problem_dampener(line_without_current))
        .count() as i32
}

fn report_is_valid(line_without_current: &str) -> bool {
    let parts: Vec<i32> = line_without_current
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

fn report_is_valid_using_problem_dampener(line_without_current: &str) -> bool {
    let parts: Vec<i32> = line_without_current
        .split(" ")
        .map(|element| element.parse::<i32>().unwrap())
        .collect();

    let first_number = parts[0];
    let second_number = parts[1];

    if first_number == second_number {
        let slice = parse_parts_to_string(&parts[1..].to_vec());

        report_is_valid(&slice);
    }

    if first_number < second_number {
        validate_increasing_using_problem_dampener(parts)
    } else {
        validate_decreasing_using_problem_dampener(parts)
    }
}

fn validate_increasing_using_problem_dampener(mut parts: Vec<i32>) -> bool {
    for i in 1..=parts.len() - 2 {
        let previous_number = parts[i - 1];
        let number = parts[i];
        let next_number = parts[i + 1];

        if (number <= previous_number || number - previous_number > 3)
            || (next_number <= number || next_number - number > 3)
        {
            let mut parts_clone = parts.clone();
            let mut parts_clone2 = parts.clone();
            parts.remove(i);
            parts_clone.remove(i + 1);
            parts_clone2.remove(i - 1);
            let line_without_current = parse_parts_to_string(&parts);
            let line_without_next = parse_parts_to_string(&parts_clone);
            let line_without_previous = parse_parts_to_string(&parts_clone2);

            let result1 = report_is_valid(&line_without_current);
            let result2 = report_is_valid(&line_without_next);
            let result3 = report_is_valid(&line_without_previous);

            return result1 || result2 || result3;
        }
    }

    true
}

fn validate_decreasing_using_problem_dampener(mut parts: Vec<i32>) -> bool {
    for i in 1..=parts.len() - 2 {
        let previous_number = parts[i - 1];
        let number = parts[i];
        let next_number = parts[i + 1];

        if (number >= previous_number || previous_number - number > 3)
            || (next_number >= number || next_number - number > 3)
        {
            let mut parts_clone = parts.clone();
            let mut parts_clone2 = parts.clone();
            parts.remove(i);
            parts_clone.remove(i + 1);
            parts_clone2.remove(i - 1);
            let line_without_current = parse_parts_to_string(&parts);
            let line_without_next = parse_parts_to_string(&parts_clone);
            let line_without_previous = parse_parts_to_string(&parts_clone2);

            let result1 = report_is_valid(&line_without_current);
            let result2 = report_is_valid(&line_without_next);
            let result3 = report_is_valid(&line_without_previous);

            return result1 || result2 || result3;
        }
    }

    true
}

fn parse_parts_to_string(parts: &Vec<i32>) -> String {
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

        for (line_without_current, expected, reason) in cases {
            assert_eq!(
                report_is_valid(line_without_current),
                expected,
                "Failed on input '{}': {}",
                line_without_current,
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
        let line_without_current = "1 3 2 4 5";
        assert_eq!(
            report_is_valid_using_problem_dampener(line_without_current),
            true
        );
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
            ("9 7 6 2 1", false, "Unsafe because 6 2 is a decrease of 4."),
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

        for (line_without_current, expected, reason) in cases {
            assert_eq!(
                report_is_valid_using_problem_dampener(line_without_current),
                expected,
                "Failed on input '{}': {}",
                line_without_current,
                reason
            );
        }
    }

    #[test]
    fn test_line_part2() {
        let line = "21 24 21 19 17 14";
        assert_eq!(
            report_is_valid_using_problem_dampener(line),
            true
        );

        let line = "30 32 29 27 25 20";
        assert_eq!(
            report_is_valid_using_problem_dampener(line),
            false
        );

        let line = "80 80 78 75 74 72 69 71";
        assert_eq!(
            report_is_valid_using_problem_dampener(line),
            false
        );

        let line = "45 45 44 42 40 38 41 41";
        assert_eq!(
            report_is_valid_using_problem_dampener(line),
            false
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test_input.txt"), 4);
    }
}
