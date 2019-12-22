use std::{env, fs};
use std::ops::Add;

fn split_in_instructions(line: &str) -> Vec<i32> {
    let parts: Vec<&str> = line.split(",").collect();
    
    parts
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn do_instructions(instructions_str: &str, noun: i32, verb: i32) -> i32 {
    let mut instructions: Vec<i32> = split_in_instructions(instructions_str);

    instructions[1] = noun;
    instructions[2] = verb;

    for i in 0..instructions.len() - 1 {
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

    instructions[0]
}

fn find_noun_verb(instructions_str: &str, result_expected: i32) -> (i32, i32) {
    for n in 1..99 {
        for v in 1..99 {
            let result = do_instructions(&instructions_str, n, v);

            if result == result_expected {
                let noun = n as i32;
                let verb = v as i32;

                return (noun, verb);
            }
        }
    }

    panic!("Fail")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let filename = &args[1];
    let expected_result: i32 = args[2].parse().unwrap();

    let content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(error) => panic!("{}", error)
    };

    let (noun, verb) = match content.lines().next() {
        Some(line) => find_noun_verb(line, expected_result),
        None => panic!("No instructions in intcode")
    };
    let result = 100 * noun + verb;

    println!("Result (100 * noun + verb): {:?}", result);
}

#[test]
fn doing_instructions() {
    assert_eq!(
        do_instructions("1,1,1,4,99,5,6,0,99", 1, 1),
        30
    );
}

#[test]
fn finding_noun_verb() {
    let content = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(error) => panic!("{}", error)
    };

    let line = match content.lines().next() {
        Some(line) => line,
        None => panic!("No instructions line"),
    };

    assert_eq!(
        find_noun_verb(&line, 2890696),
        (12, 2)
    )
}