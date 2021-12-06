use std::fs;
use std::collections::HashMap;

type Coords = Vec<Vec<Vec<i32>>>;
type Pair = Vec<Vec<i32>>;
type Point = (i32, i32);

fn main() {
    let contents: Coords = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|s| s.split(",").map(|s| s.parse::<i32>().unwrap()).collect())
                .collect()
        })
        .collect();

    let points = extend_coords(&contents, false);
    let points2 = extend_coords(&contents, true);

    println!("Part 1: {}", count_points(points));
    println!("Part 2: {}", count_points(points2));
}

fn count_points(points: Vec<Point>) -> usize {
    let mut value_counts : HashMap<Point, i32> = HashMap::new();

    for point in points.iter() {
        *value_counts.entry(*point).or_insert(0) += 1;
    };
    
    value_counts
        .into_iter()
        .filter(|x| x.1 > 1)
        .count()
}

fn extend_coords(coords: &Coords, count_diagonals: bool) -> Vec<Point> {
    let mut expanded_coords = Vec::new();
    for pair in coords {
        if pair[0][0] == pair[1][0] {
            expanded_coords.append(&mut extend_vertical_coords(pair.to_vec()));
        } else if pair[0][1] == pair[1][1] {
            expanded_coords.append(&mut extend_horizontal_coords(pair.to_vec()));
        } else if count_diagonals {
            expanded_coords.append(&mut extend_diagonal_coords(pair.to_vec()));
        }
    }

    expanded_coords
}

fn extend_vertical_coords(pair: Pair) -> Vec<Point> {
    let (x_start, y_start, _x_end, y_end) = explode_points(pair);
    let mut points: Vec<Point> = Vec::new();

    if y_start > y_end {
        for y in (y_end..=y_start).rev() {
            points.push((x_start, y))
        }
    } else {
        for y in y_start..=y_end {
            points.push((x_start, y))
        }
    }

    points
}

fn extend_horizontal_coords(pair: Pair) -> Vec<Point> {
    let (x_start, y_start, x_end, _y_end) = explode_points(pair);
    let mut points: Vec<Point> = Vec::new();
    if x_start > x_end {
        for x in (x_end..=x_start).rev() {
            points.push((x, y_start));
        }
    } else {
        for x in x_start..=x_end {
            points.push((x, y_start));
        }
    }


    points
}

fn explode_points(pair: Pair) -> (i32, i32, i32, i32) {
    (pair[0][0], pair[0][1], pair[1][0], pair[1][1])
}

fn extend_diagonal_coords(pair: Pair) -> Vec<Point> {
    let (x_start, y_start, x_end, y_end) = explode_points(pair);
    let mut points: Vec<Point> = Vec::new();

    // up and right
    if x_start < x_end && y_start < y_end {
        let x: Vec<i32> = (x_start..=x_end).collect();
        let y: Vec<i32> = (y_start..=y_end).collect();

        for i in 0..x.len() {
            points.push((x[i], y[i]));
        }

    // down and right
    } else if x_start < x_end && y_start > y_end {
        let x: Vec<i32> = (x_start..=x_end).collect();
        let y: Vec<i32> = (y_end..=y_start).rev().collect();

        for i in 0..x.len() {
            points.push((x[i], y[i]));
        }

    // down and left
    } else if x_start > x_end && y_start > y_end {
        let x: Vec<i32> = (x_end..=x_start).rev().collect();
        let y: Vec<i32> = (y_end..=y_start).rev().collect();

        for i in 0..x.len() {
            points.push((x[i], y[i]));
        }

    // up and left
    } else if x_start > x_end && y_start < y_end {
        let x: Vec<i32> = (x_end..=x_start).rev().collect();
        let y: Vec<i32> = (y_start..=y_end).collect();

        for i in 0..x.len() {
            points.push((x[i], y[i]));
        }
    } else {
        panic!("direction not supported")
    }

    points
}

#[test]
fn test_extend_coords() {
    let vertical: Pair = vec![vec![1, 1], vec![1, 3]];
    assert_eq!(extend_vertical_coords(vertical), vec![(1, 1), (1, 2), (1, 3)]);

    let vertical_rev: Pair = vec![vec![1, 3], vec![1, 1]];
    assert_eq!(extend_vertical_coords(vertical_rev), vec![(1, 3), (1, 2), (1, 1)]);

    let horizontal: Pair = vec![vec![7, 7], vec![9, 7]];
    assert_eq!(extend_horizontal_coords(horizontal), vec![(7, 7), (8, 7), (9, 7)]);

    let horizontal_rev: Pair = vec![vec![9, 7], vec![7, 7]];
    assert_eq!(extend_horizontal_coords(horizontal_rev), vec![(9, 7), (8, 7), (7, 7)]);

    let diagonal_up_right: Pair = vec![vec![1, 1], vec![3, 3]];
    assert_eq!(extend_diagonal_coords(diagonal_up_right), vec![(1, 1), (2, 2), (3, 3)]);
    
    let diagonal_up_left: Pair = vec![vec![9, 7], vec![7, 9]];
    assert_eq!(extend_diagonal_coords(diagonal_up_left), vec![(9, 7), (8, 8), (7, 9)]);
}

#[test]
fn test_part_one() {
    let coords: Coords = vec![
        vec![vec![0, 9], vec![5, 9]],
        vec![vec![8, 0], vec![0, 8]],
        vec![vec![9, 4], vec![3, 4]],
        vec![vec![2, 2], vec![2, 1]],
        vec![vec![7, 0], vec![7, 4]],
        vec![vec![6, 4], vec![2, 0]],
        vec![vec![0, 9], vec![2, 9]],
        vec![vec![3, 4], vec![1, 4]],
        vec![vec![0, 0], vec![8, 8]],
        vec![vec![5, 5], vec![8, 2]],
    ];

    let points = extend_coords(&coords, false);
    assert_eq!(count_points(points), 5);
}

#[test]
fn test_part_two() {
    let coords: Coords = vec![
        vec![vec![0, 9], vec![5, 9]],
        vec![vec![8, 0], vec![0, 8]],
        vec![vec![9, 4], vec![3, 4]],
        vec![vec![2, 2], vec![2, 1]],
        vec![vec![7, 0], vec![7, 4]],
        vec![vec![6, 4], vec![2, 0]],
        vec![vec![0, 9], vec![2, 9]],
        vec![vec![3, 4], vec![1, 4]],
        vec![vec![0, 0], vec![8, 8]],
        vec![vec![5, 5], vec![8, 2]],
    ];

    let points = extend_coords(&coords, true);
    assert_eq!(count_points(points), 12);
}