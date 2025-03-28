use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

use glam::IVec2;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];
const DIRECTIONS_MATRIX: [MatrixCell; 4] = [
    MatrixCell::TOP,
    MatrixCell::BOTTOM,
    MatrixCell::RIGHT,
    MatrixCell::LEFT,
];

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct MatrixCell {
    pub row: isize,
    pub col: isize,
}

use std::ops::Add;

impl Add for MatrixCell {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl MatrixCell {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    pub const TOP: Self = Self { row: -1, col: 0 };
    pub const BOTTOM: Self = Self { row: 1, col: 0 };
    pub const LEFT: Self = Self { row: 0, col: -1 };
    pub const RIGHT: Self = Self { row: 0, col: 1 };
}

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
        .map(|(area, perimeter)| area * perimeter)
        .sum()
}

fn part2(file_path: &str) -> i32 {
    let lines = read_file(file_path).unwrap();
    let mut map = HashMap::new();
    let rows = lines.len();
    let cols = lines[0].len();

    for (row_index, line) in lines.iter().enumerate() {
        for (col_index, c) in line.chars().enumerate() {
            map.insert(MatrixCell::new(row_index as isize, col_index as isize), c);
        }
    }

    let areas = get_areas(&map, &rows, &cols);
    let areas_sides: Vec<(usize, usize)> = areas
        .iter()
        .map(|area| {
            if area.len() == 1 {
                return (1, 4);
            }

            let mut sides_count = 0;

            for position in area.iter() {
                let top_left_neighbor = *position + MatrixCell::TOP + MatrixCell::LEFT;
                let top_neighbor = *position + MatrixCell::TOP;
                let top_right_neighbor = *position + MatrixCell::TOP + MatrixCell::RIGHT;
                let right_neighbor = *position + MatrixCell::RIGHT;
                let bottom_right_neighbor = *position + MatrixCell::BOTTOM + MatrixCell::RIGHT;
                let bottom_neighbor = *position + MatrixCell::BOTTOM;
                let bottom_left_neighbor = *position + MatrixCell::BOTTOM + MatrixCell::LEFT;
                let left_neighbor = *position + MatrixCell::LEFT;

                //check top left corner
                if !area.contains(&left_neighbor) && !area.contains(&top_left_neighbor) && !area.contains(&top_neighbor) {
                    sides_count += 1;
                }

                //check top right corner
                if !area.contains(&top_neighbor) && !area.contains(&top_right_neighbor) && !area.contains(&right_neighbor) {
                    sides_count += 1;
                }

                //check bottom right corner
                if !area.contains(&right_neighbor) && !area.contains(&bottom_right_neighbor) && !area.contains(&bottom_neighbor) {
                    sides_count += 1;
                }

                //check bottom left corner
                if !area.contains(&bottom_neighbor) && !area.contains(&bottom_left_neighbor) && !area.contains(&left_neighbor) {
                    sides_count += 1;
                }

                //check inner top left corner
                if (area.contains(&right_neighbor) && area.contains(&bottom_neighbor) && !area.contains(&bottom_right_neighbor)) 
                || (!area.contains(&top_neighbor) && !area.contains(&right_neighbor) && area.contains(&top_right_neighbor)) {
                    sides_count += 1;
                }

                //check inner top right corner
                if (area.contains(&left_neighbor) && area.contains(&bottom_neighbor) && !area.contains(&bottom_left_neighbor)) 
                    || (!area.contains(&top_neighbor) && !area.contains(&left_neighbor) && area.contains(&top_left_neighbor)) {
                    sides_count += 1;
                }

                //check inner bottom right corner
                if (area.contains(&left_neighbor) && area.contains(&top_neighbor) && !area.contains(&top_left_neighbor)) 
                || (!area.contains(&left_neighbor) && !area.contains(&bottom_neighbor) && area.contains(&bottom_left_neighbor)) {
                    sides_count += 1;
                }

                //check inner bottom left corner
                if (area.contains(&right_neighbor) && area.contains(&top_neighbor) && !area.contains(&top_right_neighbor)) 
                || (!area.contains(&right_neighbor) && !area.contains(&bottom_neighbor) && area.contains(&bottom_right_neighbor)){
                    sides_count += 1;
                }
            }

            (area.len(), sides_count)
        })
        .collect();

    areas_sides.iter().fold(0, |acc, (area_len, sides_count)| {
        acc + area_len * sides_count
    }) as i32
}

