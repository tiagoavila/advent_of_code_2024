use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    ptr::read,
};

fn main() {
    println!("Advent of Code 2024 - day06");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> usize {
    let lines = read_file(file_path).unwrap();
    let mut obstructions = Vec::new();
    let mut guard_position: (usize, usize) = (0, 0);
    let row_count = lines.len();
    let col_count = lines[0].len();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstructions.push((i, j));
                }
                '^' => {
                    guard_position = (i, j);
                }
                _ => {}
            }
        }
    }

    println!("{:?}", obstructions);
    println!("{:?}", guard_position);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut guard_left_area: bool = false;
    let mut guard_directions = ['^', '>', 'v', '<'].iter().cycle();
    while !guard_left_area {
        let guard_direction = guard_directions.next().unwrap();
        let obstruction = obstructions
            .iter()
            .find(|(row, col)| match guard_direction {
                '^' => {
                    return *row < guard_position.0 && *col == guard_position.1;
                }
                'v' => {
                    return *row > guard_position.0 && *col == guard_position.1;
                }
                '<' => {
                    return *row == guard_position.0 && *col < guard_position.1;
                }
                '>' => {
                    return *row == guard_position.0 && *col > guard_position.1;
                }
                _ => {
                    return false;
                }
            });

        match obstruction {
            Some((row, col)) => match guard_direction {
                '^' => {
                    insert_visited_in_up_or_down(
                        &(*row + 1),
                        &guard_position.0,
                        guard_position,
                        &mut visited,
                    );
                    guard_position.0 = *row + 1;
                }
                'v' => {
                    insert_visited_in_up_or_down(
                        &guard_position.0,
                        &(*row - 1),
                        guard_position,
                        &mut visited,
                    );
                    guard_position.0 = *row - 1;
                }
                '<' => {
                    insert_visited_in_left_or_right(
                        &(*col + 1),
                        &guard_position.1,
                        guard_position,
                        &mut visited,
                    );
                    guard_position.1 = *col + 1;
                }
                '>' => {
                    insert_visited_in_left_or_right(
                        &guard_position.1,
                        &(*col - 1),
                        guard_position,
                        &mut visited,
                    );
                    guard_position.1 = *col - 1;
                }
                _ => {}
            },
            None => {
                match guard_direction {
                    '^' => {
                        insert_visited_in_up_or_down(
                            &0,
                            &guard_position.0,
                            guard_position,
                            &mut visited,
                        );
                    }
                    'v' => {
                        insert_visited_in_up_or_down(
                            &guard_position.0,
                            &(row_count - 1),
                            guard_position,
                            &mut visited,
                        );
                    }
                    '<' => {
                        insert_visited_in_left_or_right(
                            &0,
                            &guard_position.1,
                            guard_position,
                            &mut visited,
                        );
                    }
                    '>' => {
                        insert_visited_in_left_or_right(
                            &guard_position.1,
                            &(col_count - 1),
                            guard_position,
                            &mut visited,
                        );
                    }
                    _ => {}
                }
                guard_left_area = true;
            }
        }
    }

    visited.len()
}

fn insert_visited_in_up_or_down(
    start: &usize,
    end: &usize,
    guard_position: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) {
    for guard_row in *start..=*end {
        visited.insert((guard_row, guard_position.1));
    }
}

fn insert_visited_in_left_or_right(
    start: &usize,
    end: &usize,
    guard_position: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) {
    for guard_col in *start..=*end {
        visited.insert((guard_position.0, guard_col));
    }
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
        assert_eq!(part1("test.txt"), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }
}
