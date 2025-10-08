use std::{collections::{HashMap, HashSet}, fs::File, io::{self, BufRead, BufReader}};

fn main() {
    let input = parse_data("input.txt").unwrap();
    
    let mut start_position = (0, 0);

    for i in 0..input.len(){
        for j in 0..input[0].len() {
            if input[i][j] == 'S' {
                start_position = (i , j);
            }
        }
    }

    let path = construct_path(&input, start_position);
    let res_part1 = part_1(&input, start_position, &path);
    let res_part2 = part_2(&input, start_position, path);
    println!("{}", res_part1);
    println!("{}", res_part2);
}

fn construct_path(grid: &Vec<Vec<char>>, start_position: (usize, usize)) -> HashMap<(usize, usize), i32>{

    let mut closed_set = HashSet::new();
    let mut path = HashMap::new();
    let max =  grid.len() - 2;

    let mut current_position: (usize, usize) = start_position;
    let mut steps = 0;
    path.insert(current_position, steps);

    closed_set.insert(current_position);
    while grid[current_position.0][current_position.1] != 'E' {

        if current_position.0 > 0 && grid[current_position.0 - 1][current_position.1] != '#' && closed_set.insert((current_position.0 - 1, current_position.1)) {
            current_position.0 -= 1;
            steps += 1;
            path.insert(current_position, steps);
            continue;
        }

        if current_position.0 < max && grid[current_position.0 + 1][current_position.1] != '#' && closed_set.insert((current_position.0 + 1, current_position.1)) {
            current_position.0 += 1;
            steps += 1;
            path.insert(current_position, steps);
            continue;
        }
        if current_position.1 > 0 && grid[current_position.0][current_position.1 - 1] != '#' && closed_set.insert((current_position.0, current_position.1 - 1)) {
            current_position.1 -= 1;
            steps += 1;
            path.insert(current_position, steps);
            continue;
        }

        if current_position.1 < max && grid[current_position.0][current_position.1 + 1] != '#' && closed_set.insert((current_position.0, current_position.1 + 1)) {
            current_position.1 += 1;
            steps += 1;
            path.insert(current_position, steps);
            continue;
        }
    }

    path
}

fn part_2(grid: &Vec<Vec<char>>, start_position: (usize, usize), path: HashMap<(usize, usize), i32>) -> i64 {

    let mut closed_set = HashSet::new();
    let max =  grid.len() - 2;

    let mut current_position: (usize, usize) = start_position;
    let mut steps = 0;
    let mut cheatpaths = 0;
    cheatpaths += reachable_coordinates(&path, steps, grid, &current_position);

    closed_set.insert(current_position);
    while grid[current_position.0][current_position.1] != 'E' {

        if current_position.0 > 0 && grid[current_position.0 - 1][current_position.1] != '#' && closed_set.insert((current_position.0 - 1, current_position.1)) {
            current_position.0 -= 1;
            steps += 1;
            cheatpaths += reachable_coordinates(&path, steps, grid, &current_position);
            continue;
        }

        if current_position.0 < max && grid[current_position.0 + 1][current_position.1] != '#' && closed_set.insert((current_position.0 + 1, current_position.1)) {
            current_position.0 += 1;
            steps += 1;
            cheatpaths += reachable_coordinates(&path, steps, grid, &current_position);
            continue;
        }
        if current_position.1 > 0 && grid[current_position.0][current_position.1 - 1] != '#' && closed_set.insert((current_position.0, current_position.1 - 1)) {
            current_position.1 -= 1;
            steps += 1;
            cheatpaths += reachable_coordinates(&path, steps, grid, &current_position);
            continue;
        }

        if current_position.1 < max && grid[current_position.0][current_position.1 + 1] != '#' && closed_set.insert((current_position.0, current_position.1 + 1)) {
            current_position.1 += 1;
            steps += 1;
            cheatpaths += reachable_coordinates(&path, steps, grid, &current_position);
            continue;
        }
    }

    cheatpaths
}

