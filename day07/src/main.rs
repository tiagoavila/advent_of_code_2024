use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

fn main() {
    println!("Advent of Code 2024 - day07");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> i64 {
    let lines = read_file(file_path).unwrap();
    lines
        .iter()
        .filter_map(|line| line.split_once(':'))
        .map(|(total, operators)| {
            (
                total.parse::<i64>().unwrap(),
                operators
                    .trim()
                    .split_whitespace()
                    .map(|operator| operator.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            )
        })
        .filter_map(|(total, operators)| {
            let mut operators_iter = operators.iter();
            let mut temp_results: Vec<i64> = Vec::new();
            temp_results.push(*operators_iter.next().unwrap());

            for operator in operators_iter {
                let mut new_temp_results = Vec::new();
                for temp_result in temp_results {
                    if temp_result + operator <= total {
                        new_temp_results.push(temp_result + operator);
                    }
                    if temp_result * operator <= total {
                        new_temp_results.push(temp_result * operator);
                    }
                }
                temp_results = new_temp_results;
            }

            temp_results.iter().any(|&x| x == total).then_some(total)
        })
       .sum()
}

fn part2(file_path: &str) -> i64 {
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
        assert_eq!(part1("test.txt"), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }
}
