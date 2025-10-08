use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

//TODO topological sort i stedet for swap sort

fn parse_file(path: &str) -> io::Result<(Vec<(usize, usize)>, Vec<Vec<usize>>)> {
    // Define vectors to hold the parsed data
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    let mut lists: Vec<Vec<usize>> = Vec::new();

    // Open the file
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Parse the file line by line
    for line in reader.lines() {
        let line = line?;
        if line.contains('|') {
            // Process pairs
            if let Some((first, second)) = line.split_once('|') {
                let first_num = first.trim().parse::<usize>().unwrap();
                let second_num = second.trim().parse::<usize>().unwrap();
                pairs.push((first_num, second_num));
            }
        } else if line.contains(',') {
            // Process lists
            let numbers = line
                .split(',')
                .map(|num| num.trim().parse::<usize>().unwrap())
                .collect();
            lists.push(numbers);
        }
    }

    Ok((pairs, lists))
}

fn main() {
    part1();
    part2();
}

fn part1() {
        let path = "input.txt";

        let (pairs, lists) = parse_file(path).unwrap();
    
        let mut look_up_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    
        for (id, child) in pairs {
            look_up_map.entry(id).or_insert_with(HashSet::new).insert(child);
        }
    
        let mut result = 0;
    
        'outer: for list in lists {
            for i in (1..list.len()).rev() {
                let remaining = &list[0..i];
    
                if let Some(result) = look_up_map.get(&list[i]) {
                    if remaining.iter().any(|&x| result.contains(&x)) { continue 'outer; }
                }
            }
    
            result += list[list.len() / 2];
        }
    
        println!("result {}", result);

}

fn part2() {
    let path = "input.txt";

    let (pairs, lists) = parse_file(path).unwrap();

    let mut look_up_map: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (id, child) in pairs {
        look_up_map.entry(id).or_insert_with(HashSet::new).insert(child);
    }

    let mut result = 0;

    'outer: for list in lists {
        for i in (1..list.len()).rev() {
            let remaining = &list[0..i];

            if let Some(set) = look_up_map.get(&list[i]) {
                if remaining.iter().any(|&x| set.contains(&x)) { 

                    let reordered_vec = topological_sort(&look_up_map, list.clone());

                    result += reordered_vec[list.len() / 2];
                    continue 'outer;
                }
            }
        }
    }

    println!("result {}", result);
}

fn reorder(look_up_map: &HashMap<usize, HashSet<usize>>, mut list: Vec<usize>) -> Vec<usize> {

    while !is_ordering_correct(look_up_map, &list) {

        for i in (1..list.len()).rev() {
            let remaining = &list[0..i];
    
            if let Some(set) = look_up_map.get(&list[i]) {
                if let Some(index) = remaining.iter().position(|&x| set.contains(&x)) {
                    list.swap(index, i);
                }
            }
        }
    }

    return list;
}

fn is_ordering_correct(look_up_map: &HashMap<usize, HashSet<usize>>, list: &Vec<usize>) -> bool {
    for i in (1..list.len()).rev() {
        let remaining = &list[0..i];

        if let Some(set) = look_up_map.get(&list[i]) {
            if remaining.iter().any(|&x| set.contains(&x)) { 

                return false;
            }
        }
    }

    return true;
}

fn topological_sort(look_up_map: &HashMap<usize, HashSet<usize>>, list: Vec<usize>) -> Vec<usize> {
    let mut in_degrees: HashMap<usize, usize> = HashMap::new();
    for &node in &list {
        in_degrees.entry(node).or_insert(0);

        if let Some(dependencies) = look_up_map.get(&node) {
            for depency in dependencies {
                *in_degrees.entry(*depency).or_insert(0) += 1;
            }   
        }
    }

    let mut queue: VecDeque<usize> = list.iter()
        .filter(|&&node| *in_degrees.get(&node).unwrap_or(&0) == 0)
        .cloned()
        .collect();

    let mut sorted: Vec<usize> = Vec::new();
    while let Some(node) = queue.pop_front() {
        sorted.push(node);

        if let Some(children) = look_up_map.get(&node) {
            for &child in children {
                if list.contains(&child) {
                    if let Some(in_degree) = in_degrees.get_mut(&child) {
                        *in_degree -= 1;
                        if *in_degree == 0 {
                            queue.push_back(child);
                        }
                    }
                }
            }
        }
    }

    if sorted.len() != list.len() {
        println!("Cycle detected! Partial sort: {:?}", sorted);
        return Vec::new();
    }

    sorted
}

    // let mut in_degrees: HashMap<usize, usize> = HashMap::new();
    // for &node in &list {
    //     in_degrees.entry(node).or_insert(0);
    // }

    // for (&parent, children) in look_up_map {
    //     if list.contains(&parent) {
    //         for &child in children {
    //             if list.contains(&child) {
    //                 *in_degrees.entry(child).or_insert(0) += 1;
    //             }
    //         }
    //     }
    // }


// fn reorder(look_up_map: &HashMap<usize, HashSet<usize>>, list: Vec<usize>) -> Vec<usize> {
//     match list.len() {
//         0 => return Vec::new(),
//         1 => return list,
//         _ => {
//             for i in (1..list.len()).rev() {
//                 let remaining = &list[0..i];
//                 let current_element = list[i];

//                 // Check if the current element has any children in the remaining list
//                 if let Some(set) = look_up_map.get(&current_element) {

//                     if !remaining.iter().any(|&x| set.contains(&x)) {

//                         let new_vec = list.iter().enumerate()
//                             .filter(|(n, _)| *n != i)
//                             .map(|(_, &value)| value)
//                             .collect::<Vec<_>>();

//                         return [reorder(look_up_map, new_vec), vec![current_element]].concat();
//                     }
//                 } else {
//                     let new_vec = list.iter().enumerate()
//                         .filter(|(n, _)| *n != i)
//                         .map(|(_, &value)| value)
//                         .collect::<Vec<_>>();

//                     return [reorder(look_up_map, new_vec), vec![current_element]].concat();
//                 }
//             }

//             return list;
//         }
//     }
// }
