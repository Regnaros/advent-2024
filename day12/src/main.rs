use std::{fs::File, io::{self, BufRead}, vec};

const SIZE: usize = 140;

const CORNER_LOOKUP: [usize; 16] = [
    0, // 0b0000: No sides
    0, // 0b0001: Left side only
    0, // 0b0010: Bottom side only
    1, // 0b0011: Bottom + Left (1 corner)
    0, // 0b0100: Right side only
    0, // 0b0101: Right + Left (1 corner)
    1, // 0b0110: Bottom + Right (1 corner)
    2, // 0b0111: Bottom + Right + Left (2 corners)
    0, // 0b1000: Top side only
    1, // 0b1001: Top + Left (1 corner)
    0, // 0b1010: Top + Bottom (1 corner)
    2, // 0b1011: Top + Bottom + Left (2 corners)
    1, // 0b1100: Top + Right (1 corner)
    2, // 0b1101: Top + Right + Left (2 corners)
    2, // 0b1110: Top + Right + Bottom (2 corners)
    4, // 0b1111: All sides (4 corners)
];

fn main() {
    let input = load_map("input.txt").unwrap();

    part_1(input);
}

fn part_1(mut map: [[(char, bool); SIZE]; SIZE]) {
    let mut plots: Vec<(usize, usize, usize)> = vec![];

    let mut start_positions = vec![];

    for line in 0..map.len() {
        for char in 0..map[line].len() {
            if map[line][char].1 {
                continue;
            }

            start_positions.push((line, char));

            let my_res = find_plot(line, char, &mut map);
            plots.push(my_res);
        }
    }

    let mut res = 0;

    for i in 0..plots.len() {
        res += plots[i].0 * plots[i].1;
    }

    println!("result part 1 {}", res);
    res = 0;

    for i in 0..plots.len() {
        res += plots[i].2 * plots[i].1;
    }

    println!("result part 2 {}", res);
}

fn find_plot(line: usize, char: usize, map: &mut [[(char, bool); SIZE]; SIZE]) -> (usize, usize, usize) {
    if map[line][char].1 {
        return (0, 0, 0);
    }

    let mut fence = 4;
    let mut area = 1;
    map[line][char].1 = true;

    let mut corners_byte: u8 = 0b1111;
    let mut corners = 0;

    let mut up_free = false;
    let mut down_free = false;
    let mut right_free = false;
    let mut left_free = false;

    // UP
    if line > 0 && map[line - 1][char].0 == map[line][char].0 {
        fence -= 1;
        corners_byte &= !0b1000;
        let res = find_plot(line - 1, char, map);
        fence += res.0;
        area += res.1;
        corners += res.2;
        up_free = true;
    }

    // DOWN
    if line < SIZE - 1 && map[line + 1][char].0 == map[line][char].0 {
        fence -= 1;
        corners_byte &= !0b0010;
        let res = find_plot(line + 1, char, map);
        fence += res.0;
        area += res.1;
        corners += res.2;
        down_free = true;
    }

    // RIGHT
    if char < SIZE - 1 && map[line][char + 1].0 == map[line][char].0 {
        fence -= 1;
        corners_byte &= !0b0100;
        let res = find_plot(line, char + 1, map);
        fence += res.0;
        area += res.1;
        corners += res.2;
        right_free = true;
    }

    // LEFT
    if char > 0 && map[line][char - 1].0 == map[line][char].0 {
        fence -= 1;
        corners_byte &= !0b0001;
        let res = find_plot(line, char - 1, map);
        fence += res.0;
        area += res.1;
        corners += res.2;
        left_free = true;
    }
    
    if up_free && left_free && map[line -1][char -1].0 != map[line][char].0 { corners += 1 }
    if up_free && right_free && map[line -1][char +1].0 != map[line][char].0 { corners += 1 }
    if down_free && right_free && map[line +1][char +1].0 != map[line][char].0 { corners += 1 }
    if down_free && left_free && map[line +1][char -1].0 != map[line][char].0 { corners += 1 }

    return (fence, area, CORNER_LOOKUP[corners_byte as usize] + corners);
}

