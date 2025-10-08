use std::fs;

const SIZE: usize = 19999;

fn main() -> std::io::Result<()> {
    // Path to the file
    let file_path = "input.txt";

    // Read the file contents into a String
    let file_contents = fs::read_to_string(file_path)?.trim().to_string();

    // Convert the string into a Vec<char>
    let integer_array: [u16; SIZE] = file_contents
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u16) // Convert each character to u32
        .collect::<Vec<u16>>()
        .try_into()
        .map_err(|_| std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to convert to [u32; SIZE]. Ensure the input has exactly 20,000 digits.",
        ))?;

    part2(&integer_array);

    Ok(())
}

fn part2(input: &[u16; SIZE]) {
    let max = SIZE;
    let mut compressed_index: u32 = 0;
    let mut file_block: usize = max - 1;
    let mut free_block: usize;
    let mut first_free_block = 1;

    let mut informed_blocks: [(u16, u16, u32); SIZE] = [(0, 0, 0); SIZE]; //id, lenght, start

    for i in 0..input.len() {
        if i % 2 == 0 {
            informed_blocks[i] = ((i / 2) as u16, input[i], compressed_index);
            compressed_index += input[i] as u32;
        }
        else {
            informed_blocks[i] = (20000, input[i], compressed_index);
            compressed_index += input[i] as u32;
        }
    }

    let mut found_free_spot: bool;
    while first_free_block < file_block {

        found_free_spot = false;
        free_block = first_free_block;

        while file_block > free_block {

            if !found_free_spot && informed_blocks[free_block].1 > 0 {
                found_free_spot = true;
                first_free_block = free_block;
            }

            if informed_blocks[free_block].1 >= informed_blocks[file_block].1 {
                informed_blocks[file_block].2 = informed_blocks[free_block].2;
                informed_blocks[free_block].1 -= informed_blocks[file_block].1;
                informed_blocks[free_block].2 += informed_blocks[file_block].1 as u32;

                break;
            }
            
            free_block += 2;
        }

        file_block -= 2;
    }

    let mut result: i128 = 0;
    for file in informed_blocks {
        if file.0 < 20000 {
            result += calculate_weighted_sum(file.0, file.1, file.2);
        }
    }

    println!("result {}", result);
}


fn part1(mut input: [i32; SIZE]) {
    let max = SIZE;

    let mut result: i128 = 0;
    let mut compressed_index = 0;

    
    let mut block = 0;

    let mut file_block: usize = max - 1;
    let mut free_blocks: i32 = 0;

    while block <= file_block {
        if block % 2 == 0 {
            result += calculate_weighted_sum((block / 2) as i32, input[block], compressed_index);
            compressed_index += input[block as usize];

        } else {
            free_blocks += input[block as usize];

            while free_blocks > 0 {
                let dif = free_blocks - input[file_block];

                match dif.signum() {
                    0 => {
                        result += calculate_weighted_sum((file_block / 2) as i32, input[file_block], compressed_index);
                        compressed_index += input[file_block]; 
                        free_blocks = 0;
                        file_block -= 2;                         
                    }
                    1 => {
                        result += calculate_weighted_sum((file_block / 2) as i32, input[file_block], compressed_index);
                        compressed_index += input[file_block];
                        free_blocks -= input[file_block];
                        file_block -= 2;
                        
                    }
                    -1 => {
                        result += calculate_weighted_sum((file_block / 2) as i32, free_blocks, compressed_index);                  
                        compressed_index += free_blocks;
                        input[file_block] = input[file_block] - free_blocks as i32;
                        free_blocks = 0;
                    }
                    _ => panic!("help")
                }
            }
        }

        block += 1; 
    }

    println!("result {}", result);
}

fn calculate_weighted_sum<T, U, V>(id: T, length: U, start_index: V) -> i128
where
    T: Into<i128>,
    U: Into<i128>,
    V: Into<i128>,
{
    let id = id.into();
    let length = length.into();
    let start_index = start_index.into();

    (start_index..(length + start_index))
        .map(|i| i * id)
        .sum()
}
