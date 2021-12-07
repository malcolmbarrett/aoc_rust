use std::fs::read_to_string;

fn main() {
    let input = read_input();
    println!("Part 1: {}", solve_part_one(&input));
    println!("Part 2: {}", solve_part_two(&input));
}

fn solve_part_one(input: &Vec<i32>) -> i32 {
    let median = median(&input);
    input.iter().map(|x| (x - median).abs()).sum()
}

fn solve_part_two(input: &Vec<i32>) -> i32 {
    let mean = mean(&input);
    input
        .iter()
        .map(|x| {
            let n = (x - mean).abs();
            n * (n + 1) / 2
        })
        .sum()
}

fn mean(input: &Vec<i32>) -> i32 {
    (input.iter().sum::<i32>() as f64 / input.len() as f64) as i32
}

fn median(input: &Vec<i32>) -> i32 {
    let mut input = input.clone();
    input.sort();
    let mid = input.len() / 2;
    input[mid]
}

fn read_input() -> Vec<i32> {
    read_to_string("input.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

#[test]
fn test_part_one() {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    assert_eq!(median(&input), 2);
    assert_eq!(solve_part_one(&input), 37);
}

#[test]
fn test_part_two() {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    assert_eq!(mean(&input), 4);
    assert_eq!(solve_part_two(&input), 170);
}
