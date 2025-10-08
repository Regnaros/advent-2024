use std::fs::File;
use std::io::{self, BufRead};

fn main(){
    let input= load_data("input.txt").unwrap();

    part1(&input);
    part2(input);
}

fn part2(input: Vec<(u64, Vec<u64>)>) {
    let mut result = 0;
    let mut result2 = 0;

    for equation in input {
        let correct_result = equation.0.clone();
        let numbers = equation.1.clone();
        if solve_equation_part2(correct_result, numbers[0], &numbers[1..]) {
            result += correct_result;
        }
        if solve_equation_part2_with_zero_guard(correct_result, 0, &numbers, true) {
            result2 += correct_result;
        }
    }

    println!("{}", result);
    println!("{}", result2);
}

fn combine_numbers(a: &u64, b: &u64) -> u64 {
    if *a == 0 { return *b }

    let b = b.clone();
    let b_digits = (b as f64).log10().floor() as u32 + 1;
    a * 10_u64.pow(b_digits) + b
}

fn solve_equation_part2(correct_value: u64, current_value: u64, numbers: &[u64]) -> bool {
    match numbers.len() {
        1 => current_value * numbers[0] == correct_value ||
             current_value + numbers[0] == correct_value ||
             combine_numbers(&current_value, &numbers[0]) == correct_value,
        _ => {
            let first = numbers[0];
            let rest = &numbers[1..];

            let combined = combine_numbers(&current_value, &first);

            if combined <= correct_value && 
                solve_equation_part2(correct_value, combined, &numbers[1..]) {

                return true;
            }

            if current_value * first <= correct_value && 
                solve_equation_part2(correct_value, current_value * first, rest) {
                

                return true;
            }

            if current_value + first <= correct_value && 
                solve_equation_part2(correct_value, current_value + first, rest) {    

                return true;
            }

            return false
        }
    }
}

fn solve_equation_part2_with_zero_guard(correct_value: u64, current_value: u64, numbers: &[u64], first_call: bool) -> bool {
    match numbers.len() {
        1 => current_value * numbers[0] == correct_value ||
             current_value + numbers[0] == correct_value ||
             combine_numbers(&current_value, &numbers[0]) == correct_value,
        _ => {
            let first = numbers[0];
            let rest = &numbers[1..];

            let combined = combine_numbers(&current_value, &first);
            let mut zero_guard = current_value;

            if first_call {
                zero_guard = 1;
            }

            if combined <= correct_value && 
                solve_equation_part2_with_zero_guard(correct_value, combined, &numbers[1..],false) {

                return true;
            }

            if zero_guard * first <= correct_value && 
                solve_equation_part2_with_zero_guard(correct_value, zero_guard * first, rest, false) {
                

                return true;
            }

            if current_value + first <= correct_value && 
                solve_equation_part2_with_zero_guard(correct_value, current_value + first, rest, false) {    

                return true;
            }

            return false
        }
    }
}

fn part1(input: &Vec<(u64, Vec<u64>)>) {

    let mut result = 0;

    for equation in input {
        let correct_result = equation.0.clone();
        let numbers = equation.1.clone();
        if solve_equation(correct_result, 0, &numbers) {
            result += correct_result;
        }
    }

    println!("{}", result);
}

fn solve_equation(correct_value: u64, current_value: u64, numbers: &[u64]) -> bool {
    match numbers.len() {
        0 => correct_value == current_value,
        1 => current_value * numbers[0] == correct_value ||
             current_value + numbers[0] == correct_value,
        _ => {
            let first = numbers[0];
            let rest = &numbers[1..];

            if current_value * first <= correct_value && 
               solve_equation(correct_value, current_value * first, rest) {

                return true;
            }

            if current_value + first <= correct_value && 
               solve_equation(correct_value, current_value + first, rest) {

                return true;
            }

            return false
        }
    }
}

fn load_data(file_path: &str) -> io::Result<Vec<(u64, Vec<u64>)>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut data = Vec::new(); 

    for line in reader.lines() {
        let line = line?;
        if let Some((id, values)) = parse_line(&line) {
            data.push((id, values));
        } else {
            println!("Invalid line format: {}", line);
        }
    }

    Ok(data)
}

fn parse_line(line: &str) -> Option<(u64, Vec<u64>)> {
    let mut parts = line.splitn(2, ':');
    let id_part = parts.next()?.trim();
    let values_part = parts.next()?.trim();

    let id = id_part.parse::<u64>().ok()?;

    let values = values_part
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    Some((id, values))
}
