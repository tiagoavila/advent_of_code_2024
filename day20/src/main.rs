use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

use pathfinding::prelude::{astar_bag_collect, dijkstra};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

fn main() {
    println!("Advent of Code 2024 - day20");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> i32 {
    let lines = read_file(file_path).unwrap();
    let mut start: Pos = Pos(0, 0);
    let mut end: Pos = Pos(0, 0);
    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    
    for (row_idx, row) in lines.iter().enumerate() {
        for (col_idx, ch) in row.chars().enumerate() {
            if ch == 'S' {
                start = Pos(row_idx, col_idx);
                grid.insert((row_idx, col_idx), '.');	
            } else if ch == 'E' {
                end = Pos(row_idx, col_idx);
                grid.insert((row_idx, col_idx), '.');	
            } else {
                grid.insert((row_idx, col_idx), ch);
            }
        }
    };

    let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let path = dijkstra(
        &start,
        |item: &Pos| get_sucessors(&item, &grid, &directions),
        |goal| *goal == end,
    ).unwrap_or_else(|| (Vec::new(), 0));
    // println!("Start: {:?}, End: {:?}, Grid: {:?}", start, end, grid);	
    // println!("Path: {:?}", path);
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

fn get_sucessors(
    item: &Pos,
    grid: &HashMap<(usize, usize), char>,
    directions: &Vec<(i32, i32)>,
) -> Vec<(Pos, usize)> {
    let mut sucessors: Vec<(Pos, usize)> = Vec::new();
    directions.iter().for_each(|(dy, dx)| {
        let row = item.0 as i32 + dy;
        let col = item.1 as i32 + dx;
        if grid.get(&(row as usize, col as usize)).is_some_and(|&ch| ch == '.') {
            let pos = Pos(row as usize, col as usize);
            sucessors.push((pos, 1));
        }
    });
    return sucessors;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test.txt"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }
}
