use std::{env, fs};
use core::str::Lines;

fn fuel_required_for_module(mass: i32) -> i32 {
    let divided = mass as f32 / 3_f32;
    let rounded = divided.floor() as i32;
    let fuel = rounded - 2_i32;

    fuel
}

fn fuel_required_for_fuel(mass: i32) -> i32 {
    let fuel = fuel_required_for_module(mass);

    if fuel > 0 {
        return fuel + fuel_required_for_fuel(fuel);
    }

    0
}

fn make_sum(lines: Lines) -> i32 {
    let mut sum = 0;

    for line in lines {
        let mass: i32 = line.parse().unwrap();

        let fuel = fuel_required_for_module(mass);
        let fuel_for_fuel = fuel_required_for_fuel(fuel);

        sum = sum + fuel + fuel_for_fuel;
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