fn find_plot_chatgpt_short(line: usize, char: usize, map: &mut [[(char, bool); SIZE]; SIZE]) -> (usize, usize, usize) {
    if map[line][char].1 {
        return (0, 0, 0);
    }

    let mut fence = 4;
    let mut area = 1;
    map[line][char].1 = true;

    let mut corners_byte: u8 = 0b1111;
    let mut corners = 0;

    let directions = [
        (-1, 0, 0b1000), // UP
        (1, 0, 0b0010),  // DOWN
        (0, 1, 0b0100),  // RIGHT
        (0, -1, 0b0001), // LEFT
    ];

    let mut free_sides = [false; 4]; // Tracks whether each side is free (UP, DOWN, RIGHT, LEFT)

    // Handle neighbors
    for (idx, &(dx, dy, bit)) in directions.iter().enumerate() {
        let new_line = line.wrapping_add(dx as usize);
        let new_char = char.wrapping_add(dy as usize);

        if new_line < SIZE && new_char < SIZE && map[new_line][new_char].0 == map[line][char].0 {
            fence -= 1;
            corners_byte &= !bit;
            let res = find_plot(new_line, new_char, map);
            fence += res.0;
            area += res.1;
            corners += res.2;
            free_sides[idx] = true;
        }
    }

    // Check corners
    let corner_checks = [
        (-1, -1, 0, 3), // UP-LEFT
        (-1, 1, 0, 2),  // UP-RIGHT
        (1, 1, 1, 2),   // DOWN-RIGHT
        (1, -1, 1, 3),  // DOWN-LEFT
    ];

    for &(dx, dy, side1, side2) in &corner_checks {
        let new_line = line.wrapping_add(dx as usize);
        let new_char = char.wrapping_add(dy as usize);

        if free_sides[side1] && free_sides[side2] {
            if new_line >= SIZE || new_char >= SIZE || map[new_line][new_char].0 != map[line][char].0 {
                corners += 1;
            }
        }
    }

    return (fence, area, CORNER_LOOKUP[corners_byte as usize] + corners);
}


// fn find_plot(line: usize, char: usize, map: &mut [[(char, bool); SIZE]; SIZE]) -> (usize, usize) {

//     if map[line][char].1 {
//         return (0, 0);
//     }

//     let mut fence = 4;
//     let mut area = 1;
//     map[line][char].1 = true;

//     if line > 0 && map[line -1][char].0 == map[line][char].0 {
//         fence -= 1;

//         let res = find_plot(line - 1, char, map);
//         fence += res.0;
//         area += res.1;
//     }

//     if line < SIZE - 1 && map[line + 1][char].0 == map[line][char].0 {
//         fence -= 1;
//         let res = find_plot(line +1, char, map);
//         fence += res.0;
//         area += res.1;
//     }

//     if char < SIZE - 1 && map[line][char +1].0 == map[line][char].0 {
//         fence -= 1;
//         let res = find_plot(line, char +1, map);
//         fence += res.0;
//         area += res.1;
//     }

//     if char > 0 && map[line][char -1].0 == map[line][char].0 {
//         fence -= 1;
//         let res = find_plot(line, char -1, map);
//         fence += res.0;
//         area += res.1;
//     }

//     return (fence, area);
// }

// fn part_1(mut map: [[(char, i16); SIZE]; SIZE]) {

//     let mut plots: Vec<(usize, usize, HashSet<usize>)> = Vec::new(); // Area, fence
//     let mut found_plots = 0;

//     for line in 0..SIZE {
//         for plant in 0..SIZE {

//             //let mut fence = ((line == 0 || line == SIZE -1) || map[line][plant].1 != -1) as usize + (plant == 0 && plant == SIZE - 1) as usize;

//             let mut fence = (line == 0) as usize + (line == SIZE - 1) as usize + (plant == 0  || map[line][plant].1 == -1 ) as usize + (plant == SIZE - 1) as usize;


//             let mut new_plot = false;

//             if line > 0 {
//                 let upstairs_neighbour = map[line - 1][plant];

//                 // Kig op og vi er ens
//                 if upstairs_neighbour.0 == map[line][plant].0 {

