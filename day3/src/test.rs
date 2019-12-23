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

    let comparison = compare_points(points_a, points_b);

    assert_eq!(vec![(6, 5), (3, 3)], comparison);
}

#[test]
fn finding_closest() {
    let (points_a, points_b) = setup_points();
    let comparison = compare_points(points_a, points_b);
    let closest = find_closest(comparison);

    assert_eq!((3, 3), closest);
}

#[test]
fn getting_distance() {
    let (points_a, points_b) = setup_points();
    let comparison = compare_points(points_a, points_b);
    let closest = find_closest(comparison);
    let distance = get_distance(closest);

    assert_eq!(6, distance);
}
