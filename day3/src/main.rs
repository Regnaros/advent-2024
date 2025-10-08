use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    part1();
    part2();
}

fn part2() {
    let input = get_input("input.txt").unwrap().join("");
    let modified_input = format!("do(){}don't()", input);
    let regex = Regex::new(r"do\(\)(.*?)don\'t\(\)").unwrap();

    let mut result = 0;    

    for caps in regex.captures_iter(&modified_input) {
        result += perform_multiplications(caps.get(1).unwrap().as_str())
    }

    println!("value: {}", result);
}

fn perform_multiplications(input: &str) -> i32 {

    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut result = 0;

    for caps in regex.captures_iter(input) {
        let first_number = caps.get(1).unwrap().as_str();
        let second_number = caps.get(2).unwrap().as_str();

        result += first_number.parse::<i32>().unwrap() * second_number.parse::<i32>().unwrap();
    }

    return result;
}

fn part1() {
    let input = get_input("input.txt").unwrap();

    let mut result = 0;

    for line in input {
        result += perform_multiplications(&line);
    }

    println!("result: {}", result);
}

fn get_input(pathstr: &str)  -> io::Result<Vec<String>> {
    let file = File::open(&pathstr)?;

    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    Ok(lines)  
}
