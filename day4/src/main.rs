use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

const SIZE: usize = 140;

fn main() {
    let input = load_matrix_from_file("input.txt").unwrap();

    let start = Instant::now();
    part1(&input);
    let duration = start.elapsed();
    println!("method_one took: {:?}", duration);
    let start = Instant::now();
    part2(&input);
    let duration = start.elapsed();
    println!("method_two took: {:?}", duration);
}

fn part2(input: &[[char; SIZE]; SIZE]) {
    let mut result = 0;

    for row in 0..SIZE {
        for column in 0..SIZE {
            if column > 0 && column < SIZE - 1 && row > 0 && row < SIZE - 1 && input[row][column] == 'A' {
                if ((input[row - 1][column - 1] == 'M' && input[row + 1][column + 1] == 'S') ||
                   (input[row - 1][column - 1] == 'S' && input[row + 1][column + 1] == 'M')) &&
                   ((input[row + 1][column - 1] == 'M' && input[row - 1][column + 1] == 'S') ||
                   (input[row + 1][column - 1] == 'S' && input[row - 1][column + 1] == 'M')) {
                    
                    result += 1;
                }
            } 
        }
    };

    println!("result {}", result);
}

fn part1(input: &[[char; SIZE]; SIZE]) {
    let mut result = 0;

    for row in 0..SIZE {
        for column in 0..SIZE {
            result += check_horizontal(row, column, input);
            result += check_vertical(row, column, input);
            result += check_diagonals(row, column, input);
        }
    };

    println!("result {}", result);
}

fn check_horizontal(row: usize, column: usize, chars: &[[char; SIZE]; SIZE]) -> usize {

    if chars[row][column] != 'X' { return 0; }

    let mut result: usize = 0;

    if column > 2 {
        let slice: String = chars[row][column - 3..column].iter().collect();

        if chars[row][column] == 'X' && slice == "SAM" { result += 1; } 
    }

    if column < SIZE - 3 {
        let slice: String = chars[row][column + 1..column + 4].iter().collect();

        if chars[row][column] == 'X' && slice == "MAS" { result += 1; } 
    }

    return result;
}

fn check_vertical(row: usize, column: usize, chars: &[[char; SIZE]; SIZE]) -> usize {

    if chars[row][column] != 'X' { return 0; }

    let mut result = 0;

    if row > 2 && chars[row - 1][column] == 'M' && chars[row - 2][column] == 'A' && chars[row - 3][column] == 'S' {
        result += 1;
    }

    if row < SIZE - 3 && chars[row + 1][column] == 'M' && chars[row + 2][column] == 'A' && chars[row + 3][column] == 'S' {
        result += 1;
    }

    return result;
}

fn check_diagonals(row: usize, column: usize, chars: &[[char; SIZE]; SIZE]) -> usize {

    if chars[row][column] != 'X' { return 0; }

    let mut result = 0;

    if row > 2 && column > 2 && chars[row - 1][column - 1] == 'M' && chars[row - 2][column - 2] == 'A' && chars[row - 3][column - 3] == 'S' { result += 1;  }

    if row > 2 && column < SIZE - 3 && chars[row - 1][column + 1] == 'M' && chars[row - 2][column + 2] == 'A' && chars[row - 3][column + 3] == 'S' { result += 1; }

    if row < SIZE - 3 && column > 2 && chars[row + 1][column - 1] == 'M' && chars[row + 2][column - 2] == 'A' && chars[row + 3][column - 3] == 'S' { result += 1; }

    if row < SIZE - 3 && column < SIZE - 3 && chars[row + 1][column + 1] == 'M' && chars[row + 2][column + 2] == 'A' && chars[row + 3][column + 3] == 'S' { result += 1; }

    return result;
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