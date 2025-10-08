use std::{collections::{HashMap, HashSet}, usize};
use itertools::Itertools;
const STEPS: usize = 26;

fn main() {
    let numpad = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['#', '0', 'A'],
    ];  

    let look_up = find_best_path_robot_pad();
    let mut stored_results: HashMap<(char, Vec<char>), [usize; STEPS]> = HashMap::new();
    let mut result = 0;
    let mut result_part1 = 0;

    for number in vec![("805A", 805), ("682A", 682), ("671A", 671), ("973A", 973), ("319A", 319)] {
        let mut combinations = Vec::new();
        let mut start = 'A';
    
        for sequence in number.0.chars() {
            let paths = find_all_routes(&numpad, &start, &sequence);
    
            if combinations.is_empty() {
                combinations = paths;
            }
            else {
                combinations = combine_vectors(combinations, paths);
            }
    
            start = sequence;
        }

        let mut min = usize::MAX;
        let mut min_part1 = usize::MAX;
        for combination in combinations {
            let part_two_copy = combination.clone();
            let lenght_part1 = calculate_sequence_lenght(&look_up, & mut stored_results, 2, 'A', combination);
            let lenght = calculate_sequence_lenght(&look_up, & mut stored_results, STEPS - 1, 'A', part_two_copy);

            if lenght < min {
                min = lenght;
            }
            if lenght_part1 < min_part1 {
                min_part1 = lenght_part1;
            }
        }
        
        result += min * number.1;
        result_part1 += min_part1 * number.1;
    }

    println!("part 1: {}", result_part1);
    println!("part 2: {}", result);
}

fn calculate_sequence_lenght(
    lookup: &HashMap<(char, char), Vec<Vec<char>>>, 
    stored_results: & mut HashMap<(char, Vec<char>), [usize; STEPS]>, 
    steps: usize, 
    start: char, 
    sequence: Vec<char>)
     -> usize {
        
    if let Some(results) = stored_results.get(&(start, sequence.clone())) {
        if results[steps] != 0 {

            return results[steps];
        }
    }

    if steps == 0 {
        let entry = stored_results.entry((start, sequence.clone())).or_insert([0; STEPS]); 
        entry[steps] = sequence.len();

        return sequence.len();
    }

    let mut result = 0;
    let mut start_symbol = start;
    for i in 0..sequence.len() {
        let mut min = usize::MAX;
        for path in lookup.get(&(start_symbol, sequence[i])).unwrap() {
            let lenght = calculate_sequence_lenght(lookup, stored_results, steps-1, 'A', path.to_vec());
            if lenght < min  {
                min = lenght;                
            }
        }

        result += min;
        start_symbol = sequence[i];
    }

    let entry = stored_results.entry((start, sequence.clone())).or_insert([0; STEPS]); 
    entry[steps] = result;

    result
}

fn find_position(pad: &Vec<Vec<char>>, symbol: &char) -> Option<(i8, i8)> {

    for i in 0..pad.len() {
        for j in 0..pad[0].len() {
            if pad[i][j] == *symbol {
                return Some((i as i8, j as i8));
            }
        }
    }

    None
}

fn find_best_path_robot_pad() -> HashMap<(char, char), Vec<Vec<char>>> {
    let robot_pad = vec![
        vec!['#', '^', 'A'],
        vec!['<', 'v', '>']
    ];

    let mut result = HashMap::new();

    let symbols = vec!['^', 'A', '<', 'v', '>'];
    for start in &symbols {
        for end in &symbols {
            result.insert((*start, *end), find_all_routes(&robot_pad, &start, &end));
        }
    }

    result
}

fn find_all_routes(pad: &Vec<Vec<char>>, start_position: &char, end_position: &char) -> Vec<Vec<char>> {
    let start_position = find_position(pad, start_position).unwrap();
    let end_position = find_position(pad, end_position).unwrap();
    let row_difference = start_position.0 - end_position.0;
    let col_difference = start_position.1 - end_position.1;

    let mut instructions = vec![];
    match row_difference.signum() {
        1 => instructions.extend(vec!['^'; row_difference as usize]),
        -1 => instructions.extend(vec!['v'; row_difference.abs() as usize]),
        _ => {}
    }

    match col_difference.signum() {
        1 => instructions.extend(vec!['<'; col_difference as usize]),
        -1 => instructions.extend(vec!['>'; col_difference.abs() as usize]),
        _ => {}
    }

    let number_of_permuations = instructions.len();
    let permutations: HashSet<Vec<char>> = instructions
        .into_iter()
        .permutations(number_of_permuations)
        .unique()
        .collect();

    let mut result = vec![];
    'outer: for mut perm in permutations {
        let mut temp_start = start_position;

        for instruction in &perm {
            match instruction {
                '^' => temp_start.0 -= 1,
                'v' => temp_start.0 += 1,
                '<' => temp_start.1 -= 1,
                _ => temp_start.1 += 1
            }

            if pad[temp_start.0 as usize][temp_start.1 as usize] == '#' {
                continue 'outer;
            }
        }

        perm.push('A');
        result.push(perm);
    }

    result
}

fn combine_vectors(vec1: Vec<Vec<char>>, vec2: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    for v1 in &vec1 {
        for v2 in &vec2 {
            let mut combined = v1.clone();
            combined.extend(v2.iter().cloned());
            result.push(combined);
        }
    }

    result
}