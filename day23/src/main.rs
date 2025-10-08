use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader, Result}};

fn main() {
    let data = parse_data("input.txt").unwrap();
    part_1(&data);
    part_2(data);
}

fn part_2(input: Vec<String>) {

    let mut computer_connections: HashMap<String, HashSet<String>> = HashMap::new();

    for connection in input {
        let computers: Vec<&str> = connection.split('-').collect();
        let entry = computer_connections.entry((&computers[0]).to_string()).or_insert(HashSet::new());
        entry.insert(computers[1].to_owned());

        let entry = computer_connections.entry((&computers[1]).to_string()).or_insert(HashSet::new());
        entry.insert(computers[0].to_owned());
    }

    let mut max = 0;
    let mut max_vector = vec![];
    for (computer, connections) in &computer_connections {
        let mut confirmed_connections = vec![computer];

        for connection in connections {
            
            let unverified_connections = computer_connections.get(connection).unwrap();
            let contains_all = confirmed_connections.iter().all(|item| unverified_connections.contains(*item));

            if contains_all {
                confirmed_connections.push(connection);
            }
        }

        if confirmed_connections.len() > max {
            max = confirmed_connections.len();
            max_vector = confirmed_connections;
        }
    }

    max_vector.sort();
    println!("{:?}", max_vector);
}

fn part_1(input: &Vec<String>) {

    let mut computer_connections: HashMap<String, HashSet<String>> = HashMap::new();

    for connection in input {
        let computers: Vec<&str> = connection.split('-').collect();
        let entry = computer_connections.entry((&computers[0]).to_string()).or_insert(HashSet::new());
        entry.insert(computers[1].to_owned());

        let entry = computer_connections.entry((&computers[1]).to_string()).or_insert(HashSet::new());
        entry.insert(computers[0].to_owned());
    }

    let mut interconnected_sets: HashSet<Vec<&String>> = HashSet::new();
    for (computer, connections) in &computer_connections {
        if computer.starts_with('t') {
            for connection in connections {
                for third_party in computer_connections.get(connection).unwrap() {
                    if connections.contains(third_party) {
                        let mut interconnected_set = vec![computer, connection, third_party];
                        interconnected_set.sort();
                        interconnected_sets.insert(interconnected_set);
                    }
                }
            }
        }
    }

    println!("{}", interconnected_sets.len());
}

fn parse_data(path: &str) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut data = vec![];
    for line in reader.lines() {
        let line = line?;
        data.push(line);
    }

    Ok(data)
}
