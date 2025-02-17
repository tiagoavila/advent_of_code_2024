use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path,
};

use matrix_utils::{Cell, MatrixUtils};
use petgraph::{
    algo::{self, dijkstra},
    graph::UnGraph,
};

mod matrix_utils;

fn main() {
    println!("Advent of Code 2024 - day16");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> i32 {
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

    let matrix_utils = MatrixUtils::new(lines.len(), lines.len());

    println!("maze {:?}", maze);
    println!("Start position {:?}", start_position);

    let g = UnGraph::<(), ()>::from_edges(&[(1, 2), (1, 3), (2, 3), (3, 4)]);
    let node_map = dijkstra(&g, 1.into(), Some(3.into()), |_| 1);
    let paths =
        algo::all_simple_paths::<Vec<_>, _>(&g, 1.into(), 3.into(), 0, None).collect::<Vec<_>>();
    // println!("{:?}", paths);
    0
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

    #[test]
    fn test_part1_example2() {
        assert_eq!(part1("test2.txt"), 11048);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }
}
