use std::{collections::{HashMap, HashSet, VecDeque}, fs::File, io::{self, BufRead, BufReader}};

fn main() {
    let data = parse_data("input.txt").unwrap(); 
    find_correct_values(&data);
    part_2(data);
}

fn part_2(input: WireStructure) {
    // let input_for_part1 = input.clone();
    
    let current_binary = "1011110000111111111110101101011110010001110000".to_string();
    let correct_binary = "1011111001000010000110101110011110010001110000".to_string();

    let wire_dependencies = parse_reverse_wire_values("input.txt").unwrap();

    for i in 0..current_binary.len() {
        let current = current_binary.chars().nth(i).unwrap();
        let correct = correct_binary.chars().nth(i).unwrap();
        let zbit =  if i > 9 { format!("z{}", i) } else { format!("z0{}", i) };

        if current != correct {
            let mut dependency_set = find_dependencies(&wire_dependencies, zbit.clone());
            dependency_set.remove(&zbit);

            println!("{}", zbit);
            println!("{:?}", dependency_set);
        }
    }

}

fn find_dependencies(wire_values: &HashMap<String, HashSet<String>>, bit: String) -> HashSet<String> {
    
    let mut result = HashSet::new();
    result.insert(bit.clone());

    match wire_values.get(&bit) {
        None => {
            return result;
        },
        Some(dependencies) => {
            for dependency in dependencies {
                result.extend(find_dependencies(&wire_values, dependency.to_string()));
            }
        }
    }

    return result
}

fn part_1(input: WireStructure) {
    let mut wire_values = input.0;
    let remaining_operations = input.1;
    let mut ready_operations = input.2;

    while !ready_operations.is_empty() {
        let next_operation = ready_operations.pop_front().unwrap();
        let inputs = remaining_operations.get(&next_operation).unwrap();

        let left_input = wire_values.get(&inputs.0.0).unwrap().0.unwrap();
        let right_input = wire_values.get(&inputs.0.1).unwrap().0.unwrap();
        let action = inputs.1.clone();

        let result: u8 = match action.as_str() {
            "AND" => if left_input == 1 && right_input == 1 { 1 } else { 0 },
            "OR" => if left_input == 1 || right_input == 1 { 1 } else { 0 },
            _ => if left_input != right_input { 1 } else { 0 },
        };

        let wire_value = wire_values.get_mut(&next_operation).unwrap();
        wire_value.0 = Some(result);

        for dependency in wire_value.1.clone() {
            let operation_inputs = remaining_operations.get(&dependency).unwrap();
            let left_input = operation_inputs.0.0.clone();
            let right_input = operation_inputs.0.1.clone();

            if let (Some(left), Some(right)) = (wire_values.get(&left_input), wire_values.get(&right_input)) {
                if left.0 != None && right.0 != None {
                    ready_operations.push_back(dependency);
                }
            }
        }
    }

    let mut output_vector = vec![];
    for kvpair in &wire_values {
        if kvpair.0.contains("z") {
            output_vector.push(kvpair.0);
        }
    }

    output_vector.sort();

    let mut binary_string: String = "".to_owned();
    
    for key in output_vector.into_iter().rev() {
        if let Some(value) = wire_values.get(key) {
            // println!("{}: {:?}", key, value.0.unwrap());
            binary_string.push_str(&value.0.unwrap().to_string());
        }
    }

    println!("{}", binary_string);
    let decimal_value = u64::from_str_radix(&binary_string, 2).unwrap();
    println!("Decimal Value: {}", decimal_value);
    // println!("{:?}", wire_values);
}

type WireStructure = (HashMap<String, (Option<u8>, Vec<String>)>, HashMap<String, ((String, String), String)>, VecDeque<String>);

