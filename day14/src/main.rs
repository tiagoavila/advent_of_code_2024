use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Robot {
    row: usize,
    col: usize,
    row_velocity: isize,
    col_velocity: isize,
}

impl Robot {
    fn navigate(&mut self, rows: usize, cols: usize) {
        self.row = Robot::handle_movement(self.row, rows, self.row_velocity);
        self.col = Robot::handle_movement(self.col, cols, self.col_velocity);
    }

    fn handle_movement(position: usize, tiles_number: usize, velocity: isize) -> usize {
        let new_position = position as isize + velocity;

        if new_position < 0 {
            return tiles_number - new_position.abs() as usize;
        }

        if new_position as usize >= tiles_number {
            return new_position as usize - tiles_number;
        }

        return new_position as usize;
    }
}

fn main() {
    println!("Advent of Code 2024 - day14");
    println!("Part 1: {}", part1("challenge.txt", (103, 101)));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str, room_area: (usize, usize)) -> i32 {
    let lines = read_file(file_path).unwrap();
    let mut robots: Vec<Robot> = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let position_part = parts[0].to_string().replace("p=", "");
            let position: Vec<&str> = position_part.split(',').collect();

            let velocity_part = parts[1].to_string().replace("v=", "");
            let velocity: Vec<&str> = velocity_part.split(',').collect();

            let row = position[0].parse::<usize>().unwrap();
            let col = position[1].parse::<usize>().unwrap();
            let row_velocity = velocity[0].parse::<isize>().unwrap();
            let col_velocity = velocity[1].parse::<isize>().unwrap();

            Robot {
                row,
                col,
                row_velocity,
                col_velocity,
            }
        })
        .collect();

    for _ in 0..100 {
        robots
            .iter_mut()
            .for_each(|robot| robot.navigate(room_area.0, room_area.1));
    }

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
        assert_eq!(part1("test.txt", (7, 11)), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }

    #[test]
    fn test_movement_single_robot_by_five_seconds(){
        let mut robot = Robot {
            row: 4,
            col: 2,
            row_velocity: -3,
            col_velocity: 2,
        };

        let rows = 7;
        let cols = 11;

        for _ in 0..5 {
            robot.navigate(rows, cols);
        }

        assert_eq!(robot.row, 3);
        assert_eq!(robot.col, 1);
    }
}
