use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

use regex::Regex;

pub mod matrix_operations;

fn main() {
    println!("Advent of Code 2024 - day13");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> usize {
    let mut lines = read_file(file_path).unwrap();
    lines.retain(|line| !line.is_empty());

    let re = Regex::new(r"^.+X[+=](\d+).+Y[+=](\d+)").unwrap();

    let chunks: Vec<Option<(usize, usize)>> = lines
        .chunks(3) // Borrow chunks of 3 elements
        .map(|chunk| {
            let (x_values, y_values): (Vec<f64>, Vec<f64>) = chunk
                .into_iter()
                .map(|line| {
                    if let Some(captures) = re.captures(line) {
                        if let (Some(x), Some(y)) = (captures.get(1), captures.get(2)) {
                            // Extract and parse the captured groups
                            let x_value: f64 = x.as_str().parse().unwrap();
                            let y_value: f64 = y.as_str().parse().unwrap();

                            return (x_value, y_value);
                        }
                    }

                    return (0.0, 0.0);
                })
                .collect::<Vec<(f64, f64)>>()
                .into_iter()
                .unzip();

            let mut matrix = vec![x_values, y_values];
            crate::matrix_operations::gaussian_elimination(&mut matrix)
        }) // Convert borrowed slices into owned Vec<String>
        .collect();

    chunks
        .into_iter()
        .filter(|line| line.is_some())
        .map(|line| {
            let (a, b) = line.unwrap();
            a * 3 + b
        })
        .sum::<usize>()
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
        assert_eq!(part1("test.txt"), 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }
}
