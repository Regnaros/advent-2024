use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part2();
}

fn part1() -> io::Result<()> {
        let path = "../../input.txt";
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
    
        let mut left_numbers: Vec<i32> = Vec::new();
        let mut right_numbers: Vec<i32> = Vec::new();
        for line in reader.lines() {
            let line = line?;
    
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let [left, right] = parts[..] {
                left_numbers.push(left.parse::<i32>().unwrap_or_default());
                right_numbers.push(right.parse::<i32>().unwrap_or_default());
            }
        }
    
        left_numbers.sort();
        right_numbers.sort();
    
        let mut result = 0;
    
        for index in 0..left_numbers.len() {
            result += left_numbers[index].abs_diff(right_numbers[index])
        }
    
        println!(" Value: {}", result);
    
        Ok(())
}

fn part2() -> io::Result<()> {
    let path = "../../input.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut left_numbers: Vec<u32> = Vec::new();
    let mut right_numbers: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split_whitespace().collect();
        if let [left, right] = parts[..] {
            left_numbers.push(left.parse::<u32>().unwrap_or_default());
            right_numbers.push(right.parse::<u32>().unwrap_or_default());
        }
    }

    left_numbers.sort();
    right_numbers.sort();

    let mut result = 0;

    for left_number in left_numbers {
        let mut occurances = 0;

        for right_number in &right_numbers {
            if left_number == *right_number {
                occurances += 1;
            }
        }

        result += occurances * left_number;
    }

    println!(" Value: {}", result);

    Ok(())
}
