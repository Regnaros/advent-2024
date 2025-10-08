use std::collections::HashSet;
use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut p_coordinates = Vec::new();
    let mut velocities = Vec::new();

    // Compile the regex pattern to capture the p and v coordinates
    let re = Regex::new(r"p=(-?\d+),(-?\d+)\s*v=(-?\d+),(-?\d+)").unwrap();

    for line in reader.lines() {
        let line = line?;

        // Apply the regex to each line
        if let Some(captures) = re.captures(&line) {
            // Capture groups for p and v coordinates
            let p_x: i32 = captures[1].parse().unwrap();
            let p_y: i32 = captures[2].parse().unwrap();
            let v_x: i32 = captures[3].parse().unwrap();
            let v_y: i32 = captures[4].parse().unwrap();

            // Store the coordinates in their respective lists
            p_coordinates.push((p_x, p_y));
            velocities.push((v_x, v_y));
        }
    }

    part_1(&p_coordinates, &velocities);
    part_1_and_2_combined(p_coordinates, &velocities);

    Ok(())
}

fn are_all_tuples_unique(vec: &Vec<(i32, i32)>) -> bool {
    let mut set = HashSet::new();
    vec.iter().all(|&tuple| set.insert(tuple))
}

fn part_1_and_2_combined(mut p_coordinates: Vec<(i32, i32)>, velocities: &Vec<(i32, i32)>) {
    let grid_width = 101;
    let grid_height = 103;
    let mut not_unique = true;
    let mut seconds = 0;

    while not_unique {
        seconds += 1;
        for i in 0..p_coordinates.len() {
            let y: i32 = {
                let new_y = p_coordinates[i].1 + velocities[i].1;
                ((new_y % grid_height) + grid_height) % grid_height
            };
    
            let x: i32 = {
                let new_x = p_coordinates[i].0 + velocities[i].0;
                ((new_x % grid_width) + grid_width) % grid_width
            };

            p_coordinates[i] = (x, y);
        }

        not_unique = !are_all_tuples_unique(&p_coordinates);

        if seconds == 100 {
            let mut first = 0;
            let mut second = 0;
            let mut third = 0;
            let mut fourth = 0;

            for position in &p_coordinates {
                if position.0 < 50 && position.1 < 51 {
                    first += 1;
                }
                if position.0 > 50 && position.1 < 51 {
                    second += 1;
                }
                if position.0 < 50 && position.1 > 51 {
                    third += 1;
                }
                if position.0 > 50 && position.1 > 51 {
                    fourth += 1;
                }
            }

            println!("first {}, second {}, third {}, fourth {}, total {}", first, second, third, fourth, first * second * third * fourth);
        }

    }
    println!("{}", seconds);
}

fn part_1(p_coordinates: &Vec<(i32, i32)>, velocities: &Vec<(i32, i32)>) {
    let grid_width = 101;
    let grid_height = 103;

    let mut final_positions = Vec::new();

    for current in 0..p_coordinates.len() {
        let y: i32 = {
            let new_y = p_coordinates[current].1 + velocities[current].1 * 100;
            ((new_y % grid_height) + grid_height) % grid_height
        };

        let x: i32 = {
            let new_x = p_coordinates[current].0 + velocities[current].0 * 100;
            ((new_x % grid_width) + grid_width) % grid_width
        };

        if x < 0 || x > 100 || y < 0 || y > 102 {
            println!("i: {}, {:?}, x: {}, y: {}", current, p_coordinates[current], x, y);
        }

        final_positions.push((x, y));
    }


    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut fourth = 0;

    for position in final_positions {
        if position.0 < 50 && position.1 < 51 {
            first += 1;
        }
        if position.0 > 50 && position.1 < 51 {
            second += 1;
        }
        if position.0 < 50 && position.1 > 51 {
            third += 1;
        }
        if position.0 > 50 && position.1 > 51 {
            fourth += 1;
        }
    }

    println!("first {}, second {}, third {}, fourth {}, total {}", first, second, third, fourth, first * second * third * fourth);
}
