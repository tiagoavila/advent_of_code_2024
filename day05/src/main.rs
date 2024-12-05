use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    println!("Advent of Code 2024 - Day 05");
    println!("Part 1: {}", part1("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> i32 {
    let file = read_file(file_path).unwrap();
    let (page_ordering_rules, pages_to_produce) = split_input(file);
    let page_ordering_rules = parse_page_ordering_rules_to_list_of_tuples(page_ordering_rules);
    let pages_to_produce = parse_pages_to_produce_vec_of_vec(pages_to_produce);
    pages_to_produce
        .iter()
        .filter(|x| validate_line(x, &page_ordering_rules))
        .collect::<Vec<&Vec<i32>>>()
        .iter()
        .map(|x| get_middle_element(x))
        .sum()
}

fn get_middle_element(vec: &Vec<i32>) -> i32 {
    let mid_index = vec.len() / 2;
    
    // If you want the "lower" middle in case of an even-length vector
    vec[mid_index]
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

fn split_input(vec: Vec<String>) -> (Vec<String>, Vec<String>) {
    if let Some(index) = vec.iter().position(|x| x == "") {
        let before = vec[..index].to_vec();
        let after = vec[index + 1..].to_vec();
        (before, after)
    } else {
        // If the value is not found, return the original vector and an empty one
        (vec, Vec::new())
    }
}

fn parse_page_ordering_rules_to_list_of_tuples(
    page_ordering_rules: Vec<String>,
) -> Vec<(i32, i32)> {
    let page_ordering_rules: Vec<(i32, i32)> = page_ordering_rules
        .iter()
        .map(|x| {
            let string_tuple = x.split_once("|").unwrap();
            (
                string_tuple.0.parse::<i32>().unwrap(),
                string_tuple.1.parse::<i32>().unwrap(),
            )
        })
        .collect();
    page_ordering_rules
}

fn parse_pages_to_produce_vec_of_vec(pages_to_produce: Vec<String>) -> Vec<Vec<i32>> {
    let pages_to_produce: Vec<Vec<i32>> = pages_to_produce
        .iter()
        .map(parse_page_line_to_vector)
        .collect();
    pages_to_produce
}

fn parse_page_line_to_vector(x: &String) -> Vec<i32> {
    x.split(',')
        .filter_map(|s| s.parse::<i32>().ok()) // Parse each value and skip invalid ones
        .collect()
}

fn find_applicable_rules_by_page_line(
    page_line: &Vec<i32>,
    page_ordering_rules: &Vec<(i32, i32)>,
) -> Vec<(i32, i32)> {
    let mut applicable_rules = Vec::new();
    let page_line_hashset: HashSet<i32> = page_line.iter().cloned().collect();
    for rule in page_ordering_rules {
        if page_line.contains(&rule.0) && page_line_hashset.contains(&rule.1) {
            applicable_rules.push(*rule);
        }
    }

    applicable_rules
}

fn validate_line(page_line: &Vec<i32>, page_ordering_rules: &Vec<(i32, i32)>) -> bool {
    let applicable_rules = find_applicable_rules_by_page_line(&page_line, page_ordering_rules);
    let mut valid = true;
    for rule in applicable_rules {
        let index_first_page = page_line.iter().position(|&x| x == rule.0).unwrap();
        let index_second_page = page_line.iter().position(|&x| x == rule.1).unwrap();
        if index_first_page > index_second_page {
            valid = false;
            break;
        }
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test.txt"), 143);
    }

    #[test]
    fn test_find_applicable_rules_by_page_line() {
        let page_line = parse_page_line_to_vector(&"75,47,61,53,29".to_string());
        let page_ordering_rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let applicable_rules = find_applicable_rules_by_page_line(&page_line, &page_ordering_rules);
        let expected_rules = vec![(47, 53), (75, 29), (75, 53), (53, 29), (61, 53), (61, 29), (75, 47), (47, 61), (75, 61), (47, 29)];
        assert!(applicable_rules.len() == expected_rules.len());
        assert!(applicable_rules == expected_rules);
    }

    #[test]
    fn test_validate_line_that_is_valid() {
        let page_line = parse_page_line_to_vector(&"75,47,61,53,29".to_string());
        let page_ordering_rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        assert!(validate_line(&page_line, &page_ordering_rules));
    }

    #[test]
    fn test_validate_line_that_is_invalid() {
        let page_line = parse_page_line_to_vector(&"75,97,47,61,53".to_string());
        let page_ordering_rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        assert_eq!(validate_line(&page_line, &page_ordering_rules), false);
    }
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("test.txt"), 0);
    // }
}
