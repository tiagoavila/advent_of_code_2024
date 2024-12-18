use itertools::Itertools;
use std::{
    char,
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

fn main() {
    println!("Advent of Code 2024 - day08");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> i32 {
    let lines = read_file(file_path).unwrap();
    let rows = lines.len() as i32;
    let cols = lines[0].len() as i32;
    let map = parse_input_to_map(lines);

    let combination_size = 2;
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    map.iter().for_each(|(_key, value)| {
        // Generate all combinations
        let combinations = value.iter().combinations(combination_size);

        for combination in combinations {
            let &[(row1, col1), (row2, col2)] = combination.as_slice() else {
                continue;
            };

            let (row_diff, col_diff) = get_diffs(row1, col1, row2, col2);

            if check_inside_bounds(*row1 - row_diff, *col1 - col_diff, rows, cols) {
                antinodes.insert((*row1 - row_diff, col1 - col_diff));
            }

            if check_inside_bounds(*row2 + row_diff, *col2 + col_diff, rows, cols) {
                antinodes.insert((*row2 + row_diff, col2 + col_diff));
            }
        }
    });

    antinodes.len() as i32
}

fn part2(file_path: &str) -> i32 {
    let lines = read_file(file_path).unwrap();
    let rows = lines.len() as i32;
    let cols = lines[0].len() as i32;
    let map = parse_input_to_map(lines);

    let combination_size = 2;
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    map.iter().for_each(|(_key, value)| {
        // Generate all combinations
        let combinations = value.iter().combinations(combination_size);

        for combination in combinations {
            let &[(row1, col1), (row2, col2)] = combination.as_slice() else {
                continue;
            };
            antinodes.insert((*row1, *col1));
            antinodes.insert((*row2, *col2));

            let (mut row_diff, mut col_diff) = get_diffs(row1, col1, row2, col2);
            let initial_row_diff = row_diff;
            let initial_col_diff = col_diff;

            while check_inside_bounds(*row1 - row_diff, *col1 - col_diff, rows, cols) {
                antinodes.insert((*row1 - row_diff, col1 - col_diff));
                row_diff = row_diff + initial_row_diff;
                col_diff = col_diff + initial_col_diff;
            } 

            row_diff = initial_row_diff;
            col_diff = initial_col_diff;

            while check_inside_bounds(*row2 + row_diff, *col2 + col_diff, rows, cols) {
                antinodes.insert((*row2 + row_diff, col2 + col_diff));
                row_diff = row_diff + initial_row_diff;
                col_diff = col_diff + initial_col_diff;
            } 
        }
    });

    antinodes.len() as i32
}

/// Get the differences between rows and cols to find the next antinode.
/// The differences are calculated based on the direction of the antennas.
/// If the second antenna is below and to the right of the first antenna, the differences are positive.
/// If the second antenna is below and to the left of the first antenna, the differences are negative.
fn get_diffs(row1: &i32, col1: &i32, row2: &i32, col2: &i32) -> (i32, i32) {
    let mut row_diff = *row1 as i32 - *row2 as i32;
    let mut col_diff = *col1 as i32 - *col2 as i32;
    if row2 > row1 && col2 > col1 {
        row_diff = row_diff.abs();
        col_diff = col_diff.abs();
    } else {
        row_diff = row_diff * -1;
        col_diff = col_diff * -1;
    }
    (row_diff, col_diff)
}

fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Collect the lines into a vector
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    Ok(lines)
}

fn parse_input_to_map(lines: Vec<String>) -> HashMap<String, Vec<(i32, i32)>> {
    let mut map = HashMap::new();
    for (row, line) in lines.iter().enumerate() {
        let parts: Vec<(usize, char)> = line.char_indices().collect();
        for (col, char) in parts {
            if char != '.' {
                map.entry(char.to_string())
                    .and_modify(|entry: &mut Vec<(i32, i32)>| entry.push((row as i32, col as i32)))
                    .or_insert_with(|| {
                        let mut new_entry = Vec::new();
                        new_entry.push((row as i32, col as i32));
                        new_entry
                    });
            }
        }
    }
    map
}

fn check_inside_bounds(row: i32, col: i32, rows: i32, cols: i32) -> bool {
    row >= 0 && col >= 0 && row < rows && col < cols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test.txt"), 14);
    }

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1("example1.txt"), 2);
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(part1("example2.txt"), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 34);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("example_part2.txt"), 9);
    }
}
