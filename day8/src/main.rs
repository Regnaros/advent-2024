use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

const SIZE: usize = 50; // Example size, change as needed
const I32SIZE: i32 = SIZE as i32;

fn load_matrix_from_file(file_path: &str) -> Result<[[char; SIZE]; SIZE], io::Error> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Initialize an empty matrix
    let mut matrix = [[' '; SIZE]; SIZE];

    // Read lines from the file
    for (i, line) in reader.lines().enumerate() {
        if i >= SIZE {
            break; // Stop if we've filled the matrix
        }
        let line = line?;
        let chars: Vec<char> = line.chars().collect();

        // Fill the matrix row
        for (j, &ch) in chars.iter().take(SIZE).enumerate() {
            matrix[i][j] = ch;
        }
    }

    Ok(matrix)
}

fn main() {
    let file_path = "input.txt";

    let input = load_matrix_from_file(file_path).unwrap();

    let mut antennas:HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for line in 0..input.len() {
        for char in 0..input[line].len() {
            if input[line][char] != '.' {
                antennas.entry(input[line][char]).or_insert(Vec::new()).push((line as i32, char as i32));
            }
        }
    }

    part_1(antennas);
}

fn part_1(antennas: HashMap<char, Vec<(i32, i32)>>) {

    let mut result = vec![];

    for (_, antenna_positions) in &antennas {
        for current_antenna_index in 1..antenna_positions.len() {
            result.append(&mut evaluate_antinodes(antenna_positions[current_antenna_index-1], &antenna_positions[current_antenna_index..]));
        }
    }

    let resultset: HashSet<(i32, i32)> = result.into_iter().collect();
    println!("{}",resultset.len());
}

fn evaluate_antinodes(current_antenna: (i32, i32), remaining_antennas: &[(i32, i32)],) -> Vec<(i32, i32)> {

    if remaining_antennas.is_empty() {
        return Vec::new();
    }

    let mut antinodes: Vec<(i32, i32)> = Vec::new();

    let opposite_antenna = remaining_antennas[0];

    let dx = opposite_antenna.0 - current_antenna.0;
    let dy = opposite_antenna.1 - current_antenna.1;

    if opposite_antenna.0 + dx >= 0 && opposite_antenna.0 + dx < I32SIZE && opposite_antenna.1 + dy >= 0 && opposite_antenna.1 + dy < I32SIZE {
        antinodes.push((opposite_antenna.0 + dx, opposite_antenna.1 + dy));
    }
    if current_antenna.0 - dx >= 0 && current_antenna.0 - dx < I32SIZE && current_antenna.1 - dy >= 0 && current_antenna.1 - dy < I32SIZE {
        antinodes.push((current_antenna.0 - dx, current_antenna.1 - dy));
    }

    antinodes.extend(evaluate_antinodes(current_antenna, &remaining_antennas[1..]));

    return antinodes;
}