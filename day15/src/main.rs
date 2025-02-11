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
    let (warehouse, movements) = get_warehouse_and_movements(file_path);
    let (mut robot, mut warehouse_map) = get_warehouse_map(warehouse);

    for movement in movements.chars() {
        let direction = match movement {
            '^' => matrix::Direction::TOP,
            'v' => matrix::Direction::BOTTOM,
            '<' => matrix::Direction::LEFT,
            '>' => matrix::Direction::RIGHT,
            _ => panic!("Invalid movement"),
        };
        robot.move_robot(&mut warehouse_map, direction);
    }

    warehouse_map
        .iter()
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
    let (warehouse, movements) = get_warehouse_and_movements(file_path);
    let doubled_warehouse = double_warehouse(warehouse);
    let warehouse_map = process_warehouse_movements(doubled_warehouse, movements);

    0
}

fn process_warehouse_movements(
    warehouse: Vec<String>,
    movements: String,
) -> HashMap<(usize, usize), char> {
    let (mut robot, mut warehouse_map) = get_warehouse_map(warehouse);

    for movement in movements.chars() {
        let direction = match movement {
            '^' => matrix::Direction::TOP,
            'v' => matrix::Direction::BOTTOM,
            '<' => matrix::Direction::LEFT,
            '>' => matrix::Direction::RIGHT,
            _ => panic!("Invalid movement"),
        };
        robot.move_robot_part2(&mut warehouse_map, direction);
    }

    warehouse_map
}

fn double_warehouse(warehouse: Vec<String>) -> Vec<String> {
    warehouse
        .iter()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    match ch {
                        '#' => "##",
                        'O' => "[]",
                        '.' => "..",
                        '@' => "@.",
                        _ => "", // Using _ is the conventional way to match any remaining patterns
                    }
                    .to_string()
                })
                .collect::<String>()
        })
        .collect()
}

fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Collect the lines into a vector
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    Ok(lines)
}

fn get_warehouse_map(warehouse: Vec<String>) -> (matrix::Robot, HashMap<(usize, usize), char>) {
    let mut robot = matrix::Robot::new(0, 0);
    let mut warehouse_map: HashMap<(usize, usize), char> = HashMap::new();
    for (row, line) in warehouse.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            warehouse_map.insert((row, col), ch);

            if ch == '@' {
                robot = matrix::Robot::new(row, col);
            }
        }
    }
    (robot, warehouse_map)
}

fn get_warehouse_and_movements(file_path: &str) -> (Vec<String>, String) {
    let lines = read_file(file_path).unwrap();
    let empty_line_index = lines.iter().position(|line| line.is_empty()).unwrap();
    let warehouse = lines[..empty_line_index].to_vec();
    let movements = lines[empty_line_index + 1..].to_vec().join("");
    (warehouse, movements)
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

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("test.txt"), 0);
    // }

    #[test]
    fn test_move_boxes_to_left_and_empty_down() {
        let (warehouse, _) = get_warehouse_and_movements("simple_example2.txt");
        let doubled_warehouse = double_warehouse(warehouse);
        let movements = "<v".to_string();
        let warehouse_map = process_warehouse_movements(doubled_warehouse, movements);
        assert_eq!(warehouse_map.get(&(3, 5)).unwrap(), &'[');
        assert_eq!(warehouse_map.get(&(3, 6)).unwrap(), &']');
        assert_eq!(warehouse_map.get(&(3, 7)).unwrap(), &'[');
        assert_eq!(warehouse_map.get(&(3, 8)).unwrap(), &']');
        assert_eq!(warehouse_map.get(&(3, 9)).unwrap(), &'.');
        assert_eq!(warehouse_map.get(&(4, 9)).unwrap(), &'@');
    }

    #[test]
    fn test_move_boxes_up() {
        let (warehouse, _) = get_warehouse_and_movements("simple_example2.txt");
        let doubled_warehouse = double_warehouse(warehouse);
        let movements = "<vv<<^".to_string();
        let warehouse_map = process_warehouse_movements(doubled_warehouse, movements);

        assert_eq!(warehouse_map.get(&(2, 5)).unwrap(), &'[');
        assert_eq!(warehouse_map.get(&(2, 6)).unwrap(), &']');
        assert_eq!(warehouse_map.get(&(2, 7)).unwrap(), &'[');
        assert_eq!(warehouse_map.get(&(2, 8)).unwrap(), &']');

        assert_eq!(warehouse_map.get(&(3, 5)).unwrap(), &'.');
        assert_eq!(warehouse_map.get(&(3, 6)).unwrap(), &'[');
        assert_eq!(warehouse_map.get(&(3, 7)).unwrap(), &']');
        assert_eq!(warehouse_map.get(&(3, 8)).unwrap(), &'.');

        assert_eq!(warehouse_map.get(&(3, 9)).unwrap(), &'.');
        assert_eq!(warehouse_map.get(&(4, 9)).unwrap(), &'.');
        assert_eq!(warehouse_map.get(&(4, 8)).unwrap(), &'.');
        assert_eq!(warehouse_map.get(&(4, 7)).unwrap(), &'@');
    }
}
