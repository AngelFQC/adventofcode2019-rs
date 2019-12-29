#[cfg(test)]
use super::*;

fn setup_points() -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let points_a = get_points(get_instructions("R8,U5,L5,D3"));
    let points_b = get_points(get_instructions("U7,R6,D4,L4"));

    (points_a, points_b)
}

#[test]
fn getting_points() {
    let (points_a, points_b) = setup_points();

    assert_eq!(
        vec![
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 0),
            (8, 1),
            (8, 2),
            (8, 3),
            (8, 4),
            (8, 5),
            (7, 5),
            (6, 5),
            (5, 5),
            (4, 5),
            (3, 5),
            (3, 4),
            (3, 3),
            (3, 2)
        ],
        points_a
    );
    assert_eq!(
        vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 5),
            (0, 6),
            (0, 7),
            (1, 7),
            (2, 7),
            (3, 7),
            (4, 7),
            (5, 7),
            (6, 7),
            (6, 6),
            (6, 5),
            (6, 4),
            (6, 3),
            (5, 3),
            (4, 3),
            (3, 3),
            (2, 3)
        ],
        points_b
    );
}

#[test]
fn comparing_points() {
    let (points_a, points_b) = setup_points();

    let comparison = get_crossing_points(&points_a, &points_b);

    assert_eq!(vec![(6, 5), (3, 3)], comparison);
}

#[test]
fn finding_closest() {
    let (points_a, points_b) = setup_points();
    let comparison = get_crossing_points(&points_a, &points_b);
    let closest = find_closest(comparison);

    assert_eq!((3, 3), closest);
}

#[test]
fn getting_distance() {
    let (points_a, points_b) = setup_points();
    let comparison = get_crossing_points(&points_a, &points_b);
    let closest = find_closest(comparison);
    let distance = get_distance(closest);

    assert_eq!(6, distance);
}

#[test]
fn getting_best_steps() {
    let data = [
        vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
            "610",
        ],
        vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            "410",
        ],
    ];

    for d in data.iter() {
        let points_a = get_points(get_instructions(d[0]));
        let points_b = get_points(get_instructions(d[1]));

        let comparison = get_crossing_points(&points_a, &points_b);
        let better = get_best_steps(comparison, points_a, points_b);

        let expected_better: i32 = d[2].parse().unwrap();

        assert_eq!(expected_better, better);
    }
}
