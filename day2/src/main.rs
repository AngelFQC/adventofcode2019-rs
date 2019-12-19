use std::{env, fs};

fn split_in_instructions(line: &str) {
    let parts: Vec<&str> = line.split(",").collect();
    let mut parsed: Vec<i32> = parts.into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    for (i, opcode) in parsed.iter().enumerate() {
        if i != 0 && i % 4 != 0 {
            continue;
        }

        let input_a_pos: usize = parsed[i as usize + 1_usize] as usize;
        let input_b_pos: usize = parsed[i as usize + 2_usize] as usize;
        let output_pos: usize = parsed[i as usize + 3_usize] as usize;

        println!("{:?} {:?} {:?}", input_a_pos, input_b_pos, output_pos);

        let input_a: i32 = parsed[input_a_pos];
        let input_b: i32 = parsed[input_b_pos];

        println!("{:?} {:?}", input_a, input_b);

        match opcode {
            1 => parsed[output_pos] = input_a + input_b,
            2 => parsed[output_pos] = input_a * input_b,
            99 => break,
            _ => panic!("opcode encountered: {}", opcode)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File was not read");

    for line in content.lines() {
        split_in_instructions(line);
    }
}
