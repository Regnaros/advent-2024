use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

const SIZE: usize = 130;

fn main() {
    let input = load_matrix_from_file("input.txt").unwrap();

    let patrol_position = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(move |(j, &c)| if c == '^' { Some((i, j)) } else { None })
        })
        .next()
        .expect("No '^'");

    let patrol = (patrol_position.0 as i32, patrol_position.1 as i32);

    let path = part1(&input, patrol);

    let patrol = (patrol_position.0 as i32, patrol_position.1 as i32);

    part2(input, patrol, path);
}

fn part2(mut input: [[char; SIZE]; SIZE], original_patrol: (i32, i32), path:HashSet<(usize, usize)>) {

    let mut patrol;

    let directions_rows = vec![-1, 0, 1, 0];
    let directions_columns = vec![0, 1, 0, -1];
    let mut direction;
    let max = SIZE as i32;

    let mut visisted_with_direction = HashSet::new();
    let mut new_obs = HashSet::new();
    
    for element in &path {
        if element.0 == original_patrol.0 as usize && element.1 == original_patrol.1 as usize { continue; }

        input[element.0][element.1] = '#';

        patrol = (original_patrol.0, original_patrol.1);
        direction = 0;
        visisted_with_direction.clear();


        while patrol.0 > 0 && patrol.0 < max -1 && patrol.1 > 0 && patrol.1 < max -1  {
            let next_step: (usize, usize) = ((patrol.0 + directions_rows[direction % 4]) as usize, (patrol.1 + directions_columns[direction % 4]) as usize);
    
            match input[next_step.0][next_step.1] {
                '#' => { 
                    if !visisted_with_direction.insert((patrol.0, patrol.1, direction % 4)) {

                        new_obs.insert(element);
                        break;
                    }

                    direction += 1
                },
                _ => { 
                    patrol = (next_step.0 as i32, next_step.1 as i32);
                }
            }
        }

        input[element.0][element.1] = '.';
    }

    println!("{}", new_obs.len());
}

fn part1(input: &[[char; SIZE]; SIZE], mut patrol: (i32, i32)) -> HashSet<(usize, usize)>  {
    
    let directions_rows = vec![-1, 0, 1, 0];
    let directions_columns = vec![0, 1, 0, -1];

    let mut direction = 0;
    
    let max = SIZE as i32;

    let mut visited = HashSet::new();

    while patrol.0 > 0 && patrol.0 < max -1 && patrol.1 > 0 && patrol.1 < max -1  {
        let next_step: (usize, usize) = ((patrol.0 + directions_rows[direction % 4]) as usize, (patrol.1 + directions_columns[direction % 4]) as usize);

        match input[next_step.0][next_step.1] {
            '#' => direction += 1,
            _ => { 
                visited.insert(next_step);
                patrol = (next_step.0 as i32, next_step.1 as i32);
            }
        }
    }
    
    println!("visited {}", visited.len());
    return visited;
}

fn load_matrix_from_file(file_path: &str) -> io::Result<[[char; SIZE]; SIZE]> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Initialize the matrix
    let mut matrix = [[' '; SIZE]; SIZE];

    // Iterate through lines and populate the matrix
    for (row_index, line) in reader.lines().enumerate() {
        let line = line?;
        if row_index >= SIZE { // There should only be one row
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many rows"));
        }
        let chars: Vec<char> = line.chars().collect();
        if chars.len() != SIZE {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Incorrect row length"));
        }
        for (col_index, ch) in chars.into_iter().enumerate() {
            matrix[row_index][col_index] = ch;
        }
    }

    Ok(matrix)
}