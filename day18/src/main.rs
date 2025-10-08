use std::{collections::{BTreeSet, HashSet}, fs::File, io::{self, BufRead}};

fn main() {
    let test = false;

    if test {
        let coordinates = parse_data("test.txt").unwrap();

        let mut grid = Vec::new();
        for _ in 0..7 {
            grid.push(vec!['.'; 7]);
        }

        for i in 0..12 {
            grid[coordinates[i].0][coordinates[i].1] = '#';
        }

        let result_part1 = part_1(&grid, 6).unwrap();
        println!("{}", result_part1);

        part_2(&mut grid, coordinates[12..].to_vec(), 6);
    } else {
        let coordinates = parse_data("input.txt").unwrap();

        let mut grid = Vec::new();
        for _ in 0..71 {
            grid.push(vec!['.'; 71]);
        }

        for i in 0..1024 {
            grid[coordinates[i].0][coordinates[i].1] = '#';
        }

        let result_part1 = part_1(&grid, 70).unwrap();
        println!("{}", result_part1);

        part_2(&mut grid, coordinates[12..].to_vec(), 70);
    }
}

fn parse_data(file_path: &str) -> io::Result<Vec<(usize, usize)>> {
    let path = file_path;

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut coordinates: Vec<(usize, usize)> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() == 2 {
            if let (Ok(x), Ok(y)) = (parts[0].trim().parse(), parts[1].trim().parse()) {
                coordinates.push((y, x));
            }
        }
    }

    Ok(coordinates)
}

fn part_2(grid: &mut Vec<Vec<char>>, remaining_coordinates: Vec<(usize, usize)>, max: usize) {
    for coordinate in remaining_coordinates {
        grid[coordinate.0][coordinate.1] = '#';

        if let None = part_1(grid, max) {
            println!("{:?}", (coordinate.1, coordinate.0));
            return;
        }
    } 

    println!("Error");
}

fn part_1(grid: &Vec<Vec<char>>, max: usize) -> Option<usize> {

    let mut open_set: BTreeSet<(usize, (usize, usize))> = BTreeSet::new();
    let mut closed_set = HashSet::new();

    let mut current_position: (usize, usize) = (0, 0);
    let mut steps = 0;

    closed_set.insert(current_position);
    while current_position != (max, max) {

        if current_position.0 > 0 && grid[current_position.0 - 1][current_position.1] != '#' && closed_set.insert((current_position.0 - 1, current_position.1)) {
            open_set.insert((steps + 1, (current_position.0 - 1, current_position.1)));
        }

        if current_position.0 < max && grid[current_position.0 + 1][current_position.1] != '#' && closed_set.insert((current_position.0 + 1, current_position.1)) {
            open_set.insert((steps + 1, (current_position.0 + 1, current_position.1)));
        }

        if current_position.1 > 0 && grid[current_position.0][current_position.1 - 1] != '#' && closed_set.insert((current_position.0, current_position.1 - 1)) {
            open_set.insert((steps + 1, (current_position.0, current_position.1 - 1)));
        }

        if current_position.1 < max && grid[current_position.0][current_position.1 + 1] != '#' && closed_set.insert((current_position.0, current_position.1 + 1)) {
            open_set.insert((steps + 1, (current_position.0, current_position.1 + 1)));
        }

        if let Some(next_position) = open_set.pop_first() {
            steps = next_position.0;
            current_position = next_position.1;
        } else {

            return None;
        }
    }

    return Some(steps);
}