fn parse_reverse_wire_values(path: &str) -> io::Result<HashMap<String, HashSet<String>>> {

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut in_first_section = true;

    let mut wire_dependencies = HashMap::new();
    

    for line in reader.lines() {
        let line = line?;

        if line.trim().is_empty() {
            in_first_section = false;
            continue;
        }

        if in_first_section {
            continue;
        }

        let operation: Vec<&str> = line.split_whitespace().collect();
        let first_input = operation[0].to_string();
        let second_input = operation[2].to_string();
        let output = operation[4].to_string();

        let entry = wire_dependencies.entry(output).or_insert(HashSet::new());
        entry.insert(first_input);
        entry.insert(second_input);
    }

    Ok(wire_dependencies)
}

fn find_correct_values(input: &WireStructure) -> String {  
    let wire_values = &input.0;

    let mut x_binary = String::from("");
    let mut y_binary = String::from("");

    let mut x_vector = Vec::new();
    let mut y_vector = Vec::new();

    for key_value in wire_values {
        if key_value.0.contains('x') {
            x_vector.push(key_value.0);
        }
        if key_value.0.contains('y') {
            y_vector.push(key_value.0);
        }
    }

    x_vector.sort();
    y_vector.sort();

    for key in x_vector.into_iter().rev() {
        if let Some(value) = wire_values.get(key) {
            // println!("{}: {:?}", key, value.0.unwrap());
            x_binary.push_str(&value.0.unwrap().to_string());
        }
    }

    for key in y_vector.into_iter().rev() {
        if let Some(value) = wire_values.get(key) {
            // println!("{}: {:?}", key, value.0.unwrap());
            y_binary.push_str(&value.0.unwrap().to_string());
        }
    }

    println!("x binary: {}", x_binary);
    println!("y binary: {}", y_binary);
    let x_decimal_value = u64::from_str_radix(&x_binary, 2).unwrap();
    let y_decimal_value = u64::from_str_radix(&y_binary, 2).unwrap();
    println!("x {}, y {}, Total value Value: {}",x_decimal_value, y_decimal_value, x_decimal_value + y_decimal_value);
    let total_binary = format!("{:b}", x_decimal_value + y_decimal_value);
    println!("Total binary: {}", total_binary);
    total_binary
}

fn parse_data(path: &str) -> io::Result<WireStructure> {

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut in_first_section = true;
    let mut wire_values: HashMap<String, (Option<u8>, Vec<String>)> = HashMap::new();
    let mut operations: HashMap<String, ((String, String), String)> = HashMap::new();
    let mut ready_operations: VecDeque<String> = VecDeque::from(vec![]);
    

    for line in reader.lines() {
        let line = line?;

        if line.trim().is_empty() {
            in_first_section = false;
            continue;
        }

        if in_first_section {
            let wire_and_value: Vec<&str> = line.trim().split(": ").collect();
            wire_values.insert(wire_and_value[0].to_string(), (Some(wire_and_value[1].parse::<u8>().unwrap()), vec![]));

        } else {
            let operation: Vec<&str> = line.split_whitespace().collect();
            let first_input = operation[0].to_string();
            let action = operation[1].to_string();
            let second_input = operation[2].to_string();
            let output = operation[4].to_string();

            if let (Some((Some(_), _)), Some((Some(_), _))) = (wire_values.get(&first_input), wire_values.get(&second_input)) {
                ready_operations.push_back(output.clone());
            }
            
            match wire_values.get_mut(&first_input){
                Some((_, vec)) => vec.push(output.clone()),
                None => {
                    wire_values.insert(first_input.clone(), (None, vec![output.clone()])); 
                }
            }

            match wire_values.get_mut(&second_input){
                Some((_, vec)) => vec.push(output.clone()),
                None => {
                    wire_values.insert(second_input.clone(), (None, vec![output.clone()])); 
                }
            }

            if !wire_values.contains_key(&output) {
                wire_values.insert(output.clone(), (None, vec![]));
            }

            operations.insert(output, ((first_input, second_input), action));
        }
    }

    // println!("{:?}", wire_values);
    // println!("{:?}", ready_operations);
    // println!("{:?}", operations);

    Ok((wire_values, operations, ready_operations))
}