use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const DIRECTIONS: [(char, (i32, i32)); 4] = [
    ('<', (0, -1)),
    ('>', (0, 1)),
    ('^', (-1, 0)),
    ('v', (1, 0)),
];

const fn get_direction(c: &char) -> Option<(i32, i32)> {
    let mut i = 0;
    while i < DIRECTIONS.len() {
        if DIRECTIONS[i].0 == *c {
            return Some(DIRECTIONS[i].1);
        }
        i += 1;
    }
    None
}

fn main() {
    let mut input = load_data(&false).unwrap();
    let mut input_part_2 = construct_wide_matrix(&input.1);

    part_1(input.0, &mut input.1, &input.2);
    part_2(input_part_2.0, &mut input_part_2.1, input.2);
}

fn part_2(mut position: (i32, i32), warehouse: &mut Vec<Vec<char>>, instructions: Vec<char>) {

    for instruction in instructions {
        let direction = get_direction(&instruction).unwrap();

        match warehouse[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] {
            '.' => {
                warehouse[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] = '@';
                warehouse[position.0 as usize][position.1 as usize] = '.';
                position = (position.0 + direction.0, position.1 + direction.1);
            }
            '#' => continue,
            _ => {
                match instruction {
                    '^' => {
                        if let Some(push_vector) = calculate_push_set(&position, &direction, &warehouse) {
                            let push_set: BTreeSet<(usize, usize)> = push_vector.into_iter().collect();

                            for tuple in push_set {
                                warehouse[tuple.0 - 1][tuple.1] = warehouse[tuple.0][tuple.1];
                                warehouse[tuple.0][tuple.1] = '.';
                            }

                            warehouse[(position.0 - 1) as usize][position.1 as usize] = '@';
                            warehouse[position.0 as usize][position.1 as usize] = '.';
                            position.0 -= 1;
                        }
                    }
                    'v' => {
                        if let Some(push_vector) = calculate_push_set(&position, &direction, &warehouse) {
                            let push_set: BTreeSet<_> = push_vector.into_iter().map(|(a, b)| Reverse((a, b))).collect();

                            for Reverse((x, y)) in push_set {
                                warehouse[x + 1][y] = warehouse[x][y];
                                warehouse[x][y] = '.';
                            }
                            warehouse[(position.0 + 1) as usize][position.1 as usize] = '@';
                            warehouse[position.0 as usize][position.1 as usize] = '.';
                            position.0 += 1;
                        }
                    }
                    _ => push_horizontally(&mut position, &direction, warehouse)
                }
            }  
        }

    }

    let mut result = 0;

    for i in 0..warehouse.len(){
        for j in 0..warehouse[i].len(){
            if warehouse[i][j] == '[' {
                result += i * 100 + j;
            }
        }
    }

    println!("part 2 {}", result);
}

//My Own
// fn calculate_push_set(position: &(i32, i32), direction: &(i32, i32), warehouse: &Vec<Vec<char>>) -> Option<Vec<(usize, usize)>> {
//     let mut push_set = vec![];

//     let x = (position.0 + direction.0) as usize;
//     let y = position.1 as usize;

//     if warehouse[x][y] == '#' {
//         return None;
//     }

//     if warehouse[x][y] == '.' {
//         return Some(push_set);
//     }

//     if warehouse[x][y] == ']' {
//         match (calculate_push_set(&(x as i32, y as i32), direction, warehouse), calculate_push_set(&(x as i32, (y - 1) as i32), direction, warehouse)) {
//             (Some(push_set1), Some(push_set2)) => {
//                 push_set.push((x, y));
//                 push_set.push((x, y - 1));
//                 push_set.extend(push_set1);
//                 push_set.extend(push_set2);
//             }
//             _ => return None
//         }
//     }

//     if warehouse[x][y] == '[' {
//         match (calculate_push_set(&(x as i32, y as i32), direction, warehouse), calculate_push_set(&(x as i32, (y + 1) as i32), direction, warehouse)) {
//             (Some(push_set1), Some(push_set2)) => {
//                 push_set.push((x, y));
//                 push_set.push((x, y + 1));
//                 push_set.extend(push_set1);
//                 push_set.extend(push_set2);
//             }
//             _ => return None
//         }
//     }

//     return Some(push_set);
// }

// Chat gpt shortened version
fn calculate_push_set(position: &(i32, i32), direction: &(i32, i32), warehouse: &Vec<Vec<char>>) -> Option<Vec<(usize, usize)>> {
    let mut push_set = vec![];

    let x = (position.0 + direction.0) as usize;
    let y = position.1 as usize;

    if warehouse[x][y] == '#' {
        return None;
    }

    if warehouse[x][y] == '.' {
        return Some(push_set);
    }

    let push_result = match warehouse[x][y] {
        ']' => (calculate_push_set(&(x as i32, y as i32), direction, warehouse),
                calculate_push_set(&(x as i32, (y - 1) as i32), direction, warehouse)),
        '[' => (calculate_push_set(&(x as i32, y as i32), direction, warehouse),
                calculate_push_set(&(x as i32, (y + 1) as i32), direction, warehouse)),
        _ => return Some(vec![(x, y)]), // If it's anything else, just return the current position
    };

    if let (Some(push_set1), Some(push_set2)) = push_result {
        push_set.push((x, y));
        push_set.extend(push_set1);
        push_set.push((x, if warehouse[x][y] == ']' { y - 1 } else { y + 1 }));
        push_set.extend(push_set2);
        Some(push_set)
    } else {
        None
    }
}

