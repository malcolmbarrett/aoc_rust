use std::fs;
mod bingo;
use bingo::card::BingoCard;

fn main() {
    let input = read_input();
    if let Some((first, elements)) = input.split_first() {
        let mut draws = collect_draws(first);
        let mut cards = collect_cards(elements);

        let mut first_winning_card = find_first_winning_card(&mut cards, &mut draws);
        println!("Part 1: {}", first_winning_card.solve());
    };

    if let Some((first, elements)) = input.split_first() {
        let mut draws = collect_draws(first);
        let mut cards = collect_cards(elements);

        let mut last_winning_card = find_last_winning_card(&mut cards, &mut draws);
        println!("Part 2: {}", last_winning_card.solve());
    };
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
    let mut which_card = 0;
    for draw in draws.iter_mut() {
        for (i, card) in cards.iter_mut().enumerate() {
            if !card.is_winner() {
                card.dab(*draw);
                if card.is_winner() {
                    last_winning_card = card.clone();
                    which_card = i;
                }
            }
        }
    }
    println!("The last winning card was index {}", which_card);
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
fn test_bingo_cards() {
    let mut card = BingoCard {
        b: vec![13, 62, 38, 10, 41],
        i: vec![93, 59, 60, 74, 75],
        n: vec![79, 18, 57, 90, 28],
        g: vec![56, 76, 34, 96, 84],
        o: vec![78, 42, 69, 14, 19],
        dabbed: Vec::new(),
    };

    assert!(!card.check_rows());

    card.dab(13);
    card.dab(62);
    card.dab(38);
    card.dab(10);

    assert!(!card.check_rows());

    card.dab(56);
    card.dab(76);
    card.dab(34);
    card.dab(96);
    card.dab(84);

    assert!(card.check_rows());
    assert!(card.is_winner());
    assert!(!card.check_cols());

    card.dab(13);
    card.dab(93);
    card.dab(79);
    card.dab(56);
    card.dab(78);

    assert!(card.check_cols());
    assert!(card.is_winner());
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
