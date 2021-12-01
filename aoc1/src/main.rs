use std::fs;
use std::io::{prelude::*, BufReader};

fn main() {
    let data = read_input();
    let count_pt_1 = count_more_than_last(&data);
    let sums = sum_slide_three(&data);
    let count_pt_2 = count_more_than_last(&sums);

    println!("Part 1: {}", count_pt_1);
    println!("Part 2: {}", count_pt_2);
}

fn read_input() -> Vec<u32> {
    let input_file = fs::File::open("input").expect("File read error");
    let reader = BufReader::new(input_file);
    let mut data: Vec<u32> = Vec::new();
    for line in reader.lines() {
        data.push(line.expect("ahh").parse::<u32>().unwrap());
    }

    data
}

fn count_more_than_last(data: &Vec<u32>) -> usize {
    let mut count = 0;
    let mut last = &data[0];
    for x in &data[1..] {
        if x > last {
            count += 1;
        }
        last = x;
    }

    count
}

fn sum_slide_three(vec: &Vec<u32>) -> Vec<u32> {
    let mut sums = Vec::new();
    for x in vec.windows(3) {
        let sum: u32 = x.iter().sum();
        sums.push(sum);
    }

    sums
}

#[test]
fn test_part_one() {
    let v = vec![1, 2, 1, 3, 1, 5];
    assert_eq!(count_more_than_last(&v), 3);
}

#[test]
fn test_part_two() {
    let v = vec![1, 2, 1, 3, 1, 5, 1, 1];
    let v_sums = sum_slide_three(&v);
    assert_eq!(v_sums, vec![4, 6, 5, 9, 7, 7]);
    assert_eq!(count_more_than_last(&v_sums), 2);
}