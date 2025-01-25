use std::{
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
            let (x_values, y_values): (Vec<i64>, Vec<i64>) = chunk
                .into_iter()
                .map(|line| {
                    if let Some(captures) = re.captures(line) {
                        if let (Some(x), Some(y)) = (captures.get(1), captures.get(2)) {
                            // Extract and parse the captured groups
                            let x_value: i64 = x.as_str().parse().unwrap();
                            let y_value: i64 = y.as_str().parse().unwrap();

                            return (x_value, y_value);
                        }
                    }

                    return (0, 0);
                })
                .collect::<Vec<(i64, i64)>>()
                .into_iter()
                .unzip();

            // let mut matrix = vec![x_values, y_values];
            let eq1 = (x_values[0].clone(), x_values[1].clone(), x_values[2].clone());
            let eq2 = (y_values[0].clone(), y_values[1].clone(), y_values[2].clone());
            crate::matrix_operations::solve_by_substitution(eq1, eq2)
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

fn part2(file_path: &str) -> usize {
    let mut lines = read_file(file_path).unwrap();
    lines.retain(|line| !line.is_empty());

    let re = Regex::new(r"^.+X[+=](\d+).+Y[+=](\d+)").unwrap();

    let chunks: Vec<Option<(usize, usize)>> = lines
        .chunks(3) // Borrow chunks of 3 elements
        .map(|chunk| {
            let (x_values, y_values): (Vec<i64>, Vec<i64>) = chunk
                .into_iter()
                .map(|line| {
                    if let Some(captures) = re.captures(line) {
                        if let (Some(x), Some(y)) = (captures.get(1), captures.get(2)) {
                            // Extract and parse the captured groups
                            let x_value: i64 = x.as_str().parse().unwrap();
                            let y_value: i64 = y.as_str().parse().unwrap();

                            return (x_value, y_value);
                        }
                    }

                    return (0, 0);
                })
                .collect::<Vec<(i64, i64)>>()
                .into_iter()
                .unzip();

            // let mut matrix = vec![x_values, y_values];
            let mut matrix = vec![
                vec![x_values[0] as f64, x_values[1] as f64, x_values[2] as f64 + 10000000000000.0],
                vec![y_values[0] as f64, y_values[1] as f64, y_values[2] as f64 + 10000000000000.0],
            ];
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
    fn test_part1_challenge() {
        assert_eq!(part1("challenge.txt"), 37128);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("challenge.txt"), 74914228471331);
    }
}