fn reachable_coordinates(
    path: &HashMap<(usize, usize), i32>,
    steps: i32,
    grid: &Vec<Vec<char>>,
    start: &(usize, usize)
) -> i64 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let temp_start = (start.0 as i32, start.1 as i32);
    let max_steps: i32 = 20;
    let mut cheating_paths = 0;
    let threshold = 100;

    for i in -max_steps..=max_steps {
        for j in -(max_steps - i.abs())..=(max_steps - i.abs()) {

            if temp_start.0 + i >= 0 && temp_start.0 + i < rows && 
                temp_start.1 + j >= 0 && temp_start.1 + j < cols &&
                 grid[(temp_start.0 + i) as usize][(temp_start.1 + j) as usize] != '#' {

                let potential_cheat = ((temp_start.0 + i) as usize, (temp_start.1 + j) as usize);

                if let Some(normal_steps) = path.get(&potential_cheat) {
                    if normal_steps - ((start.0.abs_diff(potential_cheat.0) + start.1.abs_diff(potential_cheat.1)) as i32) - steps >= threshold {
        
                        cheating_paths += 1;
                    }
                }    
            }
        }
    }

    cheating_paths
}

fn parse_data(path: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for (_, line) in reader.lines().enumerate() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();

        data.push(chars);
    }

    Ok(data)
}

fn part_1(grid: &Vec<Vec<char>>, start_position: (usize, usize), path: &HashMap<(usize, usize), i32>) -> i32 {

    let mut closed_set = HashSet::new();
    let max =  grid.len() - 2;

    let mut current_position: (usize, usize) = start_position;
    let mut steps = 0;
    let mut cheatpaths = 0;

    closed_set.insert(current_position);
    while grid[current_position.0][current_position.1] != 'E' {

        if current_position.0 > 0 && grid[current_position.0 - 1][current_position.1] != '#' && closed_set.insert((current_position.0 - 1, current_position.1)) {
            current_position.0 -= 1;
            steps += 1;
            
            cheatpaths += find_cheats(&path, grid, &current_position, steps, &max);
            continue;
        }

        if current_position.0 < max && grid[current_position.0 + 1][current_position.1] != '#' && closed_set.insert((current_position.0 + 1, current_position.1)) {
            current_position.0 += 1;
            steps += 1;
            cheatpaths += find_cheats(&path, grid, &current_position, steps, &max);
            continue;
        }
        if current_position.1 > 0 && grid[current_position.0][current_position.1 - 1] != '#' && closed_set.insert((current_position.0, current_position.1 - 1)) {
            current_position.1 -= 1;
            steps += 1;
            cheatpaths += find_cheats(&path, grid, &current_position, steps, &max);
            continue;
        }

        if current_position.1 < max && grid[current_position.0][current_position.1 + 1] != '#' && closed_set.insert((current_position.0, current_position.1 + 1)) {
            current_position.1 += 1;
            steps += 1;
            cheatpaths += find_cheats(&path, grid, &current_position, steps, &max);
            continue;
        }
    }

    cheatpaths
}

fn find_cheats(path: &HashMap<(usize, usize), i32>, grid: &Vec<Vec<char>>, position: &(usize, usize), steps: i32 , max: &usize) -> i32 {
    let mut result = 0;
    let threshold = 100;

    if position.0 > 1 && grid[position.0 - 1][position.1] == '#' {
        if let Some(number_of_steps) = path.get(&(position.0 - 2, position.1)) 
        {
            if number_of_steps - steps - 2 >= threshold
            {
                result += 1;
            }
        }
    }
    if position.0 < max - 1 && grid[position.0 + 1][position.1] == '#' {
        if let Some(number_of_steps) = path.get(&(position.0 + 2, position.1)) 
        {
            if number_of_steps - steps - 2 >= threshold
            {
                result += 1;
            }
        }
    }
    if position.1 > 1 && grid[position.0][position.1 - 1] == '#' {
        if let Some(number_of_steps) = path.get(&(position.0, position.1 - 2)) 
        {
            if number_of_steps - steps - 2 >= threshold
            {
                result += 1;
            }
        }
    }
    if position.0 > 1 && grid[position.0][position.1 + 1] == '#' {
        if let Some(number_of_steps) = path.get(&(position.0, position.1 + 2)) 
        {
            if number_of_steps - steps - 2 >= threshold
            {
                result += 1;
            }
        }
    }

    result
}