fn find_area_and_perimeter(
    map: &HashMap<IVec2, char>,
    rows: &usize,
    cols: &usize,
) -> Vec<(usize, usize)> {
    let mut areas_and_perimeters: Vec<(usize, usize)> = Vec::new();
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
                    if is_inside_bounds(rows, cols, &neighbor) && !visited.contains(&neighbor) {
                        let neighbor_value = map.get(&neighbor).unwrap();
                        if *neighbor_value == area_identifier {
                            queue.push(neighbor);
                        }
                    }

                    // Every time a neighbor of the same area is found decrease the parameter of the current area by 1
                    if is_inside_bounds(rows, cols, &neighbor) {
                        let neighbor_value = map.get(&neighbor).unwrap();
                        if *neighbor_value == area_identifier {
                            current_perimeter -= 1;
                        }
                    }
                });

                perimeter += current_perimeter;
            }

            areas_and_perimeters.push((area_size, perimeter));
            area_identifier = ' ';
            area_size = 0;
            perimeter = 0;
        }
    }

    areas_and_perimeters
}

fn get_areas(map: &HashMap<MatrixCell, char>, rows: &usize, cols: &usize) -> Vec<Vec<MatrixCell>> {
    let mut areas: Vec<Vec<MatrixCell>> = Vec::new();
    let mut visited: HashSet<MatrixCell> = HashSet::new();
    let mut area_identifier = ' ';
    let mut area: Vec<MatrixCell> = Vec::new();
    let mut queue: Vec<MatrixCell> = Vec::new();

    for row in 0..*rows {
        for col in 0..*cols {
            let current_position = MatrixCell::new(row as isize, col as isize);

            if visited.contains(&current_position) {
                continue;
            }

            if area_identifier == ' ' {
                area_identifier = map.get(&current_position).unwrap().clone();
                queue.push(*&current_position);
            }

            while !queue.is_empty() {
                let current_position = queue.pop().unwrap();

                if visited.contains(&current_position) {
                    continue;
                }

                visited.insert(current_position);
                area.push(current_position);

                DIRECTIONS_MATRIX.iter().for_each(|dir| {
                    let neighbor = current_position + *dir;
                    // Add the neighbor to the queue if it is inside the bounds and it has not been visited
                    if is_cell_inside_bounds(rows, cols, &neighbor) && !visited.contains(&neighbor)
                    {
                        let neighbor_value = map.get(&neighbor).unwrap();
                        if *neighbor_value == area_identifier {
                            queue.push(neighbor);
                        }
                    }
                });
            }

            areas.push(area);
            area_identifier = ' ';
            area = Vec::new();
        }
    }

    areas
}

fn is_inside_bounds(rows: &usize, cols: &usize, position: &IVec2) -> bool {
    position.x >= 0 && position.x < *rows as i32 && position.y >= 0 && position.y < *cols as i32
}

fn is_cell_inside_bounds(rows: &usize, cols: &usize, position: &MatrixCell) -> bool {
    position.row >= 0
        && position.row < *rows as isize
        && position.col >= 0
        && position.col < *cols as isize
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
    fn test_part2_simple_example() {
        assert_eq!(part2("simple_example_pt2.txt"), 32);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 1206);
    }

    #[test]
    fn test_example1_part2() {
        assert_eq!(part2("example1.txt"), 80);
    }

    #[test]
    fn test_example_e_shape_part2() {
        assert_eq!(part2("e_shape_example.txt"), 236);
    }

    #[test]
    fn test_example_2_part2() {
        assert_eq!(part2("example2.txt"), 436);
    }

    #[test]
    fn test_example_3_part2() {
        assert_eq!(part2("example3.txt"), 368);
    }

    #[test]
    fn test_example_with_one_inner_area_part2() {
        assert_eq!(part2("example_with_one_inner_area.txt"), 68);
    }
}
