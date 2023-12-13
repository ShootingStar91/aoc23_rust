#![allow(warnings)]
use std::env;
use std::fmt;
use std::fs;

static example_data: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

fn parse_row(row: &str) -> (Vec<i32>, Vec<i32>) {
    let sides = row.split(":").collect::<Vec<&str>>()[1]
        .split("|")
        .collect::<Vec<&str>>();
    let mut winning: Vec<i32> = Vec::new();
    let mut attempts: Vec<i32> = Vec::new();

    for num in sides[0].split_whitespace().collect::<Vec<&str>>() {
        winning.push(num.parse::<i32>().unwrap())
    }
    for num in sides[1].split_whitespace().collect::<Vec<&str>>() {
        attempts.push(num.parse::<i32>().unwrap())
    }
    (winning, attempts)
}

fn parse(data: &str) -> Vec<(Vec<i32>, Vec<i32>)> {
    let rows = data.split("\n").collect::<Vec<&str>>();
    let mut cards: Vec<(Vec<i32>, Vec<i32>)> = Vec::new();

    for row in rows {
        let (winning, attempts) = parse_row(row);
        cards.push((winning, attempts));
    }
    cards
}

fn get_points(card: (Vec<i32>, Vec<i32>)) -> i32 {
    let mut points = 0;
    let (winning, attempts) = card;
    for attempt in attempts {
        if winning.contains(&attempt) {
            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }
    }
    points
}

fn compute_part_1(data: &str) {
    let cards = parse(data);
    let mut sum = 0;
    for card in cards {
        sum += get_points(card);
    }
    println!("Part 1: {}", sum);
}

fn get_matching_amount(card: &Card) -> i32 {
    let mut amount = 0;
    for attempt in &card.attempts {
        if card.winning.contains(&attempt) {
            amount += 1;
        }
    }
    amount
}

fn count_cards(cards: Vec<Card>) -> i32 {
    let mut amount: i32 = 0;
    for card in cards {
        amount += card.copies;
    }
    amount
}

#[derive(Clone)]
struct Card {
  winning: Vec<i32>,
  attempts: Vec<i32>,
  copies: i32,
}

fn compute_part_2(data: &str) {
    let mut initial_cards = parse(data);
    let mut cards: Vec<Card> = Vec::new();
    for initial_card in initial_cards {
        let card: Card = Card { winning: initial_card.0, attempts: initial_card.1, copies: 1 };
        cards.push(card);
    }
    let mut amount = 0;
    let mut i = 0;
    for i in 0..cards.len() {
      let card = cards.get(i).unwrap();
      let multiplicator = card.copies;
      let matches = get_matching_amount(&card);
      for m in 1..matches+1 {
        let card_to_copy = cards.get_mut(i + m as usize).unwrap();
        card_to_copy.copies += 1 * multiplicator;
      }
    }
    let amount = count_cards(cards);
    println!("Part 2: {}", amount);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() > 1) {
        let file_path = &args[1];
        println!("Reading from file '{}' ...", file_path);
        let data = fs::read_to_string(file_path).expect("Could not read file!\n");
        compute_part_1(&data);
        compute_part_2(&data);
    } else {
        compute_part_1(example_data);
        compute_part_2(example_data);
    }
}
