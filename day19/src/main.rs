use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

fn main() {
    println!("Advent of Code 2024 - day19");
    println!("Part 1: {}", process("challenge.txt"));
    println!("Part 2: {}", part2("challenge.txt"));
}

fn part1(file_path: &str) -> i32 {
    let mut lines = read_file(file_path).unwrap();
    let first_line = lines.remove(0);
    let mut towel_patterns = first_line
        .split_whitespace()
        .map(|pattern| pattern.replace(",", ""))
        .collect::<Vec<String>>();
    towel_patterns.sort();
    lines.remove(0); // Skip the empty line
    lines
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, design)| {
            if design_is_possible2(&design, &towel_patterns) {
                println!("Design {} is possible", index);
                return acc + 1;
            }
            acc
        })
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

fn design_is_possible(design: String, towel_patterns: &HashSet<String>) -> bool {
    if design.is_empty() {
        return true;
    }

    for pattern in towel_patterns {
        if design.starts_with(pattern)
            && design_is_possible(String::from(&design[pattern.len()..]), towel_patterns)
        {
            return true;
        }
    }

    return false;
}

fn design_is_possible2(design: &str, towel_patterns: &Vec<String>) -> bool {
    if design.is_empty() {
        return true;
    }

    for pattern in towel_patterns {
        if pattern.len() <= design.len()
            && design.starts_with(pattern)
            && design_is_possible2(&design[pattern.len()..], towel_patterns)
        {
            return true;
        }
    }

    return false;
}

pub fn process(input: &str) -> usize {
    let mut lines = read_file(input).unwrap();
    let first_line = lines.remove(0);
    let mut towel_patterns: Vec<String> = first_line
        .split_whitespace()
        .map(|pattern| pattern.replace(",", ""))
        .collect();
    towel_patterns.sort();
    lines.remove(0); // Skip the empty line

    let towel_patterns: Vec<&str> = towel_patterns.iter().map(|s| s.as_str()).collect();
    let count = lines
        .iter()
        .filter(|design| validate_design(design, &towel_patterns))
        .count();

    count
}

