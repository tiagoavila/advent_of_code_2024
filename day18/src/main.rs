use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};
use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

fn main() {
    println!("Advent of Code 2024 - day18");
    println!("Part 1: {}", part1("challenge.txt", 70, Some(1024)));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str, maximum_index: usize, take: Option<usize>) -> usize {
    let length = maximum_index + 1;
    let mut grid = vec![vec!['.'; length]; length];

    let lines = read_file(file_path).unwrap();
    let take: usize = take.unwrap_or(lines.len());
    lines.into_iter().take(take).for_each(|line| {
        let (x, y) = line
            .split_once(',')
            .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
            .unwrap();
        grid[y][x] = '#';
    });

    let end: Pos = Pos(maximum_index, maximum_index);
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    dijkstra(&Pos(0, 0), |item: &Pos| {
        let mut sucessors: Vec<(Pos, usize)> = Vec::new();
        directions.iter().for_each(|(dx, dy)| {
            let x = item.0 as i32 + dx;
            let y = item.1 as i32 + dy;
            if x >= 0 && x < length as i32 && y >= 0 && y < length as i32 {
                if grid[y as usize][x as usize] == '.' {
                    let pos = Pos(x as usize, y as usize);
                    sucessors.push((pos, 1));
                }
            }
        });
        return sucessors;
    } , |goal| *goal == end)
    .unwrap_or_else(|| (Vec::new(), 0))
    .1
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
        assert_eq!(part1("test.txt", 6, Option::Some(12)), 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }
}
