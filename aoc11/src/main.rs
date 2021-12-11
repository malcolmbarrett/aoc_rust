use std::fs::read_to_string;
use std::collections::HashMap;
type OctoMatrix = Vec<Vec<u32>>;
type OctoMap = HashMap<(isize, isize), u32>;

fn main() {
    let input = read_input();
    let octopi = as_octopi(&input);
    println!("Part 1: {:?}", solve_part_one(&octopi));
    println!("Part 2: {:?}", solve_part_two(&octopi));
}

fn solve_part_one(input: &OctoMap) -> u32 {
    let mut octomap = input.clone();
    let mut flashes = 0;
    for _ in 1..=100 {
        advance_energy(&mut octomap, &mut flashes);
    }

    flashes
}

fn solve_part_two(input: &OctoMap) -> u32 {
    let mut octomap = input.clone();
    let mut flashes = 0;
    for i in 1..=1000 {
        advance_energy(&mut octomap, &mut flashes);
        if all_flashing(&octomap) {
            return i as u32;
        }
    }

    unreachable!();
}

fn as_octopi(input: &OctoMatrix) -> OctoMap {
    let mut octomap = OctoMap::new();
    input.
        iter()
        .enumerate()
        .for_each(|(row, line)| {
            line
                .iter()
                .enumerate()
                .for_each(|(col, value)| {
                    octomap.insert((row as isize, col as isize), *value);
                });
        });

    octomap
}

fn get_neighbor_coords(coords: (isize, isize)) -> Vec<(isize, isize)> {
    let mut neighbors = Vec::new();
    for move_row in -1..=1 {
        for move_col in -1..=1 {
            neighbors.push((coords.0 + move_row, coords.1 + move_col));
        }
    }

    neighbors
        .into_iter()
        .filter(|(x, y)| x >= &0 && x <= &9 && y >= &0 && y <= &9)
        .collect()
}

fn all_flashing(octomap: &OctoMap) -> bool {
    octomap
        .values()
        .all(|x| *x == 0)
}

fn advance_energy(octomap: &mut OctoMap, flashes: &mut u32) {
    let mut stack: Vec<(isize, isize)> = Vec::new();
    for row in 0..10 {
        for col in 0..10 {
            stack.push((row, col));
            while let Some((row, col)) = stack.pop() {
                let energy = octomap.get_mut(&(row, col)).unwrap();
                *energy += 1;
                
                if *energy == 10 {
                    let mut neighbors = get_neighbor_coords((row, col));
                    stack.append(&mut neighbors);
                }
            }
        }
    }

    *flashes += count_flashes(&octomap);
    reset_flashes(octomap);
}

fn count_flashes(octomap: &OctoMap) -> u32 {
    octomap
        .values()
        .filter(|&value| *value > 9)
        .count() as u32
}

fn reset_flashes(octomap: &mut OctoMap) {
    for energy in octomap.values_mut() {
        if *energy > 9 {
            *energy = 0;
        }
    }
}

fn read_input() -> OctoMatrix {
    read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(process_line)
        .collect()
}


fn process_line(line: &str) -> Vec<u32> {
    line
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect()
}

#[test]
fn test_part_one() {
    let input = vec![
        vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ];
    let octopi = as_octopi(&input);
    assert_eq!(solve_part_one(&octopi), 1656);
}

#[test]
fn test_part_two() {
    let input = vec![
        vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ];
    let octopi = as_octopi(&input);
    assert_eq!(solve_part_two(&octopi), 195);
}
