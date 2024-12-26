use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    vec,
};

const BLINKING_TIMES_PART1: usize = 25;
const BLINKING_TIMES_PART2: usize = 75;

fn main() {
    println!("Advent of Code 2024 - day11");
    println!("Part 1: {}", part1("challenge.txt", BLINKING_TIMES_PART1));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str, blinking_times: usize) -> usize {
    let line = read_file(file_path).unwrap();
    let mut line_vec: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

    for _ in 0..blinking_times {
        line_vec = line_vec
            .iter()
            .flat_map(|rock| {
                if *rock == "0" {
                    let one_vector = vec![String::from("1")];
                    one_vector.into_iter()
                } else if rock.len() % 2 == 0 {
                    let rock_split = rock.split_at(rock.len() / 2);
                    let even_length = vec![
                        remove_leading_zeros(rock_split.0),
                        remove_leading_zeros(rock_split.1),
                    ];

                    even_length.into_iter()
                } else {
                    let rock_value = (rock.parse::<i64>().unwrap() * 2024).to_string();
                    vec![rock_value].into_iter()
                }
            })
            .collect();
    }
    line_vec.len()
}

fn part2(file_path: &str) -> u64 {
    let line = read_file(file_path).unwrap();
    let line_vec: Vec<u64> = line
        .split_whitespace()
        .map(|chunk| chunk.parse::<u64>().unwrap())
        .collect();
    let mut cache: HashMap<(u64, usize), u64> = HashMap::new();
    let mut sum = 0;

    for n in line_vec {
        sum += count(n, 75, &mut cache)
    }

    sum
}

// Got this from https://github.com/javorszky/adventofcode2024/blob/main/day11/src/part2.rs
fn count(number: u64, steps: usize, memo: &mut HashMap<(u64, usize), u64>) -> u64 {
    if memo.contains_key(&(number, steps)) {
        return memo[&(number, steps)];
    }

    // we have reached the end of this branch, there is only one number, and it's the one we're
    // looking at, except we don't care what the number is
    if steps == 0 {
        return 1;
    }

    let number_as_string = format!("{:?}", number);
    let res;

    if number == 0 {
        res = count(1, steps - 1, memo);
    } else if number_as_string.len() % 2 == 0 {
        let left = number_as_string[..number_as_string.len()/2].parse::<u64>().unwrap();
        let right = number_as_string[number_as_string.len()/2..].parse::<u64>().unwrap();

        res = count(left, steps - 1, memo) + count(right, steps - 1, memo);
    } else {
        res = count(number * 2024, steps - 1, memo);
    }

    memo.insert((number, steps), res);

    res
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
        assert_eq!(part1("test.txt", BLINKING_TIMES_PART1), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }
}
