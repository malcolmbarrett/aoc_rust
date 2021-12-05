use std::fs;
mod bingo;
use bingo::card::BingoCard;

fn main() {
    let input = read_input();
    let mut first_winning_card = find_winning_card(&input, "first");
    println!("Part 1: {}", first_winning_card.solve());
    
    let mut last_winning_card = find_winning_card(&input, "last");
    println!("Part 2: {}", last_winning_card.solve());
}

fn find_winning_card(input: &Vec<Vec<String>>, search: &str) -> BingoCard {
    let mut winning_card = BingoCard::empty();

    if let Some((first, elements)) = input.split_first() {
        let mut draws = collect_draws(first);
        let mut cards = collect_cards(elements);

        if search == "first" {
            winning_card = find_first_winning_card(&mut cards, &mut draws)
        } else {
            winning_card = find_last_winning_card(&mut cards, &mut draws)
        } 
    };

    winning_card
}

fn find_first_winning_card(cards: &mut Vec<BingoCard>, draws: &mut Vec<i32>) -> BingoCard {
    let mut first_winning_card = BingoCard::empty();
    'check_cards: for draw in draws.iter_mut() {
        for card in cards.iter_mut() {
            card.dab(*draw);
            if card.is_winner() {
                first_winning_card = card.clone();
                break 'check_cards;
            }
        }
    }

    first_winning_card
}

fn find_last_winning_card(cards: &mut Vec<BingoCard>, draws: &mut Vec<i32>) -> BingoCard {
    let mut last_winning_card = BingoCard::empty();
    for draw in draws.iter_mut() {
        for card in cards.iter_mut() {
            if !card.is_winner() {
                card.dab(*draw);
                if card.is_winner() {
                    last_winning_card = card.clone()
                };
            }
        }
    }
    last_winning_card
}

fn collect_draws(first: &Vec<String>) -> Vec<i32> {
    first[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn collect_cards(stringed_cards: &[Vec<String>]) -> Vec<BingoCard> {
    let mut cards: Vec<BingoCard> = Vec::new();
    for stringed_card in stringed_cards {
        let mut rows: Vec<Vec<i32>> = Vec::new();
        for stringed_row in stringed_card {
            let row = stringed_row
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            rows.push(row);
        }
        cards.push(BingoCard::new(rows));
    }

    cards
}

fn read_input() -> Vec<Vec<String>> {
    fs::read_to_string("input.txt")
        .expect("File read error")
        .split("\n\n")
        .map(|s| s.to_string())
        .map(|s| s.split("\n").map(|s| s.to_string()).collect())
        .collect()
}

#[test]
fn test_part_1() {
    let mut draws = BingoCard::test_draws();
    let mut cards = BingoCard::test_cards();

    let mut first_winning_card = find_first_winning_card(&mut cards, &mut draws);
    assert_eq!(first_winning_card.solve(), 4512);
}

#[test]
fn test_part_2() {
    let mut draws = BingoCard::test_draws();
    let mut cards = BingoCard::test_cards();

    let mut last_winning_card = find_last_winning_card(&mut cards, &mut draws);
    assert_eq!(last_winning_card.solve(), 1924);
}
