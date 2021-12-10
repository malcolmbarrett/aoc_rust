use std::fs::read_to_string;

fn main() {
    let input = read_input();
    println!("{:?}", solve_part_one(&input));
    println!("{:?}", solve_part_two(&input));
}

fn solve_part_one(input: &Vec<String>) -> i64 {
    input
        .iter()
        .map(process_corrupted_line)
        .collect::<Vec<i64>>()
        .iter()
        .sum()
}

fn solve_part_two(input: &Vec<String>) -> i64 {
    let mut scores: Vec<i64> = input
        .iter()
        .map(process_incomplete_line)
        .filter(|x| *x != 0)
        .collect();

    scores.sort();

    scores[scores.len() / 2]
}

fn is_opener(delim: &char) -> bool {
    *delim == '(' || *delim == '[' || *delim == '{' || *delim == '<'
}

fn score_corrupted_line(delim: &char) -> i64 {
    match delim {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!()
    }
}


fn close(delim: &char) -> char {
    match delim {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!()
    }
}

fn process_corrupted_line(line: &String) -> i64 {
    let mut stack = Vec::new();
    for delim in line.chars() {
        if is_opener(&delim) {
            stack.push(delim);
        } else {
            if let Some(opener) = stack.pop() {
                if close(&opener) != delim {
                    return score_corrupted_line(&delim)
                }
            }
        }
    }

    0
}

fn process_incomplete_line(line: &String) -> i64 {
    let mut stack = Vec::new();
    for delim in line.chars() {
        if is_opener(&delim) {
            stack.push(delim);
        } else {
            if let Some(opener) = stack.pop() {
                if close(&opener) != delim {
                    return 0
                }
            }
        }
    }

    score_incomplete_line(&stack)
}


fn base_score(delim: &char) -> i64 {
    match delim {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!()
    }
}

fn score_incomplete_line(stack: &Vec<char>) -> i64 {  
    stack
        .iter()
        .rev()
        .fold(0, |input, &delim| (input * 5) + base_score(&delim))
}


fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

#[test]
fn test_part_one() {
    let input = vec![
        "[({(<(())[]>[[{[]{<()<>>".to_string(),
        "[(()[<>])]({[<{<<[]>>(".to_string(),
        "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
        "(((({<>}<{<{<>}{[]{[]{}".to_string(),
        "[[<[([]))<([[{}[[()]]]".to_string(),
        "[{[{({}]{}}([{[{{{}}([]".to_string(),
        "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
        "[<(<(<(<{}))><([]([]()".to_string(),
        "<{([([[(<>()){}]>(<<{{".to_string(),
        "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
    ];

    assert_eq!(solve_part_one(&input), 26397)
}

#[test]
fn test_part_two() {
    let input = vec![
        "[({(<(())[]>[[{[]{<()<>>".to_string(),
        "[(()[<>])]({[<{<<[]>>(".to_string(),
        "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
        "(((({<>}<{<{<>}{[]{[]{}".to_string(),
        "[[<[([]))<([[{}[[()]]]".to_string(),
        "[{[{({}]{}}([{[{{{}}([]".to_string(),
        "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
        "[<(<(<(<{}))><([]([]()".to_string(),
        "<{([([[(<>()){}]>(<<{{".to_string(),
        "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
    ];

    assert_eq!(solve_part_two(&input), 288957)
}