fn push_horizontally(position: &mut (i32, i32), direction: &(i32, i32), warehouse: &mut Vec<Vec<char>>) {

    let mut lenght_of_push = 0;

    loop {
        let x = (position.0 + direction.0 * lenght_of_push) as usize;
        let y = (position.1 + direction.1 * lenght_of_push) as usize;

        if warehouse[x][y] == '#'{
            return;
        } 

        if warehouse[x][y] == '.'{
            break;
        } 

        lenght_of_push += 1; 
    }

    for i in (1..=lenght_of_push).rev() {
        let x = (position.0 + direction.0 * i) as usize;
        let y = (position.1 + direction.1 * i) as usize;

        let prev_x = (position.0 + direction.0 * (i - 1)) as usize;
        let prev_y = (position.1 + direction.1 * (i - 1)) as usize;

        warehouse[x][y] = warehouse[prev_x][prev_y];
    }

    warehouse[position.0 as usize][position.1 as usize] = '.';
    position.0 = position.0 + direction.0;
    position.1 = position.1 + direction.1;
}

fn part_1(mut position: (i32, i32), warehouse: &mut Vec<Vec<char>>, instructions: &Vec<char>) {

    for instruction in instructions {
        let direction = get_direction(instruction).unwrap();

        match warehouse[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] {
            '.' => {
                warehouse[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] = '@';
                warehouse[position.0 as usize][position.1 as usize] = '.';
                position = (position.0 + direction.0, position.1 + direction.1);
            }
            'O' => {
                if let Some(lenght_of_push) = calculate_lenght_of_push(position, direction, warehouse) {
                    push(&position, &direction, lenght_of_push, warehouse);
                    position = (position.0 + direction.0, position.1 + direction.1);
                }
            }
            _ => continue,
        }
    }

    let mut result = 0;

    for i in 0..warehouse.len(){
        for j in 0..warehouse[i].len(){
            if warehouse[i][j] == 'O' {
                result += i * 100 + j;
            }
        }
    }

    println!("part 1: {}", result);
}

fn calculate_lenght_of_push(position: (i32, i32), direction: (i32, i32), warehouse: &Vec<Vec<char>>) -> Option<i32> {

    let mut lenght_of_push = 0;

    loop {
        if warehouse[(position.0 + direction.0 * lenght_of_push) as usize][(position.1 + direction.1 * lenght_of_push) as usize] == '#'{
            return None;
        } 

        if warehouse[(position.0 + direction.0 * lenght_of_push) as usize][(position.1 + direction.1 * lenght_of_push) as usize] == '.'{
            return Some(lenght_of_push);
        } 

        lenght_of_push += 1; 
    }
}

fn push(position: &(i32, i32), direction: &(i32, i32), lenght_of_push: i32, warehouse: &mut Vec<Vec<char>>) {
    warehouse[position.0 as usize][position.1 as usize] = '.';
    warehouse[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] = '@';
    warehouse[(position.0 + direction.0 * lenght_of_push) as usize][(position.1 + direction.1 * lenght_of_push) as usize] = 'O';
}

fn construct_wide_matrix(warehouse: &Vec<Vec<char>>) -> ((i32, i32), Vec<Vec<char>>){
    let mut wide_warehouse = Vec::with_capacity(warehouse.len());
    let mut start_position = (0, 0);

    for i in 0..warehouse.len() {
        let mut lane = Vec::new();

        for j in 0..warehouse.len() {
            match warehouse[i][j]{
                '#' => lane.extend(['#','#']),
                'O' => lane.extend(['[',']']),             
                '.' => lane.extend(['.', '.']),
                _ => {
                    start_position = (i as i32, (j * 2) as i32);
                    lane.extend(['@', '.']); 
                }
            }
        }

        wide_warehouse.push(lane);
    }

    (start_position, wide_warehouse)
}

fn load_data(is_test: &bool) -> io::Result<((i32, i32), Vec<Vec<char>>, Vec<char>)> {
    let mut path_string = "input.txt";
    let mut dimensions = 50;

    if *is_test {
        path_string = "test.txt";
        dimensions = 10;
    }

    let path = Path::new(path_string);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut matrix = Vec::with_capacity(dimensions);
    let mut instructions = Vec::new();
    let mut robot_position = (0, 0);

    for (row_index, line) in reader.lines().enumerate() {
        let line = line?;
        
        if line.contains('#') {
            let mut string_vector = Vec::with_capacity(line.len());

            for (col_index, ch) in line.chars().enumerate() {
                if ch == '@' {
                    robot_position = (row_index as i32, col_index as i32);
                }
                string_vector.push(ch);
            }

            matrix.push(string_vector);
        }

        if line.contains('<') {
            instructions.extend(line.chars());
        }
    }

    Ok((robot_position, matrix, instructions))
}