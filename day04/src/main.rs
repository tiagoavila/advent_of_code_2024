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

const DIAGONAL_DIRECTIONS: [(isize, isize); 4] = [
    (-1, -1), // Top-left
    (-1, 1),  // Top-right
    (1, -1),  // Bottom-left
    (1, 1),   // Bottom-right
];

fn main() {
    println!("Advent of Code 2024 - Day 04");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> i32 {
    let matrix = parse_input_to_matrix(file_path);
    let mut count = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 'X' {
                count += find_xmas(&matrix, &row, &col);
            }
        }
    }
    count
}

fn part2(file_path: &str) -> i32 {
    let matrix = parse_input_to_matrix(file_path);
    let mut count = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 'A'
                && row != 0
                && row != matrix.len() - 1
                && col != 0
                && col != matrix[row].len() - 1
            {
                count += find_mas_in_x_shape(&matrix, &row, &col);
            }
        }
    }
    count
}

fn find_xmas(matrix: &Vec<Vec<char>>, row: &usize, col: &usize) -> i32 {
    let mut count = 0;
    let search_count = 3;
    for (d_row, d_col) in DIRECTIONS.iter() {
        let mut current_row = *row as isize + d_row;
        let mut current_col = *col as isize + d_col;
        let mut next_letters = vec!['M', 'A', 'S'].into_iter();

        for i in 0..search_count {
            if current_row < 0
                || current_col < 0
                || current_row >= matrix.len() as isize
                || current_col >= matrix[0].len() as isize
            {
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

fn find_mas_in_x_shape(matrix: &Vec<Vec<char>>, row: &usize, col: &usize) -> i32 {
    let mut diagonal_directions_iter = DIAGONAL_DIRECTIONS.into_iter();
    let (top_left_row, top_left_col) = diagonal_directions_iter.next().unwrap();
    let (top_right_row, top_right_col) = diagonal_directions_iter.next().unwrap();
    let (bottom_left_row, bottom_left_col) = diagonal_directions_iter.next().unwrap();
    let (bottom_right_row, bottom_right_col) = diagonal_directions_iter.next().unwrap();

    let top_left_row = *row as isize + top_left_row;
    let top_left_col = *col as isize + top_left_col;
    let top_left_letter = matrix[top_left_row as usize][top_left_col as usize];
    if top_left_letter == 'M' || top_left_letter == 'S' {
        let bottom_right_row = *row as isize + bottom_right_row;
        let bottom_right_col = *col as isize + bottom_right_col;
        let expected_bottom_right_letter = if top_left_letter == 'M' { 'S' } else { 'M' };
        let bottom_right_letter = matrix[bottom_right_row as usize][bottom_right_col as usize];

        if bottom_right_letter == expected_bottom_right_letter {
            let top_right_row = *row as isize + top_right_row;
            let top_right_col = *col as isize + top_right_col;
            let top_right_letter = matrix[top_right_row as usize][top_right_col as usize];

            if top_right_letter == 'M' || top_right_letter == 'S' {
                let bottom_left_row = *row as isize + bottom_left_row;
                let bottom_left_col = *col as isize + bottom_left_col;
                let expected_bottom_left_letter = if top_right_letter == 'M' { 'S' } else { 'M' };

                if matrix[bottom_left_row as usize][bottom_left_col as usize]
                    == expected_bottom_left_letter
                {
                    return 1;
                }
            }
        }
    }

    return 0;
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
        assert_eq!(true, true);
    }

    #[test]
    fn test_part1_using_example_txt() {
        assert_eq!(part1("./example.txt"), 18);
    }

    #[test]
    fn test_part2_using_example_txt() {
        assert_eq!(part2("./example.txt"), 9);
    }
}
