use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

// Directions: Top, Right, Bottom, Left
const DIRECTIONS: [(i32, i32); 4] = [
    (-1, 0), // Top
    (0, 1),  // Right
    (1, 0),  // Bottom
    (0, -1), // Left
];

fn main() {
    println!("Advent of Code 2024 - day10");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> i32 {
    let lines = read_file(file_path).unwrap();
    let length = lines.len();
    let (map, zero_positions) = parse_input_to_map(lines);

    let trailhead_score = zero_positions.iter().fold(0, |acc, zero_position| {
        acc + find_trailhead_score(zero_position, &map, length)
    });

    return trailhead_score;
}

fn part2(file_path: &str) -> i32 {
    let lines = read_file(file_path).unwrap();
    let length = lines.len();
    let (map, zero_positions) = parse_input_to_map(lines);

    let trailhead_score = zero_positions.iter().fold(0, |acc, zero_position| {
        acc + find_trailhead_rating(zero_position, &map, length)
    });

    return trailhead_score;
}

fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Collect the lines into a vector
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    Ok(lines)
}

fn parse_input_to_map(
    lines: Vec<String>,
) -> (HashMap<(usize, usize), usize>, HashSet<(usize, usize)>) {
    let mut map = HashMap::new();
    let mut zero_positions = HashSet::new();

    for (row_index, line) in lines.iter().enumerate() {
        for (col_index, c) in line.chars().enumerate() {
            let digit = c.to_digit(10).unwrap() as usize;
            map.insert((row_index, col_index), digit);

            if digit == 0 {
                zero_positions.insert((row_index, col_index));
            }
        }
    }
    (map, zero_positions)
}

fn find_trailhead_score(
    zero_position: &(usize, usize),
    map: &HashMap<(usize, usize), usize>,
    length: usize,
) -> i32 {
    let mut stack = vec![(*zero_position, 0)];
    let mut trailhead_score: HashSet<(usize, usize)> = HashSet::new();

    while !stack.is_empty() {
        let (current_position, value) = stack.pop().unwrap();

        if value == 9 {
            trailhead_score.insert(current_position);
            continue;
        }

        DIRECTIONS.iter().for_each(|(dx, dy)| {
            let neighbor_position = (
                current_position.0 as i32 + dx,
                current_position.1 as i32 + dy,
            );

            if neighbor_position.0 < 0
                || neighbor_position.1 < 0
                || neighbor_position.0 >= length as i32
                || neighbor_position.1 >= length as i32
            {
                return;
            }

            let neighbor_position = (neighbor_position.0 as usize, neighbor_position.1 as usize);
            let neighbor_value = map.get(&neighbor_position).unwrap();

            if *neighbor_value == value + 1 {
                stack.push((neighbor_position, *neighbor_value));
            }
        });
    }

    trailhead_score.len() as i32
}

fn find_trailhead_rating(
    zero_position: &(usize, usize),
    map: &HashMap<(usize, usize), usize>,
    length: usize,
) -> i32 {
    let mut stack = vec![(*zero_position, 0)];
    let mut traihead_rating: i32 = 0;

    while !stack.is_empty() {
        let (current_position, value) = stack.pop().unwrap();

        if value == 9 {
            traihead_rating += 1;
            continue;
        }

        DIRECTIONS.iter().for_each(|(dx, dy)| {
            let neighbor_position = (
                current_position.0 as i32 + dx,
                current_position.1 as i32 + dy,
            );

            if neighbor_position.0 < 0
                || neighbor_position.1 < 0
                || neighbor_position.0 >= length as i32
                || neighbor_position.1 >= length as i32
            {
                return;
            }

            let neighbor_position = (neighbor_position.0 as usize, neighbor_position.1 as usize);
            let neighbor_value = map.get(&neighbor_position).unwrap();

            if *neighbor_value == value + 1 {
                stack.push((neighbor_position, *neighbor_value));
            }
        });
    }

    traihead_rating
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_example() {
        assert_eq!(part1("example1.txt"), 1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("test.txt"), 36);
    }

    #[test]
    fn test_part1_example() {
        let lines = read_file("test.txt").unwrap();
        let length = lines.len();
        let (map, _zero_positions) = parse_input_to_map(lines);

        let trailhead_score = find_trailhead_score(&(6, 6), &map, length);
        assert_eq!(trailhead_score, 3);

        let trailhead_score = find_trailhead_score(&(0, 2), &map, length);
        assert_eq!(trailhead_score, 5);

        let trailhead_score = find_trailhead_score(&(0, 4), &map, length);
        assert_eq!(trailhead_score, 6);

        let trailhead_score = find_trailhead_score(&(2, 4), &map, length);
        assert_eq!(trailhead_score, 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 81);
    }

    #[test]
    fn test_part2_example() {
        let lines = read_file("test.txt").unwrap();
        let length = lines.len();
        let (map, _zero_positions) = parse_input_to_map(lines);

        let trailhead_score = find_trailhead_rating(&(6, 6), &map, length);
        assert_eq!(trailhead_score, 8);

        let trailhead_score = find_trailhead_rating(&(0, 2), &map, length);
        assert_eq!(trailhead_score, 20);

        let trailhead_score = find_trailhead_rating(&(0, 4), &map, length);
        assert_eq!(trailhead_score, 24);

        let trailhead_score = find_trailhead_rating(&(2, 4), &map, length);
        assert_eq!(trailhead_score, 10);
    }
}
