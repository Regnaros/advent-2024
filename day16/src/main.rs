use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    usize, vec,
};

fn main() {
    let input = load_data(false).unwrap();
    part_1_rework((input.len() - 2, 1), input);
}

fn load_data(is_test: bool) -> io::Result<Vec<Vec<char>>> {
    let mut path_string = "input.txt";

    if is_test {
        path_string = "test.txt"
    }

    let path = Path::new(path_string);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        matrix.push(line.chars().collect());
    }

    Ok(matrix)
}

fn part_1_rework(mut position: (usize, usize), maze: Vec<Vec<char>>) -> Option<usize> {
    
    let mut current_symbol = '>';
    let mut score = 0;
    let mut ongoing = true;
    let mut max = usize::MAX;

    let mut open_set: BTreeSet<(usize, (usize, usize), char)> = BTreeSet::new();
    let mut closed_set: HashMap<((usize, usize), char), usize> = HashMap::new();
    let mut visited_tiles: HashMap<(usize, (usize, usize), char), Vec<(usize, (usize, usize), char)>> = HashMap::new();

    let mut end_position = (0, 0);
    let start_position = position;

    while ongoing {

        if maze[position.0][position.1] == 'E' && score < max {
            max = score;
            end_position = position;
        }

        let mut add_move = | new_position: (usize, usize), cost: usize, symbol: char | {
            if score + cost > max { return }

            if maze[new_position.0][new_position.1] == '#' { return; }
        
            if cost == 1 {
        
                match visited_tiles.get_mut(&(score + cost, new_position, symbol)) {
                    Some(children) => {
                        children.push((score, position, symbol));
                        return;
                    }
                    None => visited_tiles.insert((score + cost, new_position, symbol), vec![(score, position, symbol)])
                };
        
                if closed_set.get(&(new_position, symbol)).map_or(true, |&current_cost| cost+score < current_cost)
                {
                    closed_set.insert((new_position, symbol), cost + score);
                    open_set.insert((score + cost, new_position, symbol));
                }
            } else {

                match visited_tiles.get_mut(&(score + cost, position, symbol)) {
                    Some(children) => {
                        children.push((score, position, current_symbol));
                        return;
                    }
                    None => visited_tiles.insert((score + cost, position, symbol), vec![(score, position, current_symbol)])
                };
        
                if  closed_set.get(&(position, symbol)).map_or(true, |&current_cost| cost+score < current_cost)
                {
                    closed_set.insert((position, symbol), cost + score);
                    open_set.insert((score + cost, position, symbol));
                }
            }
        };

        match current_symbol {
            '^' => {
                add_move((position.0 - 1, position.1), 1, '^');
                add_move((position.0, position.1 - 1), 1000, '<');
                add_move((position.0, position.1 + 1), 1000, '>');
            }
            '>' => {
                add_move((position.0, position.1 + 1), 1,  '>');
                add_move((position.0 - 1, position.1), 1000,  '^');
                add_move((position.0 + 1, position.1), 1000,  'v');
            }
            'v' => {
                add_move((position.0 + 1, position.1), 1,  'v');
                add_move((position.0, position.1 - 1), 1000,  '<');
                add_move((position.0, position.1 + 1), 1000,  '>');
            }
            _ => {
                // '<'
                add_move((position.0, position.1 - 1), 1,  '<');
                add_move((position.0 - 1, position.1), 1000,  '^');
                add_move((position.0 + 1, position.1), 1000,  'v');
            }
        }

        if let Some(first) = open_set.pop_first() {
            score = first.0;
            position = first.1;
            current_symbol = first.2;
        } else {
            ongoing = false;
        }
    }

    let keys = visited_tiles.keys();

    let mut tiles = vec![];
    for key in keys {
        if key.1 == end_position {
            tiles.extend(count_tiles(&start_position, key, &visited_tiles));
        }
    }
    let set: HashSet<(usize, usize)> = tiles.into_iter().collect();
    
    println!("part 1 : {}", max);
    println!("part 2 : {}", set.len());
    return Some(score);
}

fn count_tiles(start: &(usize, usize),
                key: &(usize, (usize, usize), char), 
                visited_tiles: &HashMap<(usize, (usize, usize), char), Vec<(usize, (usize, usize), char)>>) -> Vec<(usize, usize)> 
{
    if key.1 == *start {
        return vec![key.1];
    }

    let parents = visited_tiles.get(key).unwrap();

    let mut result = vec![key.1];

    for parent in parents {
        result.extend(count_tiles(start, parent, visited_tiles));
    }

    return result;
}

fn part_1(mut position: (usize, usize), mut maze: Vec<Vec<char>>) {
    maze[position.0][position.1] = '>';
    let mut score = 0;

    let mut open_set: BTreeSet<(usize, (usize, usize), char)> = BTreeSet::new();
    let mut closed_set = HashSet::new();

    while position != (1, maze[1].len() - 2) {
        {
            let mut add_move = |cost: usize, new_position: (usize, usize), symbol: char| {
                if cost == 1 {
                    if maze[new_position.0][new_position.1] == '.'
                        || maze[new_position.0][new_position.1] == 'E'
                            && closed_set.insert((score + cost, new_position, symbol))
                    {
                        open_set.insert((score + cost, new_position, symbol));
                    }
                } else {
                    if maze[new_position.0][new_position.1] == '.'
                        || maze[position.0 - 1][position.1] == 'E'
                            && closed_set.insert((score + cost, position, symbol))
                    {
                        open_set.insert((score + cost, position, symbol));
                    }
                }
            };

            match maze[position.0][position.1] {
                '^' => {
                    add_move(1, (position.0 - 1, position.1), '^');
                    add_move(1000, (position.0, position.1 - 1), '<');
                    add_move(1000, (position.0, position.1 + 1), '>');
                }
                '>' => {
                    add_move(1, (position.0, position.1 + 1), '>');
                    add_move(1000, (position.0 - 1, position.1), '^');
                    add_move(1000, (position.0 + 1, position.1), 'v');
                }
                'v' => {
                    add_move(1, (position.0 + 1, position.1), 'v');
                    add_move(1000, (position.0, position.1 - 1), '<');
                    add_move(1000, (position.0, position.1 + 1), '>');
                }
                _ => {
                    // '<'
                    add_move(1, (position.0, position.1 - 1), '<');
                    add_move(1000, (position.0 - 1, position.1), '^');
                    add_move(1000, (position.0 + 1, position.1), 'v');
                }
            }
        }

        if let Some(first) = open_set.pop_first() {
            score = first.0;
            position = first.1;
            maze[position.0][position.1] = first.2;
        } else {
            println!("No route found!");

            return;
        }
    }

    println!("part 1 : {}", score);
}