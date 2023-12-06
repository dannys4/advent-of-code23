use std::collections::{VecDeque};
use std::cmp;

#[derive(Copy, Clone)]
struct Card {
    num: usize,
    num_found: usize
}

fn build_card(s: &str)->Card {
    let card_split = s.rsplit_once(": ").unwrap();
    let game_num_str = card_split.0.split_ascii_whitespace().last().unwrap();
    let num = game_num_str.parse::<usize>().unwrap();
    let (winning_str, given_str) = card_split.1.rsplit_once(" | ").unwrap();
    let winning_vec: Vec<u32> = winning_str.split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
    let given_vec: Vec<u32> = given_str.split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
    let mut score = 0;
    for g in &given_vec {
        if winning_vec.contains(g) {
            score += 1;
        }
    }
    return Card {
        num: num,
        num_found: score
    };
}

pub fn day4(contents: &String) {
    let mut cards = Vec::new();
    for line in contents.lines() {
        if line.len() == 0 {
            break;
        }
        cards.push(build_card(line));
    }
    let mut score = 0;
    for card in &cards {
        let mut card_score: u32 = 0;
        if card.num_found > 0 {
            card_score = 1 << (card.num_found - 1);
        }
        score += card_score;
    }
    println!("Part 1: {score}");
    let mut copied = VecDeque::new();
    for card in &cards {
        copied.push_back(card);
    }
    let mut total = copied.len();
    while !copied.is_empty() {
        let card = copied.pop_front().unwrap();
        for j in (card.num)..(card.num+card.num_found) {
            copied.push_back(cards.get(j).unwrap());
            total += 1;
        }
    }
    println!("Part 2: {total}");
}