use std::{collections::{HashMap, VecDeque}, fs::File, io::{self, BufRead, Result}};

fn main() {
    let numbers = parse_data().unwrap();

    part_1(numbers.clone());
    part_2(numbers);
}

fn part_2(numbers: Vec<u64>) {
    let mut collective_prices:HashMap<VecDeque<i8>, u16> = HashMap::new();
    
    for number in numbers {
        let mut prices: HashMap<VecDeque<i8>, i8> = HashMap::new();
        let mut queue: VecDeque<i8> = VecDeque::from(vec![]);

        let mut secret_number = number;
        queue.push_back((secret_number % 10) as i8);

        let mut previous_price = (secret_number % 10) as i8;
        for _ in 1..3 {
            secret_number = calculate_next_number(secret_number);
            queue.push_back((secret_number % 10) as i8 - previous_price);
            previous_price = (secret_number % 10) as i8;
        }
    
        for _ in 3..2000 {
            secret_number = calculate_next_number(secret_number);
            let price = (secret_number % 10) as i8;
            queue.push_back(price - previous_price);
            previous_price = price;

            if !prices.contains_key(&queue.clone()) {
                prices.insert(queue.clone(), price);
            }

            queue.pop_front();
        }

        for (key, value)  in prices {
            let value = value as u16;
            
            collective_prices.entry(key)
                .and_modify( |val| *val += value)
                .or_insert(value);
        }
    }

    let mut max = 0;
    for max_price in collective_prices.values() {
        if *max_price > max {
            max = *max_price;
        }
    }

    println!("{}", max);
}

fn part_1(numbers: Vec<u64>){
    let mut result: u64 = 0;

    for number in numbers {
        let mut secret_number = number;

        for _ in 0..2000 {
            secret_number = calculate_next_number(secret_number);
        }

        result += secret_number;
    }

    println!("{}", result);
}

fn calculate_next_number(number: u64) -> u64 {

    let mut secret_number = ((number * 64) ^ number) % 16777216;
    secret_number = ((secret_number / 32) ^ secret_number) % 16777216;
    secret_number = ((secret_number * 2048) ^ secret_number) % 16777216;

    return secret_number;
}

fn parse_data() -> Result<Vec<u64>> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut start_numbers: Vec<u64> = vec![];

    for line in reader.lines() {
        let line = line?;
        start_numbers.push(line.parse().unwrap());
    }

    Ok(start_numbers)
}