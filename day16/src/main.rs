use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path,
};

use matrix_utils::{Cell, Direction, MatrixUtils};
use petgraph::{
    algo::{self},
    graph::{NodeIndex, UnGraph},
};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

mod matrix_utils;

struct StepState {
    position: (usize, usize),
    direction: Direction
}

fn main() {
    println!("Advent of Code 2024 - day16");
    println!("Part 1: {}", part1("challenge.txt"));
    // println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> u32 {
    let lines = read_file(file_path).unwrap();
    let mut start_position: (usize, usize) = (0, 0);
    let mut end_position: (usize, usize) = (0, 0);
    let maze = parse_input_to_maze_matrix(lines, &mut start_position, end_position);

    let rows_len = maze.len();
    let cols_len = maze[0].len();
    let mut priority_queue = PriorityQueue::new();
    let mut known_distances: HashMap<(usize, usize), u32> = HashMap::new();
    known_distances.insert(start_position, 0);

    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for direction in directions.iter() {
        let new_row = (start_position.0 as i32 + direction.0) as usize;
        let new_col = (start_position.1 as i32 + direction.1) as usize;

        if maze[new_row][new_col] == Cell::Wall {
            continue;
        }
        priority_queue.push(start_position, Reverse(0));
    }

    0 as u32
}

fn parse_input_to_maze_matrix(lines: Vec<String>, start_position: &mut (usize, usize), mut end_position: (usize, usize)) -> Vec<Vec<Cell>> {
    let maze: Vec<Vec<Cell>> = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        *start_position = (row, col);
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
    maze
}

fn part1_old(file_path: &str) -> u32 {
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

    for row in 1..rows_len - 1 {
        for col in 1..cols_len - 1 {
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
    paths
        .iter()
        .map(|path: &Vec<NodeIndex>| calculate_path_score(path, &matrix_utils))
        .min()
        .unwrap()
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

fn calculate_path_score(path: &Vec<NodeIndex>, matrix_utils: &MatrixUtils) -> u32 {
    let mut score = 0;
    let mut direction = Direction::Right;
    let mut previous_position: (usize, usize) = matrix_utils
        .index_to_coords(path[0].index() as usize)
        .unwrap();
    for i in 1..path.len() {
        let current_position: (usize, usize) = matrix_utils
            .index_to_coords(path[i].index() as usize)
            .unwrap();
        let move_direction = get_move_direction(previous_position, current_position);
        score += calculate_move_score(&direction, &move_direction);

        direction = move_direction;
        previous_position = current_position;
    }

    score
}

fn get_move_direction(
    previous_position: (usize, usize),
    current_position: (usize, usize),
) -> Direction {
    if previous_position.0 == current_position.0 {
        if previous_position.1 < current_position.1 {
            return Direction::Right;
        }

        return Direction::Left;
    }

    if previous_position.0 < current_position.0 {
        return Direction::Down;
    }

    Direction::Up
}

fn calculate_move_score(current_direction: &Direction, move_direction: &Direction) -> u32 {
    if current_direction == move_direction {
        return 1;
    }

    match current_direction {
        Direction::Up | Direction::Down => {
            if *move_direction == Direction::Left || *move_direction == Direction::Right {
                return 1001;
            }

            if *current_direction == Direction::Up && *move_direction == Direction::Down {
                return 2001;
            }

            if *current_direction == Direction::Down && *move_direction == Direction::Up {
                return 2001;
            }

            return 0;
        }
        Direction::Left | Direction::Right => {
            if *move_direction == Direction::Up || *move_direction == Direction::Down {
                return 1001;
            }

            if *current_direction == Direction::Left && *move_direction == Direction::Right {
                return 2001;
            }

            if *current_direction == Direction::Right && *move_direction == Direction::Left {
                return 2001;
            }

            return 0;
        }
    }
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
    fn test_path_score() {
        let matrix_utils = MatrixUtils::new(15, 15);
        let path = vec![
            (13, 1),
            (12, 1),
            (11, 1),
            (10, 1),
            (9, 1),
            (9, 2),
            (9, 3),
            (8, 3),
            (7, 3),
            (7, 4),
            (7, 5),
            (7, 6),
            (7, 7),
            (7, 8),
            (7, 9),
            (7, 10),
            (7, 11),
            (8, 11),
            (9, 11),
            (10, 11),
            (11, 11),
            (12, 11),
            (13, 11),
            (13, 12),
            (13, 13),
            (12, 13),
            (11, 13),
            (10, 13),
            (9, 13),
            (8, 13),
            (7, 13),
            (6, 13),
            (5, 13),
            (4, 13),
            (3, 13),
            (2, 13),
            (1, 13),
        ];
        let path_as_node_indexes = path // Convert the path to node indexes
            .iter()
            .map(|(row, col)| {
                let index = matrix_utils.coords_to_index(*row, *col).unwrap();
                NodeIndex::new(index as usize)
            })
            .collect::<Vec<_>>();
        let result = calculate_path_score(&path_as_node_indexes, &matrix_utils);
        assert_eq!(result, 7036);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }
}
