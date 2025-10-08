use std::{collections::{HashMap, HashSet}, fs, io};

fn main() {
    let data = parse_data("input.txt").unwrap();

    part_1(data.0, data.1);
}

fn parse_data(path: &str) -> io::Result<(Vec<String>, Vec<String>)> {
    let file_contents = fs::read_to_string(path)?;
    let mut lines = file_contents.lines();

    let first_line: Vec<String> = lines
        .next()
        .unwrap_or("")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let other_lines: Vec<String> = lines
        .map(|line| line.to_string())
        .collect();

    Ok((first_line, other_lines[1..].to_vec()))
}

fn part_1(towels: Vec<String>, patterns: Vec<String>){
    // let mut result_part1 = 0;
    // let mut possible_patterns_part1: HashSet<String> = HashSet::new();
    // let mut impossible_patterns_part1: HashSet<String> = HashSet::new();

    // for pattern in &patterns {
    //     if validate_pattern(&towels, &pattern, &mut possible_patterns_part1, &mut impossible_patterns_part1) {
    //         result_part1 += 1;
    //     }
    // }

    // println!("{}", result_part1);

    let mut result_part11 = 0;
    let mut possible_patterns_part11: HashMap<String, bool> = HashMap::new();

    for pattern in &patterns {
        if validate_pattern_part1(&towels, &pattern, &mut possible_patterns_part11) {
            result_part11 += 1;
        }
    }

    println!("{}", result_part11);

    let mut result_part2 = 0;
    let mut possible_patterns_part2: HashMap<String, u64> = HashMap::new();

    for pattern in patterns {
        let res = validate_pattern_part2(&towels, &pattern, &mut possible_patterns_part2);
        result_part2 += res;
    }

    println!("{}", result_part2);
}

fn validate_pattern_part2(towels: &Vec<String>, pattern: &str, possible_patterns: &mut HashMap<String, u64>) -> u64 {

    if let Some(amount_of_patterns) = possible_patterns.get(&pattern.to_string()) {
        return *amount_of_patterns;
    }

    let mut result = 0;

    if pattern == "" {
        result += 1;
    }

    for towel in towels {
        if pattern.starts_with(towel) {
            result += validate_pattern_part2(towels, &pattern[towel.len()..], possible_patterns);
        } 
    }

    possible_patterns.insert(pattern.to_string(), result);
    result
}

fn validate_pattern_part1(towels: &Vec<String>, pattern: &str, possible_patterns: &mut HashMap<String, bool>) -> bool {

    if let Some(possible) = possible_patterns.get(&pattern.to_string()) {
        return *possible;
    }

    if pattern == "" {
        return true;
    }

    for towel in towels {
        if pattern.starts_with(towel) {
            if validate_pattern_part1(towels, &pattern[towel.len()..], possible_patterns) {
                possible_patterns.insert(pattern.to_string(), true);
                return true;
            }
        } 
    }

    possible_patterns.insert(pattern.to_string(), false);
    false
}

// fn validate_pattern(towels: &Vec<String>, pattern: &str, possible_patterns: &mut HashSet<String>, impossible_patterns: &mut HashSet<String>) -> bool {
    
//     if possible_patterns.contains(pattern) {
//         return true;
//     }

//     if impossible_patterns.contains(pattern) {
//         return false;
//     }

//     for towel in towels {
//         if towel == pattern { 
//             possible_patterns.insert((*pattern).to_string());
//             return true; 
//         }
//     }

//     let mut found_pattern = false;

//     for towel in towels {
//         match pattern.split_once(towel) {
//             Some(("", right_remainder)) => {
//                 found_pattern = validate_pattern(towels, right_remainder, possible_patterns, impossible_patterns)
//             }
//             Some((left_remainder, "")) => {
//                 found_pattern = validate_pattern(towels, left_remainder, possible_patterns, impossible_patterns)
//             }
//             Some((left_remainder, right_remainder)) => {
//                 found_pattern = validate_pattern(towels, left_remainder, possible_patterns, impossible_patterns) && validate_pattern(towels, right_remainder, possible_patterns, impossible_patterns)
//             }
//             _ => {}
//         }

//         if found_pattern { 
//             possible_patterns.insert((*pattern).to_string());
//             return true; 
//         }
//     }

//     impossible_patterns.insert((*pattern).to_string());
//     found_pattern
// }