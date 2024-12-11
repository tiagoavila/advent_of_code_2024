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
    let row_count = lines.len();
    let col_count = lines[0].len();
    let (obstructions, mut guard_position) = get_obstructions_and_guard_position(lines);

    let visited = traverse_until_left_area(row_count, col_count, &obstructions, &mut guard_position);

    visited.len()
}

fn traverse_until_left_area(row_count: usize, col_count: usize, obstructions: &Vec<(usize, usize)>, guard_position: &mut (usize, usize)) -> HashSet<(usize, usize)> {
    let mut directions = HashMap::new();
    directions.insert('^', (-1, 0));
    directions.insert('>', (0, 1));
    directions.insert('v', (1, 0));
    directions.insert('<', (0, -1));

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
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

            if obstructions.contains(&(row as usize, col as usize)) {
                match guard_direction {
                    '^' => {
                        guard_position.0 = row as usize + 1;
                    }
                    'v' => {
                        guard_position.0 = row as usize - 1;
                    }
                    '<' => {
                        guard_position.1 = col as usize + 1;
                    }
                    '>' => {
                        guard_position.1 = col as usize - 1;
                    }
                    _ => {}
                }
                // println!("{:?}", guard_position);
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

            visited.insert((row as usize, col as usize));
        }
    }
    visited
}

fn get_obstructions_and_guard_position(lines: Vec<String>) -> (Vec<(usize, usize)>, (usize, usize)) {
    let mut obstructions: Vec<(usize, usize)> = Vec::new();	
    let mut guard_position: (usize, usize) = (0, 0);

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
    
    return (obstructions, guard_position);
}

fn part2(file_path: &str) -> i32 {
    let lines = read_file(file_path).unwrap();
    let row_count = lines.len();
    let col_count = lines[0].len();
    let (obstructions, mut guard_position) = get_obstructions_and_guard_position(lines);
    let visited = traverse_until_left_area_part2(row_count, col_count, &obstructions, &mut guard_position);

    // println!("{:?}", visited);
    let mut next_direction = HashMap::new();
    next_direction.insert('^', ('>', 0, 1));
    next_direction.insert('>', ('v', 1, 0));
    next_direction.insert('v', ('<', 0, -1));
    next_direction.insert('<', ('^', -1, 0));

    let mut loops_count: HashSet<(char, usize, usize)> = HashSet::new();
    let visited_clone = visited.clone();

    for (direction, row, col) in visited {
        println!("{:?}", (direction, row, col));
        let (next_dir, row_dir, col_dir) = next_direction.get(&direction).unwrap();
        match next_dir {
            '^' => {
                if let Some((d, r, c)) = visited_clone.iter().find(|(d, r, c)| *d == *next_dir && *r < row && *c == col) {
                    loops_count.insert((*next_dir, *r, *c));
                }
            }
            'v' => {
                if let Some((d, r, c)) = visited_clone.iter().find(|(d, r, c)| *d == *next_dir && *r > row && *c == col) {
                    loops_count.insert((*next_dir, *r, *c));
                }
            }
            '<' => {
                if let Some((d, r, c)) = visited_clone.iter().find(|(d, r, c)| *d == *next_dir && *r == row && *c < col) {
                    loops_count.insert((*next_dir, *r, *c));
                }
            }
            '>' => {
                if let Some((d, r, c)) = visited_clone.iter().find(|(d, r, c)| *d == *next_dir && *r == row && *c < col) {
                    loops_count.insert((*next_dir, *r, *c));
                }
            }
            _ => {}
        }
    }

    loops_count.len() as i32
}

fn traverse_until_left_area_part2(row_count: usize, col_count: usize, obstructions: &Vec<(usize, usize)>, guard_position: &mut (usize, usize)) -> HashSet<(char, usize, usize)> {
    let mut directions = HashMap::new();
    directions.insert('^', (-1, 0));
    directions.insert('>', (0, 1));
    directions.insert('v', (1, 0));
    directions.insert('<', (0, -1));

    let mut visited: HashSet<(char, usize, usize)> = HashSet::new();
    let (guard_row, guard_col) = guard_position;
    visited.insert(('^', *guard_row, *guard_col));

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

            if obstructions.contains(&(row as usize, col as usize)) {
                match guard_direction {
                    '^' => {
                        guard_position.0 = row as usize + 1;
                    }
                    'v' => {
                        guard_position.0 = row as usize - 1;
                    }
                    '<' => {
                        guard_position.1 = col as usize + 1;
                    }
                    '>' => {
                        guard_position.1 = col as usize - 1;
                    }
                    _ => {}
                }
                // println!("{:?}", guard_position);
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

            visited.insert((*guard_direction, row as usize, col as usize));
        }
    }
    visited
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
}
