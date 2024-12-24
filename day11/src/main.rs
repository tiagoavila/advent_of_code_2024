use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

const BLINKING_TIMES: usize = 25;

fn main() {
    println!("Advent of Code 2024 - day11");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> usize {
    let line = read_file(file_path).unwrap();
    let mut line_vec: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    println!("{}", line);

    for _ in 0..BLINKING_TIMES {
        line_vec = line_vec
            .iter()
            .flat_map(|rock| {
                if *rock == "0" {
                    vec![String::from("1")].into_iter()
                } else if rock.len() % 2 == 0 {
                    let rock_split = rock.split_at(rock.len() / 2);
                    vec![remove_leading_zeros(rock_split.0), remove_leading_zeros(rock_split.1)].into_iter() 
                } else {
                    let rock_value = rock.parse::<i64>().unwrap() * 2024;
                    vec![rock_value.to_string()].into_iter() 
                }
            })
            .collect();
    }
    line_vec.len()
}

fn part2(file_path: &str) -> i32 {
    0
}

fn read_file(file_path: &str) -> io::Result<String> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Collect the lines into a vector
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    Ok(lines[0].to_string())
}

fn remove_leading_zeros(rock: &str) -> String {
    let mut rock_updated = rock.trim_start_matches('0');
    if rock_updated.is_empty() {
        rock_updated = "0";
    }

    rock_updated.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test.txt"), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }
}
