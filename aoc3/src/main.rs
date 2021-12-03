use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let data: Vec<f64> = read_input()
        .into_iter()
        .map(|col|  col.iter().sum::<i32>() as f64 / col.len() as f64)
        .collect();

    let gamma: Vec<i32> = data.iter().map(|x| if x > &0.50 { 1 } else { 0 }).collect();
    let epsilon: Vec<i32> = gamma.iter().map(|x| if x == &1 { 0 } else { 1 }).collect();

    println!("Part 1: {}", solve(gamma, epsilon));
}

fn solve(gamma: Vec<i32>, epsilon: Vec<i32>) -> isize {
    binary_to_decimal(&vec_to_single(gamma)) * binary_to_decimal(&vec_to_single(epsilon))
}

fn binary_to_decimal(binary: &str) -> isize {
    isize::from_str_radix(binary, 2).unwrap()
}

fn vec_to_single(vec: Vec<i32>) -> String {
    vec
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}


fn read_input() -> Vec<Vec<i32>> {
    let input_file = File::open("input.txt").expect("File read error");
    let reader = BufReader::new(input_file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        let iter = line.expect("File read error");
        let vec = iter
            .split("")
            .filter(|s| *s != "")
            .map(|s| s.parse().unwrap())
            .collect();
        
        lines.push(vec);
    };

    transpose(lines)
}

// from https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}


#[test]
fn test_binary() {
    assert_eq!(binary_to_decimal("10110"), 22);
    assert_eq!(binary_to_decimal("01001"), 9);
}