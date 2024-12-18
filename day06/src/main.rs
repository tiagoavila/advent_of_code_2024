use std::{
    collections::{HashMap, HashSet},
    fs::File,
    hash::Hash,
    io::{self, BufRead},
    ptr::read,
};

fn main() {
    println!("Advent of Code 2024 - day06");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> i32 {
    let lines = read_file(file_path).unwrap();
    let row_count = lines.len();
    let col_count = lines[0].len();
    let (obstructions, mut guard_position) = get_obstructions_and_guard_position(lines);

    let visited = traverse_until_left_area(
        row_count as i32,
        col_count as i32,
        &obstructions,
        &mut guard_position,
    );

    visited.len() as i32
}

fn traverse_until_left_area(
    row_count: i32,
    col_count: i32,
    obstructions: &Vec<(i32, i32)>,
    guard_position: &mut (i32, i32),
) -> HashSet<(i32, i32)> {
    let mut directions = HashMap::new();
    directions.insert('^', (-1, 0));
    directions.insert('>', (0, 1));
    directions.insert('v', (1, 0));
    directions.insert('<', (0, -1));

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(*guard_position);
    let mut guard_left_area: bool = false;
    let mut guard_directions = ['^', '>', 'v', '<'].iter().cycle();
    while !guard_left_area {
        let guard_direction = guard_directions.next().unwrap();
        let (row_dir, col_dir) = directions.get(&guard_direction).unwrap();
        let mut row = guard_position.0 as i32;
        let mut col = guard_position.1 as i32;

        loop {
            row += row_dir;
            col += col_dir;

            if obstructions.contains(&(row as i32, col as i32)) {
                match guard_direction {
                    '^' => {
                        guard_position.0 = row as i32 + 1;
                    }
                    'v' => {
                        guard_position.0 = row as i32 - 1;
                    }
                    '<' => {
                        guard_position.1 = col as i32 + 1;
                    }
                    '>' => {
                        guard_position.1 = col as i32 - 1;
                    }
                    _ => {}
                }
                break;
            }

            if row < 0 && *guard_direction == '^'
                || row >= row_count as i32 && *guard_direction == 'v'
                || col < 0 && *guard_direction == '<'
                || col >= col_count as i32 && *guard_direction == '>'
            {
                guard_left_area = true;
                break;
            }

            visited.insert((row as i32, col as i32));
        }
    }
    visited
}

fn get_obstructions_and_guard_position(lines: Vec<String>) -> (Vec<(i32, i32)>, (i32, i32)) {
    let mut obstructions: Vec<(i32, i32)> = Vec::new();
    let mut guard_position: (i32, i32) = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstructions.push((i as i32, j as i32));
                }
                '^' => {
                    guard_position = (i as i32, j as i32);
                }
                _ => {}
            }
        }
    }

    return (obstructions, guard_position);
}

fn part2(file_path: &str) -> u32 {
    let lines = read_file(file_path).unwrap();
    let row_count = lines.len();
    let col_count = lines[0].len();
    let (obstructions, mut guard_position) = get_obstructions_and_guard_position(lines);
    let initial_guard_position = guard_position;
    let visited = traverse_until_left_area(
        row_count as i32,
        col_count as i32,
        &obstructions,
        &mut guard_position,
    );

    let mut directions: HashMap<char, (i32, i32)> = HashMap::new();
    directions.insert('^', (-1, 0));
    directions.insert('>', (0, 1));
    directions.insert('v', (1, 0));
    directions.insert('<', (0, -1));

    let row_count = row_count as i32;
    let col_count = col_count as i32;
    let mut loops_count: u32 = 0;

    for (visited_row, visited_col) in visited {
        if visited_row == initial_guard_position.0 && visited_col == initial_guard_position.1 {
            continue;
        }

        let mut guard_position = initial_guard_position.clone();
        let mut guard_directions = ['^', '>', 'v', '<'].iter().cycle();
        let mut guard_direction = guard_directions.next().unwrap();
        let mut loop_detector: HashSet<(char, i32, i32)> = HashSet::new();

        while guard_position.0 >= 0
            && guard_position.0 < row_count
            && guard_position.1 >= 0
            && guard_position.1 < col_count
        {
            let (row_dir, col_dir) = directions.get(&guard_direction).unwrap();
            let mut row = guard_position.0 as i32;
            let mut col = guard_position.1 as i32;

            row += row_dir;
            col += col_dir;

            if obstructions.contains(&(row as i32, col as i32))
                || (row, col) == (visited_row, visited_col)
            {
                match guard_direction {
                    '^' => {
                        guard_position.0 = row as i32 + 1;
                    }
                    'v' => {
                        guard_position.0 = row as i32 - 1;
                    }
                    '<' => {
                        guard_position.1 = col as i32 + 1;
                    }
                    '>' => {
                        guard_position.1 = col as i32 - 1;
                    }
                    _ => {}
                }
                guard_direction = guard_directions.next().unwrap();
            } else {
                guard_position = (row, col);

                if !loop_detector.insert((*guard_direction, row, col)) {
                    loops_count += 1;
                    break;
                }
            }
        }
    }

    loops_count
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
        assert_eq!(part2("test.txt"), 6);
    }

    #[test]
    fn test_part2_challenge_input() {
        assert_eq!(part2("challenge.txt"), 2165);
    }
}
