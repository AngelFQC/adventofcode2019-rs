use std::{env, fs};
use core::str::Lines;

fn fuel_required_for_module(mass: u32) -> u32 {
    let divided = mass as f32 / 3_f32;
    let rounded = divided.floor() as u32;
    let fuel = rounded - 2_u32;

    fuel
}

fn make_sum(lines: Lines) -> u32 {
    let mut sum = 0;

    for line in lines {
        let mass: u32 = line.parse().unwrap();

        sum = sum + fuel_required_for_module(mass);
    }

    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File was not read");

    let sum = make_sum(content.lines());

    println!("---> {:?}", sum);
}
