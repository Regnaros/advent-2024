use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string("input.txt")?;
    let mut a_buttons = Vec::new();
    let mut b_buttons = Vec::new();
    let mut prizes = Vec::new();

    for chunk in file_content.split("\n") {
        let mut a_values = None;
        let mut b_values = None;
        let mut prize = None;

        for line in chunk.lines() {
            if line.starts_with("Button A:") {
                if let Some(x) = parse_coordinates(line) {
                    a_values = Some(x);
                }
            } else if line.starts_with("Button B:") {
                if let Some(x) = parse_coordinates(line) {
                    b_values = Some(x);
                }
            } else if line.starts_with("Prize:") {
                if let Some((x, y)) = parse_prize(line) {
                    prize = Some((x, y));
                }
            }
        }

        if let Some(x) = a_values {
            a_buttons.push(x);
        }
        if let Some(x) = b_values {
            b_buttons.push(x);
        }
        if let Some(p) = prize {
            prizes.push(p);
        }
    }

    let mut res_part1 = 0;
    let mut res_part2 = 0;
    for i in 0..prizes.len() {
        res_part1 += part_1(a_buttons[i], b_buttons[i], prizes[i]);
        res_part2 += part_2(a_buttons[i], b_buttons[i], prizes[i]);
    }
   
    println!("part 1 {}", res_part1);
    println!("part 2 {}", res_part2);

    Ok(())
}

fn part_2(a_button: (i64, i64), b_button: (i64, i64), price: (i64, i64)) -> i64 {
    let new_price = (price.0 + 10000000000000, price.1 +10000000000000);
    let b = (new_price.1 * a_button.0 - new_price.0 * a_button.1) / (b_button.1 * a_button.0 - a_button.1 * b_button.0);
    let numerator = new_price.0 - b_button.0 * b;

    if numerator % a_button.0 == 0 {
        let a = numerator / a_button.0;

        if a_button.1 * a + b_button.1 * b == new_price.1 {
            return a * 3 + b
        }
    }
    return 0;
}

fn part_1(a_button: (i64, i64), b_button: (i64, i64), price: (i64, i64)) -> i64 {
    let b = (price.1 * a_button.0 - price.0 * a_button.1) / (b_button.1 * a_button.0 - a_button.1 * b_button.0);
    let numerator = price.0 - b_button.0 * b;

    if numerator % a_button.0 == 0 {
        let a = numerator / a_button.0;

        if a_button.1 * a + b_button.1 * b == price.1 {
            return a * 3 + b
        }
    }
    return 0;
}

fn part_1_uneccesarily_long(a_button: (i64, i64), b_button: (i64, i64), price: (i64, i64)) -> i64 {
    let mut possible_solutions = Vec::new();

    for b in 0..101 {
        let numerator = price.0 - b_button.0 * b;

        if numerator % a_button.0 == 0 {
            let a = numerator / a_button.0;

            if a_button.1 * a + b_button.1 * b == price.1 && a < 101 {
                possible_solutions.push((a, b));
            }
        }
    }

    let mut min = i64::MAX;

    for solution in possible_solutions {
        if solution.0 * 3 + solution.1 < min {
            min = solution.0 * 3 + solution.1;
        }
    }

    if min == i64::MAX {
        return 0;
    }
    else {
        return min;
    }
}

fn parse_coordinates(line: &str) -> Option<(i64, i64)> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() == 2 {
        let x_part = parts[0].trim().split_once('+')?.1.parse().ok()?;
        let y_part = parts[1].trim().split_once('+')?.1.parse().ok()?;
        return Some((x_part, y_part));
    }
    None
}

fn parse_prize(line: &str) -> Option<(i64, i64)> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() == 2 {
        let x_part = parts[0].trim().split_once('=').unwrap().1.parse().ok()?;
        let y_part = parts[1].trim().split_once('=').unwrap().1.parse().ok()?;
        return Some((x_part, y_part));
    }
    None
}
