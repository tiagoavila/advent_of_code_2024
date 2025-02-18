use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path,
};

use matrix_utils::{Cell, MatrixUtils};
use petgraph::{
    algo::{self},
    graph::UnGraph,
};

mod matrix_utils;

fn main() {
    println!("Advent of Code 2024 - day16");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> u32 {
    let lines = read_file(file_path).unwrap();
    let mut start_position: (usize, usize) = (0, 0);
    let mut end_position: (usize, usize) = (0, 0);
    let maze: Vec<Vec<Cell>> = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        start_position = (row, col);
                        return Cell::Tile;
                    }

                    if c == 'E' {
                        end_position = (row, col);
                        return Cell::Tile;
                    }

                    if c == '.' {
                        return Cell::Tile;
                    }

                    Cell::Wall
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rows_len = maze.len();
    let cols_len = maze[0].len();
    let matrix_utils = MatrixUtils::new(rows_len, cols_len);
    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut edges: Vec<(u32, u32)> = Vec::new();

    for row in 0..rows_len {
        for col in 0..cols_len {
            if maze[row][col] == Cell::Wall {
                continue;
            }

            let index = matrix_utils.coords_to_index(row, col).unwrap() as u32;
            let cell_edges = directions
                .iter()
                .filter_map(|(dx, dy)| {
                    let new_row = row as i32 + dx;
                    let new_col = col as i32 + dy;
                    if new_row < 0 || new_col < 0 {
                        return None;
                    }

                    let new_row = new_row as usize;
                    let new_col = new_col as usize;
                    if new_row >= rows_len || new_col >= cols_len {
                        return None;
                    }

                    if maze[new_row][new_col] == Cell::Wall {
                        return None;
                    }

                    let new_index = matrix_utils.coords_to_index(new_row, new_col).unwrap() as u32;
                    if edges.contains(&(new_index, index)) {
                        return None;
                    }

                    Some((index, new_index))
                })
                .collect::<Vec<(u32, u32)>>();

            if !cell_edges.is_empty() {
                edges.extend(cell_edges);
            }
        }
    }

    let start = matrix_utils
        .coords_to_index(start_position.0, start_position.1)
        .unwrap() as u32;
    let end = matrix_utils
        .coords_to_index(end_position.0, end_position.1)
        .unwrap() as u32;
    let g = UnGraph::<(), ()>::from_edges(edges.iter().cloned());
    let paths = algo::all_simple_paths::<Vec<_>, _>(&g, start.into(), end.into(), 0, None)
        .collect::<Vec<_>>();
    paths.iter().for_each(|path| {
        let single_path = path.iter().map(|&x| matrix_utils.index_to_coords(x.index() as usize).unwrap()).collect::<Vec<_>>();
        println!("single_path: {:?}", single_path);
    });
    
    paths.iter().map(|path| path.len() as u32).min().unwrap()
}

fn part2(file_path: &str) -> i32 {
    let mut lines = read_file(file_path).unwrap();
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
        assert_eq!(part1("test.txt"), 7036);
    }

    // #[test]
    // fn test_part1_example2() {
    //     assert_eq!(part1("test2.txt"), 11048);
    // }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }
}
