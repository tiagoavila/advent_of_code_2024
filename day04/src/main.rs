use std::{
    fs::File,
    io::{self, BufRead},
};

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),  // Up
    (1, 0),   // Down
    (0, -1),  // Left
    (0, 1),   // Right
    (-1, -1), // Top-left
    (-1, 1),  // Top-right
    (1, -1),  // Bottom-left
    (1, 1),   // Bottom-right
];

fn main() {
    println!("Advent of Code 2024 - Day 04");
    println!("Part 1: {}", part1("challenge.txt"));
    // println!("Part 2: {}", part2("challenge_input.txt"));
}

fn part1(file_path: &str) -> i32 {
    let matrix = parse_input_to_matrix(file_path);
    let mut count = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 'X' {
                count += find_XMAS(&matrix, &row, &col);
                println!("Element at [{}][{}] is {}", row, col, matrix[row][col]);
            }
        }
    }
    count
}

fn find_XMAS(matrix: &Vec<Vec<char>>, row: &usize, col: &usize) -> i32 {
    let mut count = 0;
    let search_count = 3;
    for (d_row, d_col) in DIRECTIONS.iter() {
        let mut current_row = *row as isize + d_row;
        let mut current_col = *col as isize + d_col;
        let mut next_letters = vec!['M', 'A', 'S'].into_iter();

        for i in 0..search_count {
            if current_row < 0 || current_col < 0 || current_row >= matrix.len() as isize || current_col >= matrix[0].len() as isize {
                break;
            }
            let next_letter = next_letters.next().unwrap();
            let current_letter = matrix[current_row as usize][current_col as usize];
            if current_letter != next_letter {
                break;
            } else if current_letter == 'S' && i == search_count - 1 {
                count += 1;
            }
            current_row += d_row;
            current_col += d_col;
        } 
    }
    count
}

fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Collect the lines into a vector
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    Ok(lines)
}

fn parse_input_to_matrix(file_path: &str) -> Vec<Vec<char>> {
    read_file(file_path)
        .unwrap()
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_to_matrix_using_example_txt() {
        let input = parse_input_to_matrix("./example.txt");
        println!("{:?}", input);
        assert_eq!(true, true);
    }

    #[test]
    fn test_part1_using_example_txt() {
        assert_eq!(part1("./example.txt"), 18);
    }
}
