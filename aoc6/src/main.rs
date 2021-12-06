use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let mut fish = read_input();

    model_growth(&mut fish, 80);
    println!("Part 1: {:?}", solve(&fish));

    model_growth(&mut fish, 256 - 80);
    println!("Part 2: {:?}", solve(&fish));
}

fn read_input() -> HashMap<i32, i64> {
    let input: Vec<i32> = read_to_string("input.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    count_initial_fish(input)
}

fn count_initial_fish(input: Vec<i32>) -> HashMap<i32, i64> {
    let mut value_counts: HashMap<i32, i64> = HashMap::new();

    for i in 0..=8 {
        value_counts.insert(i, 0);
    }

    for fish in input.iter() {
        *value_counts.entry(*fish).or_insert(0) += 1;
    }

    value_counts
}

fn simulate_day(fish: &mut HashMap<i32, i64>)  {
    let n_new_fish = fish[&0];
    for x in 0..=8 {
        if x == 0 {
            *fish.get_mut(&0).unwrap() = 0;
        } else if x == 6 { 
            *fish.get_mut(&5).unwrap() += fish[&6];
            *fish.get_mut(&6).unwrap() = n_new_fish;
        } else {
            *fish.get_mut(&(x - 1)).unwrap() += fish[&x];
            *fish.get_mut(&x).unwrap() = 0;
        }
    }

    *fish.get_mut(&8).unwrap() = n_new_fish;
}

fn model_growth(fish: &mut HashMap<i32, i64>, days: usize) {
    for _day in 1..=days {
        simulate_day(fish);
    }
}

fn solve(fish: &HashMap<i32, i64>) -> i64 {
    fish.values().sum::<i64>()
}

#[test] 
fn test_part_one() {
    let mut fish = count_initial_fish(vec![3, 4, 3, 1, 2]);
    println!("{:?}", fish);
    model_growth(&mut fish, 18);
    println!("{:?}", fish);

    assert_eq!(solve(&fish), 26)
}

#[test] 
fn test_part_two() {
    let mut fish = count_initial_fish(vec![3, 4, 3, 1, 2]);
    model_growth(&mut fish, 256);

    assert_eq!(solve(&fish), 26984457539)
}