//                     // Har jeg ikke selv et ID bliver jeg del af deres gruppe
//                     if map[line][plant].1 == -1 {
//                         map[line][plant].1 = upstairs_neighbour.1
//                     }
//                     // Hvis jeg har et ID skal min gruppe tilføjeres til deres vector
//                     else {
//                         if map[line][plant].1 != upstairs_neighbour.1 {
//                             plots[upstairs_neighbour.1 as usize].2.insert(map[line][plant].1 as usize); 
//                         }
//                     }
                    

//                 // Kig op og vi er ikke ens
//                 } else {
//                     // Give mig selv og dem et fence bonus
//                     fence += 1;
//                     plots[map[line - 1][plant].1 as usize].1 += 1;

//                     // Har jeg ikke selv en gruppe er jeg en ny
//                     if map[line][plant].1 == -1 {
//                         map[line][plant].1 = found_plots as i16;
//                         new_plot = true;
//                     } 

//                     // SUSPICIOUS                            
//                 }
//             } else {
//                 if map[line][plant].1 == -1 {

//                     map[line][plant].1 = found_plots as i16;
//                     new_plot = true;
//                 }
//             }

//             //Kig til højre
//             if plant < SIZE - 1 {
//                 // Kig til side og vi er ens
//                 if map[line][plant + 1].0 == map[line][plant].0 {
//                     map[line][plant + 1].1 = map[line][plant].1;
//                 } else {
//                     fence += 1;
//                 }
//             } 

//             if new_plot { 
//                 plots.push((1, fence, HashSet::new()));
//                 found_plots += 1; 
//             } else {
//                 plots[map[line][plant].1 as usize].0 += 1;
//                 plots[map[line][plant].1 as usize].1 += fence;
//             }

//         }
//     }

//     // for line2 in map {
//     //     println!("{:?}", line2);
//     // }

//     // println!("result {:?}", plots);
//     // println!("result {:?}", plots.len());

//     // for i in 0..plots.len() {
//     //     println!("{:?} {}", plots[i], i);
//     // }


//     //let mut processed = HashSet::new();

//     let mut res_vec = vec![(0, 0); plots.len()];

//     for plot in (0..plots.len()).rev() {
//         res_vec[plot] = (plots[plot].0, plots[plot].1);

//         for subplot in plots[plot].2.clone() {
//             res_vec[plot].0 += res_vec[subplot].0;
//             res_vec[plot].1 += res_vec[subplot].1;
//             res_vec[subplot] = (0,0)
//         }
//     }

//     let mut result = 0;

//     for res in res_vec {
//         result += res.0 * res.1;
//     }

//     println!("{}", result);
    
//    // calculate_price(plots);
// }

// fn calculate_plots(plot: &(usize, usize, HashSet<usize>), plots: &Vec<(usize, usize, HashSet<usize>)>) -> (usize, usize) {

//     if plot.2.is_empty() {
//         return (plot.0, plot.1);
//     }

//     let mut area = plot.0;
//     let mut fence = plot.1;

//     for &subplot in &plot.2 {
//         let res = calculate_plots(&plots[subplot], plots);
//         area += res.0;
//         fence += res.1;
//     }

//     return (area, fence);
// }

// fn calculate_plots(group_part: &mut (usize, usize, HashSet<usize>, bool), plots: &Vec<(usize, usize, HashSet<usize>, bool)>) -> (usize, usize) {

//     let mut area = group_part.0;
//     let mut fence = group_part.1;

//     for fragment in group_part.2 {
//         if plots[fragment].3 {
//             continue;
//         }

//         let res = calculate_plots(&mut &plots[fragment], plots);
//         area += res.0;
//         fence += res.1;
//     }

//     group_part.3 = true;
//     return (area, fence)
// }


fn load_map(file_path: &str) -> io::Result<[[(char, bool); SIZE]; SIZE]> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut matrix = [[(' ', false); SIZE]; SIZE];

    for (i, line) in reader.lines().enumerate() {
        if i >= SIZE {
            break;
        }

        let chars: Vec<char> = line?.chars().collect();

        for (j, &ch) in chars.iter().take(SIZE).enumerate() {
            matrix[i][j].0 = ch;
        }
    }

    Ok(matrix)
}