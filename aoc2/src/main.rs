use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let mut position1 = Position { x: 0, d: 0, a: 0};
    let data = read_input();
    move_position(&mut position1, &data);

    let mut position2 = Position { x: 0, d: 0, a: 0};
    move_position_and_aim(&mut position2, &data);

    println!("Part 1: {}", position1.x * position1.d);
    println!("Part 2: {}", position2.x * position2.d);
}

#[derive(Debug)]
struct Position {
    x: isize,
    d: isize,
    a: isize,
}

impl Position {
    fn decrease_depth(&mut self, n: isize) {
        self.d -= n;
    }

    fn increase_depth(&mut self, n: isize) {
        self.d += n;
    }

    fn increase_x(&mut self, n: isize) {
        self.x += n;
    }

    fn decrease_aim(&mut self, n: isize) {
        self.a -= n;
    }

    fn increase_aim(&mut self, n: isize) {
        self.a += n;
    }

    fn increase_x_and_depth(&mut self, n: isize) {
        self.x += n;
        self.d = self.d + (self.a * n);
    }
}


#[derive(Debug)]
enum Directions {
    Forward(isize),
    Up(isize),
    Down(isize),
    NoDirection
}

fn move_position_and_aim(position: &mut Position, data: &Vec<Directions>) {
    for command in data {
        match command {
            Directions::Forward(n) => position.increase_x_and_depth(*n),
            Directions::Up(n) => position.decrease_aim(*n),
            Directions::Down(n) => position.increase_aim(*n),
            _ => panic!("I don't know where to go!")
        };
    }
}

fn move_position(position: &mut Position, data: &Vec<Directions>) {
    for command in data {
        match command {
            Directions::Forward(n) => position.increase_x(*n),
            Directions::Up(n) => position.decrease_depth(*n),
            Directions::Down(n) => position.increase_depth(*n),
            _ => panic!("I don't know where to go!")
        };
    }
}


fn read_input() -> Vec<Directions> {
    let input_file = File::open("input.txt").expect("File read error");
    let reader = BufReader::new(input_file);
    let mut data: Vec<Directions> = Vec::new();
    for line in reader.lines() { 
        let operation = line
            .expect("File read error");
        
        let mut operation_iter = operation.split_whitespace();

        let command = operation_iter.next().unwrap();
        let number: isize = operation_iter.next().unwrap().parse().unwrap();

        let direction = match command {
            "forward" => Directions::Forward(number),
            "up" => Directions::Up(number),
            "down" => Directions::Down(number),
            _ => Directions::NoDirection,
        };

        data.push(direction);
    }

    data
}

#[test]
fn test_part_one() {
    let mut position = Position { x: 0, d: 0, a: 0};
    let data = vec![Directions::Forward(1), Directions::Down(2), Directions::Down(1)];
    move_position(&mut position, &data);

    assert_eq!(position.x, 1);
    assert_eq!(position.d, 3);
    assert_eq!(position.x * position.d, 3);

    let more_data = vec![Directions::Forward(5), Directions::Up(2), Directions::Down(1)];
    move_position(&mut position, &more_data);

    assert_eq!(position.x, 6);
    assert_eq!(position.d, 2);
    assert_eq!(position.x * position.d, 12);
}

#[test]
fn test_part_two() {
    let mut position = Position { x: 0, d: 0, a: 0 };
    let data = vec![Directions::Forward(5), Directions::Down(5), Directions::Forward(8)];
    move_position_and_aim(&mut position, &data);

    assert_eq!(position.x, 13);
    assert_eq!(position.a, 5);
    assert_eq!(position.d, 40);
    assert_eq!(position.x * position.d, 520);

    let more_data = vec![Directions::Up(3), Directions::Down(8), Directions::Forward(2)];
    move_position_and_aim(&mut position, &more_data);

    assert_eq!(position.x, 15);
    assert_eq!(position.a, 10);
    assert_eq!(position.d, 60);
    assert_eq!(position.x * position.d, 900);
}