use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

use glam::IVec2;
use guid_create::GUID;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

fn main() {
    println!("Advent of Code 2024 - day12");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> usize {
    let lines = read_file(file_path).unwrap();
    let (map, (rows, cols)) = parse_input_to_map(&lines);
    find_area_and_perimeter(&map, &rows, &cols)
        .iter()
        .map(|(_key, (area, perimeter))| area * perimeter)
        .sum()
}

fn find_area_and_perimeter(
    map: &HashMap<IVec2, char>,
    rows: &usize,
    cols: &usize,
) -> HashMap<GUID, (usize, usize)> {
    let mut areas_and_perimeters: HashMap<GUID, (usize, usize)> = HashMap::new();
    let mut visited: HashSet<IVec2> = HashSet::new();
    let mut area_identifier = ' ';
    let mut area_size = 0;
    let mut perimeter = 0;
    let mut queue: Vec<IVec2> = Vec::new();

    for row in 0..*rows {
        for col in 0..*cols {
            let current_position = IVec2::new(row as i32, col as i32);

            if visited.contains(&current_position) {
                continue;
            }

            if area_identifier == ' ' {
                area_identifier = map.get(&current_position).unwrap().clone();
                queue.push(current_position);
            }

            while !queue.is_empty() {
                let current_position = queue.pop().unwrap();
                let mut current_perimeter = 4;
                if visited.contains(&current_position) {
                    continue;
                }

                visited.insert(current_position);
                area_size += 1;

                DIRECTIONS.iter().for_each(|dir| {
                    let neighbor = current_position + *dir;
                    // Add the neighbor to the queue if it is inside the bounds and it has not been visited
                    if is_inside_bounds(rows, cols, neighbor) && !visited.contains(&neighbor) {
                        let neighbor_value = map.get(&neighbor).unwrap();
                        if *neighbor_value == area_identifier {
                            queue.push(neighbor);
                        }
                    }

                    // If the neighbor is inside the bounds and it is part of the same area, then the perimeter is reduced by 1
                    if is_inside_bounds(rows, cols, neighbor) {
                        let neighbor_value = map.get(&neighbor).unwrap();
                        if *neighbor_value == area_identifier {
                            current_perimeter -= 1;
                        }
                    }
                });

                perimeter += current_perimeter;
            }

            let guid = GUID::rand();
            areas_and_perimeters.insert(guid, (area_size, perimeter));
            area_identifier = ' ';
            area_size = 0;
            perimeter = 0;
        }
    }

    areas_and_perimeters
}

fn is_inside_bounds(rows: &usize, cols: &usize, position: IVec2) -> bool {
    position.x >= 0 && position.x < *rows as i32 && position.y >= 0 && position.y < *cols as i32
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

fn parse_input_to_map(lines: &Vec<String>) -> (HashMap<IVec2, char>, (usize, usize)) {
    let mut map = HashMap::new();
    let rows = lines.len();
    let cols = lines[0].len();

    for (row_index, line) in lines.iter().enumerate() {
        for (col_index, c) in line.chars().enumerate() {
            map.insert(IVec2::new(row_index as i32, col_index as i32), c);
        }
    }

    (map, (rows, cols))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test.txt"), 1930);
    }

    #[test]
    fn test_example1() {
        assert_eq!(part1("example1.txt"), 140);
    }

    #[test]
    fn test_example2() {
        assert_eq!(part1("example2.txt"), 772);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }
}
