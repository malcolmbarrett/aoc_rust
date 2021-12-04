use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let mut input = read_input();
    let data: Vec<f64> = transpose(input.clone())
        .into_iter()
        .map(|col|  col.iter().sum::<i32>() as f64 / col.len() as f64)
        .collect();

    let gamma: Vec<i32> = data.iter().map(|x| if x >= &0.50 { 1 } else { 0 }).collect();
    let epsilon: Vec<i32> = gamma.iter().map(|x| if x == &1 { 0 } else { 1 }).collect();

    let oxygen = filter_codes(&input, "gamma");
    let co2_scrubber = filter_codes(&input, "epsilon");

    println!("Part 1: {}", solve(gamma, epsilon));
    println!("Part 2: {}", solve(oxygen, co2_scrubber));

}

fn filter_codes(codes: &Vec<Vec<i32>>, category: &str) -> Vec<i32> {
    let mut codes = codes.clone();


    for i in 0..12 {
        let data: Vec<f64> = transpose(codes.clone())
            .into_iter()
            .map(|col|  col.iter().sum::<i32>() as f64 / col.len() as f64)
            .collect();

        let gamma: Vec<i32> = data.iter().map(|x| if x >= &0.50 { 1 } else { 0 }).collect();
        let epsilon: Vec<i32> = gamma.iter().map(|x| if x == &1 { 0 } else { 1 }).collect();

        let counts = match category {
            "gamma" => gamma,
            "epsilon" => epsilon,
            _ => panic!("Wrong category")
        };

        codes = codes
            .iter()
            .filter(|x| x[i] == counts[i])
            .cloned()
            .collect();

        if codes.len() == 1 {
            break;
        }
    }

    codes[0].clone()
}

fn solve(x: Vec<i32>, y: Vec<i32>) -> isize {
    binary_to_decimal(&vec_to_single(x)) * binary_to_decimal(&vec_to_single(y))
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

    lines
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