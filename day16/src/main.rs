use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

use matrix_utils::{Cell, Direction};
use pathfinding::prelude::{astar_bag_collect, dijkstra};

mod matrix_utils;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct StepState {
    position: (usize, usize),
    direction: Direction,
}

fn main() {
    println!("Advent of Code 2024 - day16");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> u32 {
    let lines = read_file(file_path).unwrap();
    let (maze, start_position, end_position): (Vec<Vec<Cell>>, StepState, (usize, usize)) = parse_input_to_maze_matrix(lines);

    let mut directions: HashMap<Direction, (i32, i32)> = HashMap::new();
    directions.insert(Direction::Right, (0, 1));
    directions.insert(Direction::Left, (0, -1));
    directions.insert(Direction::Up, (-1, 0));
    directions.insert(Direction::Down, (1, 0));

    let result = dijkstra(
        &start_position,
        |pos| get_sucessors(pos, &directions, &maze),
        |pos| get_end_position(end_position, pos),
    );

    match result {
        Some((_, score)) => score,
        None => 0,
    }
}

fn part2(file_path: &str) -> u32 {
    let lines = read_file(file_path).unwrap();
    let (maze, start_position, end_position): (Vec<Vec<Cell>>, StepState, (usize, usize)) = parse_input_to_maze_matrix(lines);

    let mut directions: HashMap<Direction, (i32, i32)> = HashMap::new();
    directions.insert(Direction::Right, (0, 1));
    directions.insert(Direction::Left, (0, -1));
    directions.insert(Direction::Up, (-1, 0));
    directions.insert(Direction::Down, (1, 0));

    let Some((result, cost)) = astar_bag_collect(
        &start_position,
        |pos| get_sucessors(pos, &directions, &maze),
        |pos| {
            (pos.position.0.abs_diff(end_position.0) + pos.position.1.abs_diff(end_position.1)) as u32
        },
        |pos| get_end_position(end_position, pos),
    ) else {
        panic!("No path found");
    };

    let mut tiles = HashSet::new();
    for path in result {
        for pos in path {
            tiles.insert(pos.position);
        }
    }

    tiles.len() as u32
}

fn get_end_position(end_position: (usize, usize), pos: &StepState) -> bool {
    let (row, col) = pos.position;
    return row == end_position.0 && col == end_position.1;
}

fn get_sucessors(pos: &StepState, directions: &HashMap<Direction, (i32, i32)>, maze: &Vec<Vec<Cell>>) -> Vec<(StepState, u32)> {
    let mut neighbors: Vec<(StepState, u32)> = Vec::new();
    let neighbor_same_direction = (
        pos.position.0 as i32 + directions[&pos.direction].0,
        pos.position.1 as i32 + directions[&pos.direction].1,
    );
    if maze[neighbor_same_direction.0 as usize][neighbor_same_direction.1 as usize] == Cell::Tile {
        push_neighbor(&mut neighbors, neighbor_same_direction, &pos.direction, 1);
    }

    match pos.direction {
        Direction::Right | Direction::Left => {
            let neighbor_up = (
                pos.position.0 as i32 + directions[&Direction::Up].0,
                pos.position.1 as i32 + directions[&Direction::Up].1,
            );
            let neighbor_down = (
                pos.position.0 as i32 + directions[&Direction::Down].0,
                pos.position.1 as i32 + directions[&Direction::Down].1,
            );
            if maze[neighbor_up.0 as usize][neighbor_up.1 as usize] == Cell::Tile {
                push_neighbor(&mut neighbors, neighbor_up, &Direction::Up, 1001);
            }
            if maze[neighbor_down.0 as usize][neighbor_down.1 as usize] == Cell::Tile {
                push_neighbor(&mut neighbors, neighbor_down, &Direction::Down, 1001);
            }
        }
        Direction::Up | Direction::Down => {
            let neighbor_left = (
                pos.position.0 as i32 + directions[&Direction::Left].0,
                pos.position.1 as i32 + directions[&Direction::Left].1,
            );
            let neighbor_right = (
                pos.position.0 as i32 + directions[&Direction::Right].0,
                pos.position.1 as i32 + directions[&Direction::Right].1,
            );
            if maze[neighbor_left.0 as usize][neighbor_left.1 as usize] == Cell::Tile {
                push_neighbor(&mut neighbors, neighbor_left, &Direction::Left, 1001);
            }
            if maze[neighbor_right.0 as usize][neighbor_right.1 as usize] == Cell::Tile {
                push_neighbor(&mut neighbors, neighbor_right, &Direction::Right, 1001);
            }
        }
    }
    return neighbors;
}

fn push_neighbor(
    neighbors: &mut Vec<(StepState, u32)>,
    neighbor: (i32, i32),
    direction: &Direction,
    cost: u32,
) {
    neighbors.push((
        StepState {
            position: (neighbor.0 as usize, neighbor.1 as usize),
            direction: direction.clone(),
        },
        cost,
    ));
}

fn parse_input_to_maze_matrix(lines: Vec<String>) -> (Vec<Vec<Cell>>, StepState, (usize, usize)) {
    let mut start_position: StepState = StepState {
        position: (0, 0),
        direction: Direction::Right,
    };
    let mut end_position: (usize, usize) = (0, 0);
    let maze: Vec<Vec<Cell>> = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        start_position.position = (row, col);
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
    (maze, start_position, end_position)
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
        assert_eq!(part2("test.txt"), 45);
    }
}
