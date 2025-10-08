fn main() {
    part_2();
}

fn find_first_difference_from_end(vec1: &Vec<i64>, vec2: &Vec<i64>) -> Option<usize> {

    for (index, (&a, &b)) in vec1.iter().rev().zip(vec2.iter().rev()).enumerate() {
        if a != b {

            return Some(vec1.len() - 1 - index);
        }
    }
    None
}

fn part_2(){
    let base: f64 = 8.0;
    let instructions = vec![2, 4, 1, 1, 7, 5, 4, 0, 0, 3, 1, 6, 5, 5, 3, 0];
    let mut result =  0.0;
    let mut i = 15;
    result += base.powi(i); 

    while i >= 0 {
        let current_output = part_1(result as i64, 0, 0, &instructions);
        println!("{:?}", current_output);
        if let Some(index) = find_first_difference_from_end(&current_output, &instructions) {
            i = index as i32;
        } else {
            i = -1
        }

        result += base.powi(i);
    }

    let part_one_res =  part_1(result as i64, 0, 0, &instructions);
    println!("{:?} len: {}", part_one_res,  part_one_res.len());
    println!("{}", result as i64);
}


fn part_1(input_a : i64, input_b : i64, input_c : i64, instructions: &Vec<i64>) -> Vec<i64>  {
    let mut a = input_a;
    let mut b = input_b;
    let mut c = input_c;

    let mut output: Vec<i64> = vec![]; 
    let mut pointer = 0;

    let combo = | operand: i64, a: i64, b: i64, c: i64 | -> i64 {
        match operand {
            6 => c,
            5 => b,
            4 => a,
            _ => operand,
        }
    };

    while pointer <= instructions.len() - 1 {
        let operand = instructions[pointer + 1];

        match instructions[pointer] {
            0 => {
                let combo_operand = combo(operand, a, b, c);
                let base: f64 = 2.0;
                let new_a = a as f64 / base.powf(combo_operand as f64);
                a = new_a.trunc() as i64;   
            },
            1 => {
                let new_b = b ^ operand;
                b = new_b;
            }
            2 => {
                let combo_operand = combo(operand, a, b, c);

                b = combo_operand % 8;
            }
            3 => {
                if a != 0 { 
                    pointer = operand as usize;
                    continue; 
                }
            },
            4 => b = b ^ c,
            5 => output.push(combo(operand, a, b, c) % 8),
            6 => {
                let combo_operand = combo(operand, a, b, c);
                let base: f64 = 2.0;
                let new_b = a as f64 / base.powf(combo_operand as f64);
                b = new_b.trunc() as i64;   
            }
            _ => {
                let combo_operand = combo(operand, a, b, c);
                let base: f64 = 2.0;
                let new_c = a as f64 / base.powf(combo_operand as f64);
                c = new_c.trunc() as i64;   
            }
        }

        pointer += 2;
    }

    return output
}