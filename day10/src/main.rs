use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

const TEST_SIZE: usize = 45;

fn main() {
    let input = load_matrix_from_file("input.txt").unwrap();

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &[[u8; TEST_SIZE]; TEST_SIZE]) {
    let mut trail_points = 0;

    for row in 0..input.len() {
        for column in 0..input[row].len() {
            if input[row][column] == 0 {
                let trial_result_vector = find_trail_score(0, row as i8, column as i8, input);

                let resultset: HashSet<(i8, i8)> = trial_result_vector.into_iter().collect();
                trail_points += resultset.len();
            }
        }
    }

    println!("result {}", trail_points);
}

fn part_2(input: &[[u8; TEST_SIZE]; TEST_SIZE]) {
    let mut result = 0;

    for row in 0..input.len() {
        for column in 0..input[row].len() {
            if input[row][column] == 0 {
                result += find_trail_score_part2(0, row as i8, column as i8, input);
            }
        }
    }

    println!("result {}", result);
}

fn find_trail_score(current: u8, row: i8, column: i8, input: &[[u8; TEST_SIZE]; TEST_SIZE]) -> Vec<(i8, i8)> {
    if current == 9 {
        return vec![(row, column)];
    }

    let dx: [i8; 4] = [0, -1, 0, 1]; //Left, Up, Right, Down
    let dy: [i8; 4] = [-1, 0, 1, 0];

    let max = TEST_SIZE as i8;

    let mut result = vec![];

    for i in 0..4 {
        if row + dx[i] >= 0 && row + dx[i] < max &&
           column + dy[i] >= 0 && column + dy[i] < max &&
           input[(row + dx[i]) as usize][(column + dy[i]) as usize] == current + 1 {
            
            result.extend(find_trail_score(current + 1, row + dx[i], column + dy[i], input));
        }
    }

    return result;
}

fn find_trail_score_part2(current: u8, row: i8, column: i8, input: &[[u8; TEST_SIZE]; TEST_SIZE]) -> u16 {
    if current == 9 {
        return 1
    }

    let dx: [i8; 4] = [0, -1, 0, 1]; //Left, Up, Right, Down
    let dy: [i8; 4] = [-1, 0, 1, 0];

    let max = TEST_SIZE as i8;

    let mut result = 0;

    for i in 0..4 {
        if row + dx[i] >= 0 && row + dx[i] < max &&
           column + dy[i] >= 0 && column + dy[i] < max &&
           input[(row + dx[i]) as usize][(column + dy[i]) as usize] == current + 1 {
            
            result += find_trail_score_part2(current + 1, row + dx[i], column + dy[i], input);
        }
    }

    return result;
}

fn load_matrix_from_file(file_path: &str) -> io::Result<[[u8; TEST_SIZE]; TEST_SIZE]> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut matrix = [[0; TEST_SIZE]; TEST_SIZE]; 

    for (row_idx, line) in reader.lines().enumerate() {
        let line = line?; // Read each line
        for (col_idx, ch) in line.chars().enumerate() {
            if let Some(digit) = ch.to_digit(10) {
                matrix[row_idx][col_idx] = digit as u8; // Convert character to u8 and store
            }
        }
    }

    Ok(matrix)
}
