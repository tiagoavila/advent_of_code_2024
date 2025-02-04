use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

mod matrix;

fn main() {
    println!("Advent of Code 2024 - day15");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> i32 {
    let lines = read_file(file_path).unwrap();
    let empty_line_index = lines.iter().position(|line| line.is_empty()).unwrap();
    let warehouse_map = lines[..empty_line_index].to_vec();
    let movements = lines[empty_line_index + 1..].to_vec().join("");

    let mut robot = matrix::Robot::new(0, 0);
    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    for (row, line) in warehouse_map.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            map.insert((row, col), ch);

            if ch == '@' {
                robot = matrix::Robot::new(row, col);
            }
        }
    }

    for movement in movements.chars() {
        let direction = match movement {
            '^' => matrix::Direction::TOP,
            'v' => matrix::Direction::BOTTOM,
            '<' => matrix::Direction::LEFT,
            '>' => matrix::Direction::RIGHT,
            _ => panic!("Invalid movement"),
        };
        robot.move_robot(&mut map, direction);
    }

    map.iter()
        .filter_map(|position| {
            if position.1 == &'O' {
                Some((position.0 .0, position.0 .1))
            } else {
                None
            }
        })
        .fold(0, |acc, position| {
            acc + (100 * position.0 as i32 + position.1 as i32)
        })
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
        assert_eq!(part1("test.txt"), 10092);
    }

    #[test]
    fn test_simple_example() {
        assert_eq!(part1("simple_example.txt"), 2028);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }
}
