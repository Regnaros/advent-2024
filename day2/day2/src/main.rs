use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = get_input("../input.txt").unwrap();
    let mut result = 0;

    for report in input{
        match is_report_erronous(&report) {
            None => result += 1,
            _ => continue,
        };
    }

    println!("Result: {}", result);
}

fn is_report_erronous(report: &Vec<i32>) -> Option<usize> {
    let increasing = report[0] - report[1] < 0;

    for index in 0..report.len() -1 {
        let difference = report[index] - report[index +1];

        if difference.abs() > 3 || difference == 0 || (difference > 0 && increasing) || (difference < 0 && !increasing) {
            return Some(index);
        }
    }

    return None;
}

fn part2() {
    let input = get_input("../input.txt").unwrap();
    let mut result = 0;

    for report in input{
        match is_report_erronous_problem_dampening(&report) {
            None => result += 1,
            _ => continue,
        };
    }

    println!("Result: {}", result);
}

fn is_report_erronous_with_exclude(report_input: &Vec<i32>, exclude: usize) -> Option<usize> {

    let mut report = Vec::new();

    report.extend_from_slice(&report_input[0..exclude]);
    report.extend_from_slice(&report_input[exclude + 1..]);

    let increasing = report[0] - report[1] < 0;

    for index in 0..report.len() -1 {
        let difference = report[index] - report[index +1];

        if difference.abs() > 3 || difference == 0 || (difference > 0 && increasing) || (difference < 0 && !increasing) {
            return Some(index);
        }
    }

    return None;
}

fn is_report_erronous_problem_dampening(report: &Vec<i32>) -> Option<usize> {
    let result = is_report_erronous(report);

    if let Some(index) = result {
        if is_report_erronous_with_exclude(report, index) == None {
            return None;
        }

        if is_report_erronous_with_exclude(report, index + 1) == None {
            return None;
        }

        if index > 0 && is_report_erronous_with_exclude(report, index - 1) == None {
            return None;
        }

        return Some(0);
    }

    return None;
}

fn get_input(pathstr: &str)  -> io::Result<Vec<Vec<i32>>> {

    let path = pathstr; // Replace with your file path
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let all_numbers: Vec<Vec<i32>> = reader
        .lines()
        .filter_map(|line| {
            line.ok() // Handle any errors in reading lines
                .map(|line| {
                    line.split_whitespace() // Split the line into numbers
                        .filter_map(|num| num.parse::<i32>().ok()) // Parse each number into i32
                        .collect() // Collect the numbers into a Vec<i32>
                })
        })
        .collect();

    Ok(all_numbers)  
}


// fn get_input() -> String {
//     fs::read_to_string(Path::new("./input/day3.input"))
//         .expect("Something went wrong with the input")
// }