fn validate_design(design: &str, towels: &[&str]) -> bool {
    return towels
        .iter()
        .map(|towel| {
            if design.starts_with(*towel) {
                let new_design = &design[towel.len()..];
                if new_design.is_empty() {
                    return true;
                }
                validate_design(new_design, towels)
            } else {
                false
            }
        })
        .any(|v| v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test.txt"), 6);
    }

    #[test]
    fn test_empty_design() {
        let design = String::from("");
        let towel_patterns = HashSet::new();
        assert_eq!(design_is_possible(design, &towel_patterns), true);
    }

    #[test]
    fn test_design_found_with_one_pattern() {
        let design = String::from("k");
        let mut towel_patterns = HashSet::new();
        towel_patterns.insert(String::from("k"));
        assert_eq!(design_is_possible(design, &towel_patterns), true);
    }

    #[test]
    fn test_design_found_with_one_pattern_of_two_letters() {
        let design = String::from("ka");
        let mut towel_patterns = HashSet::new();
        towel_patterns.insert(String::from("k"));
        towel_patterns.insert(String::from("ka"));
        assert_eq!(design_is_possible(design, &towel_patterns), true);
    }

    #[test]
    fn test_design_not_found_with_one_pattern() {
        let design = String::from("u");
        let mut towel_patterns = HashSet::new();
        towel_patterns.insert(String::from("k"));
        assert_eq!(design_is_possible(design, &towel_patterns), false);
    }

    #[test]
    fn test_examples_from_sample() {
        let mut towel_patterns: HashSet<String> = HashSet::new();
        towel_patterns.extend(
            vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]
                .into_iter()
                .map(String::from),
        );

        let design = String::from("brwrr");
        assert_eq!(design_is_possible(design, &towel_patterns), true);

        let design = String::from("bggr");
        assert_eq!(design_is_possible(design, &towel_patterns), true);

        let design = String::from("gbbr");
        assert_eq!(design_is_possible(design, &towel_patterns), true);

        let design = String::from("rrbgbr");
        assert_eq!(design_is_possible(design, &towel_patterns), true);

        let design = String::from("bwurrg");
        assert_eq!(design_is_possible(design, &towel_patterns), true);

        let design = String::from("brgr");
        assert_eq!(design_is_possible(design, &towel_patterns), true);

        let design = String::from("ubwu");
        assert_eq!(design_is_possible(design, &towel_patterns), false);

        let design = String::from("bbrgwb");
        assert_eq!(design_is_possible(design, &towel_patterns), false);
    }

    #[test]
    fn test_from_challenge() {
        let first_line = String::from("rrbb, rgrb, rbbwbw, ruw, grwb, wbg, rrggu, wugbb, gwr, rrgu, wbbb, bwrgugb, rgww, brrw, gwwg, brgbb, ubbbu, rbgruw, bwub, rru, grbu, grw, bwrbr, wrwbwu, bgbr, urrb, bbb, guggb, gwuwuw, ubu, wbr, bb, ubwwrb, bruuwu, buw, wrr, gbgrr, ggw, ru, wub, uub, gbrw, uwrwrb, uwuu, wbbu, gwu, brrr, grggur, wugrbrw, rwuurgg, wrbg, wuugrgw, brr, wrgwu, rwwuur, wgug, guu, brru, bgr, buwruw, bbg, bw, brwuwbb, buwbrgrb, wrwuw, ubb, rgu, wru, ugw, rruubrr, bwur, rrb, uwg, wwbbuwu, rbw, uwrgbr, ubuw, uurwr, uggw, uuguur, guuww, bbrgw, ubuwbuu, buwgg, gru, uuu, u, wgguw, bruu, bu, gwurrw, rbgwur, gubg, ubgrubw, buwrgw, uu, rwgw, rrw, wgru, gwrb, bgguu, gwugwu, urrbg, ruwwb, wuw, wwgg, wwbbu, gggb, rgrruwrw, uugr, rwb, rggw, gbr, guuurru, wurubggw, gww, wuugb, rbbww, urwgb, wrruurr, guuurgg, bwbgbg, bubwb, ggrwwg, gugr, uwrrr, guurbu, uruw, gbguruu, grb, bwr, wrubggr, rbu, wgwgru, buuww, gub, wu, uw, rrbrb, ur, wrgbb, gbbu, wgw, ggwbgwb, wb, urwr, uggwbwu, wwwg, rbru, bwbw, brb, wbugw, r, gbb, rw, bru, wbbwbr, gwww, wgrr, rugg, bgrbrub, w, gwwb, wrwgu, wwgb, uwgbgwwb, wrrwb, ubw, rr, gbbuug, bbr, wugb, rbuww, uur, wgur, ugg, wwu, wwrww, uuwur, rwbbww, gbu, bgbuur, rrbugguw, ugrgwg, wurruu, rrrgww, ggbbgbb, bgbb, brrb, rwwr, brwwr, ubrbru, uburwwb, ggwu, rwbgrbu, bbwuwbr, urwggbwr, bbrgu, wbbuww, rbwwuwr, gu, brbb, wbwgu, ugbrrguw, urbu, gbgbbrub, gbuuuww, rww, gwwgwwg, buwu, bur, uubrw, rur, rgurr, rwbgrg, urr, wugw, wuur, wuub, wbgb, bgb, uubrbur, rrbugbu, ugru, guwr, uwu, uru, wbw, wugrwgb, ggwg, guw, ubbbg, grbuwg, gw, wgb, bbrbwgub, gr, rbruu, urwwu, gbuwbw, bug, rgrgb, uwr, wrw, urb, wgbw, rbbgr, rwww, wbubur, buubw, ruwgwbug, wwwgggub, uuwrwbww, burr, rubg, wgub, bww, urbuubb, bubugrb, rgwbbw, rb, ggu, gwg, rgw, ruwggbg, wrrwbg, gur, wwbgugb, ruu, rrgg, rwrr, uwb, bwbbwbg, rbgg, rwgwg, rrgrwg, gb, bgbg, urubwugb, ruww, grbbg, gwb, bugrr, wgwr, wuug, b, gbwgr, uugw, wwr, wuu, ggr, bbwb, ubbbwrg, wugbw, gguuw, bbw, ugr, wbbrrbu, grbub, ubuwuw, bwgw, bggw, bwu, ub, bwubw, rgru, brg, uruwr, rug, wrwwuubg, brw, wggurb, br, wgr, gug, wrww, gg, uuw, bguwbru, rrrw, uubgb, gbww, buu, rbbuw, bgru, ubbwr, bg, gbgrww, wrg, ugubw, rgrg, bwbbgw, ggubrwbg, uurgwg, gbw, ww, wr, wbgu, bbu, brrwb, wug, uurug, rgb, gwbwbgg, uuubg, ugub, bruw, rub, rgggrr, ruubrwuw, brguru, wbrbg, rrwr, bgrwbbr, ugu, grwbb, rgg, wwg, rgbuwr, wuwur, bbgub, ubg, rrg, wrur, bggbr, wgg, bbwbbru, bwwbrgu, rugrr, rgwbu, ugubwbgu, wwb, ggg, bwwu, gwuguu, rrwgwg, uuwu, rrug, bgwruw, brrgrb, rurg, bugb, grwbg, rwr, urug, rwg, bgg, gbggr, rrr, gbbrw, wgu, bwbg, bbgurggg, wuwguw, wwwuru, grg, grggbw, rubu, burw, grr, bgbbw, wgrbgg, rwrww, gwurgr, bruww, bub, wbu, rbb, uwgbb, rgr, ugb, buubb, wg, wwwgr, bgw, wbwr, wuwruu, wrgrrrw, rbr, wuwwr, wrwb, ggwgwbrw, urw, uwrubb, wuwg, wguu, ugwwuw, ggb, rrggr, wrb, urwgub, bgwwrw, rwu, wurug, gbgu, brrwbb, wwbub, uwrgw, bwb, ubr, uww, gbg, wgrugwbb, bwg, wbb, bbwwgr, www");
        let mut towel_patterns = first_line
            .split_whitespace()
            .map(|pattern| pattern.replace(",", ""))
            .collect::<Vec<String>>();
        towel_patterns.sort();

        let design = String::from("ugguwugrwrgwruwguuwurubggwgwbrwwwubburwrburg");
        assert_eq!(design_is_possible2(&design, &towel_patterns), false);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test.txt"), 0);
    }
}
