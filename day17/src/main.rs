use std::{
    collections::HashSet, fs::File, io::{self, BufRead}
};

use chronospatial_computer::Computer;
mod chronospatial_computer;

fn main() {
    println!("Advent of Code 2024 - day17");
    // println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> String {
    let mut lines = read_file(file_path).unwrap();
    let mut computer: Computer = Computer::new_from_input(&mut lines);
    computer.execute();
    computer.print_output()
}

fn part2(file_path: &str) -> i32 {
    let mut lines = read_file(file_path).unwrap();
    let mut computer: Computer = Computer::new_from_input(&mut lines);
    let register_a = computer.register_a;
    // for a in 35184372088831..281474976710655 {
    for a in 7..1000 {
        computer.register_a = a;
        computer.execute();
        if computer.output[0] == 2 {
            println!("a: {:b} - {:?} - output: {:?}", a, a, computer.output);
        }
        computer.reset(register_a);
    }

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
    use crate::chronospatial_computer::Computer;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test.txt"), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }

    /// If register C contains 9, the program 2,6 would set register B to 1.
    #[test]
    fn test_example_1() {
        let mut lines: Vec<String> = vec![
            "Register A: 0",
            "Register B: 0",
            "Register C: 9",
            "",
            "Program: 2,6",
        ]
        .iter()
        .map(|&line| line.to_string())
        .collect();
        let mut computer: Computer = Computer::new_from_input(&mut lines);
        computer.execute();
        assert_eq!(computer.register_b, 1);
    }

    /// If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    #[test]
    fn test_example_2() {
        let mut lines: Vec<String> = vec![
            "Register A: 10",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 5,0,5,1,5,4",
        ]
        .iter()
        .map(|&line| line.to_string())
        .collect();
        let mut computer: Computer = Computer::new_from_input(&mut lines);
        computer.execute();
        assert_eq!(computer.output, vec![0, 1, 2]);
    }

    ///If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    #[test]
    fn test_example_3() {
        let mut lines: Vec<String> = vec![
            "Register A: 2024",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 0,1,5,4,3,0",
        ]
        .iter()
        .map(|&line| line.to_string())
        .collect();
        let mut computer: Computer = Computer::new_from_input(&mut lines);
        computer.execute();
        assert_eq!(
            computer.output,
            vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]
        );
        assert_eq!(computer.register_a, 0);
    }

    /// If register B contains 29, the program 1,7 would set register B to 26
    #[test]
    fn test_example_4() {
        let mut lines: Vec<String> = vec![
            "Register A: 0",
            "Register B: 29",
            "Register C: 0",
            "",
            "Program: 1,7",
        ]
        .iter()
        .map(|&line| line.to_string())
        .collect();
        let mut computer: Computer = Computer::new_from_input(&mut lines);
        computer.execute();
        assert_eq!(computer.register_b, 26);
    }

    /// If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    #[test]
    fn test_example_5() {
        let mut lines: Vec<String> = vec![
            "Register A: 0",
            "Register B: 2024",
            "Register C: 43690",
            "",
            "Program: 4,0",
        ]
        .iter()
        .map(|&line| line.to_string())
        .collect();
        let mut computer: Computer = Computer::new_from_input(&mut lines);
        computer.execute();
        assert_eq!(computer.register_b, 44354);
    }

    #[test]
    fn text_copy_of_itself() {
            let mut lines: Vec<String> = vec![
            "Register A: 2024",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 0,3,5,4,3,0",
        ]
        .iter()
        .map(|&line| line.to_string())
        .collect();
        let mut computer: Computer = Computer::new_from_input(&mut lines);
        computer.execute();
        println!("{:?}", computer.output);
        assert!(computer.output.len() > 0);
    }
}
