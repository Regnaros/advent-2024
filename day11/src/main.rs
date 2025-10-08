use std::{collections::HashMap, fs, io, vec};

const MAX: usize = 75;

fn main() {
    let input = load_data("input.txt").unwrap();

    part_1(&input);
    part_2(input);
}

fn load_data(file_path: &str) -> io::Result<Vec<String>> {
  return Ok(fs::read_to_string(file_path)?.trim().split(' ').map(String::from).collect());
}

fn part_1(input: &Vec<String>) {
    
    let mut result = input.clone();

    for _ in 0..25 {
        let mut temp_vec = vec![];
        
        for stone in &result {
            if stone == "0" {
                temp_vec.push("1".to_string());
            } else if stone.len() % 2 == 0 {
                
                temp_vec.push(stone[..(stone.len() / 2)].to_string());

                let second_half = stone[(stone.len() / 2)..].trim_start_matches('0');
                temp_vec.push(if second_half.is_empty() { "0" } else { second_half }.to_string());
            }
            else {
                let stone_number: u64 = stone.parse().unwrap();  
                temp_vec.push((stone_number * 2024).to_string());

            }
        }

        result = temp_vec;
    }

    println!("result {:?}", result.len());
}

fn part_2(input: Vec<String>) {
    
    let mut result = 0;

    let mut dynamic_map: HashMap<String, [i128; MAX]> = HashMap::new();

    for stone in input {
        result += calculate_stones_dynamically(&stone, 0, &mut dynamic_map);
    }

    println!("result {:?}", result);
}

fn calculate_stones_dynamically(stone: &str, blinks: usize, dynamic_map: &mut HashMap<String, [i128; MAX]>) -> i128 {

    if blinks == MAX {
        return 1;
    }

    let stone_key = stone.to_string();

    let result = {
        let stone_array = dynamic_map.entry(stone_key.clone()).or_insert([-1; MAX]);

        if stone_array[blinks] != -1 {
            return stone_array[blinks];
        }

        if stone == "0" {
            calculate_stones_dynamically("1", blinks + 1, dynamic_map)
        } else if stone.len() % 2 == 0 {
            let second_half = stone[(stone.len() / 2)..].trim_start_matches('0');
            calculate_stones_dynamically(&stone[..(stone.len() / 2)], blinks + 1, dynamic_map)
                + calculate_stones_dynamically( if second_half.is_empty() { "0" } else { second_half }, blinks + 1, dynamic_map,
                )
        } else {
            let stone_number: u64 = stone.parse().unwrap();
            calculate_stones_dynamically(&(stone_number * 2024).to_string(), blinks + 1, dynamic_map)
        }
    };

    let stone_array = dynamic_map.get_mut(&stone_key).unwrap();
    stone_array[blinks] = result;

    result
}


// fn calculate_stones_dynamically2(stone: &str, blinks: usize, dynamic_map: &mut HashMap<&str, [i128; 75]>) -> i128 {

//     if blinks == 75 {
//         return 1;
//     }

//     match dynamic_map.entry(stone) {
//         std::collections::hash_map::Entry::Occupied(mut entry) => {
//             let stone_array = entry.get_mut();

//             if stone_array[blinks] != -1 {
//                 return stone_array[blinks];
//             } else {
//                 if stone == "0" {
//                     stone_array[blinks] = calculate_stones_dynamically("1", blinks + 1, dynamic_map)
//                 }
//                 else if stone.len() % 2 == 0 {
//                     let second_half = stone[(stone.len() / 2)..].trim_start_matches('0');

//                     stone_array[blinks] = calculate_stones_dynamically(&stone[..(stone.len() / 2)], blinks + 1, dynamic_map) +
//                         calculate_stones_dynamically(if second_half.is_empty() { "0" } else { second_half }, blinks + 1, dynamic_map)
//                 }
//                 else {
//                     let stone_number: u64 = stone.parse().unwrap();  
//                     stone_array[blinks] = calculate_stones_dynamically(&(stone_number * 2024).to_string(), blinks + 1, dynamic_map);
//                 }
//             }
//         },
//         std::collections::hash_map::Entry::Vacant(entry) => {
//             entry.insert([-1; 75]);
//             dynamic_map[stone][blinks] = calculate_stones_dynamically(stone, blinks, dynamic_map);
//         }
//     }


//     return dynamic_map[stone][blinks];
// }
