use std::{env, fs};

fn get_instructions(line_str: &str) -> Vec<&str> {
    let parts: Vec<&str> = line_str.split(',').collect();

    parts
}

fn get_points(instructions: Vec<&str>) -> Vec<(i32, i32)> {
    let mut points = Vec::new();

    let mut x = 0;
    let mut y = 0;

    for instruction_str in instructions {
        let (direction, size_str) = instruction_str.split_at(1);
        let size: i32 = size_str.parse().unwrap();

        for _i in 1..=size {
            match direction {
                "U" => y += 1,
                "R" => x += 1,
                "D" => y -= 1,
                "L" => x -= 1,
                _ => continue,
            };

            points.push((x, y));
        }
    }

    points
}

fn compare_points(points_a: Vec<(i32, i32)>, points_b: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut comparison = Vec::new();

    for point_a in points_a {
        if points_b.contains(&point_a) {
            comparison.push(point_a);
        };
    }

    comparison
}

fn find_closest(comparison: Vec<(i32, i32)>) -> (i32, i32) {
    let mut closest = comparison[0];
    let mut distance = (closest.0).abs() + (closest.1).abs();

    for point in comparison.iter().skip(1) {
        if distance > point.0.abs() + point.1.abs() {
            distance = point.0.abs() + point.1.abs();

            closest = *point;
        }
    }

    closest
}

fn get_distance(closest: (i32, i32)) -> i32 {
    closest.0.abs() + closest.1.abs()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let content = fs::read_to_string(filename).expect("File was not read.");
    let mut lines = content.lines();

    let line_a_str = match lines.next() {
        Some(line_a_str) => line_a_str,
        None => panic!("Line A not found."),
    };
    let line_b_str = match lines.next() {
        Some(line_b_str) => line_b_str,
        None => panic!("Line B not found"),
    };

    let points_a = get_points(get_instructions(line_a_str));
    let points_b = get_points(get_instructions(line_b_str));

    let comparison = compare_points(points_a, points_b);

    let closest = find_closest(comparison);

    let distance = get_distance(closest);

    println!("Manhattan distance: {}", distance);
}

#[cfg(test)]
mod test;
