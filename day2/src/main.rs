use std::{env, fs};

fn split_in_instructions(line: &str) -> Vec<i32> {
    let parts: Vec<&str> = line.split(",").collect();
    
    parts
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn do_instructions(instructions: &mut Vec<i32>) {
    for i in 0..instructions.len() {
        if i != 0 && i % 4 != 0 {
            continue;
        }

        let opcode = instructions[i as usize];

        let input_a_pos: usize = instructions[i as usize + 1_usize] as usize;
        let input_b_pos: usize = instructions[i as usize + 2_usize] as usize;
        let output_pos: usize = instructions[i as usize + 3_usize] as usize;

        let input_a: i32 = instructions[input_a_pos];
        let input_b: i32 = instructions[input_b_pos];

        match opcode {
            1 => instructions[output_pos] = input_a + input_b,
            2 => instructions[output_pos] = input_a * input_b,
            99 => break,
            _ => panic!("opcode encountered: {}", opcode)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let filename = &args[1];
    let position_a: usize = args[2].parse().unwrap();
    let value_a: i32 = args[3].parse().unwrap();
    let position_b: usize = args[4].parse().unwrap();
    let value_b: i32 = args[5].parse().unwrap();

    let content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(error) => panic!("{}", error)
    };

    let mut instructions = match content.lines().next() {
        Some(line) => split_in_instructions(line),
        None => panic!("No instructions in this intcode.")
    };

    instructions[position_a] = value_a;
    instructions[position_b] = value_b;

    do_instructions(&mut instructions);

    println!("Position 0: {}", instructions[0]);
}
