use std::cmp::Ordering;
use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum HandType { HIGH, PAIR, TWOPAIR, THREE, FULL, FOUR, FIVE}

struct Hand {
    cards:Vec<u8>,
    bid:usize,
    h_type:HandType
}

fn get_cardtype(c: char)->u8 {
    match c{
        '2'=>return 2,
        '3'=>return 3,
        '4'=>return 4,
        '5'=>return 5,
        '6'=>return 6,
        '7'=>return 7,
        '8'=>return 8,
        '9'=>return 9,
        'T'=>return 10,
        'J'=>return 11,
        'Q'=>return 12,
        'K'=>return 13,
        'A'=>return 14,
         _ =>return 0
    }
}

fn get_cardtype_joker(c: char)->u8 {
    match c{
        '2'=>return 2,
        '3'=>return 3,
        '4'=>return 4,
        '5'=>return 5,
        '6'=>return 6,
        '7'=>return 7,
        '8'=>return 8,
        '9'=>return 9,
        'T'=>return 10,
        'J'=>return 1,
        'Q'=>return 12,
        'K'=>return 13,
        'A'=>return 14,
         _ =>return 0
    }
}

fn find_type(hand: &Vec<u8>)->HandType {
    let mut hand_copy = hand.clone();
    hand_copy.sort_unstable();
    let mut group_hand: Vec<_> = hand_copy.iter()
        .group_by(|&x| {return *x;})
        .into_iter()
        .map(|(key, group)| {return (key, group.count())})
        .collect();
    group_hand.sort_unstable_by(|x,y| {
        if x.1 != y.1 {
            return y.1.cmp(&x.1);
        }
        return y.0.cmp(&x.0);
    });
    let top = group_hand.get(0).unwrap().1;
    if top == 5 {
        return HandType::FIVE;
    }
    if top == 4 {
        return HandType::FOUR;
    }
    let mut nxt = 0;
    if group_hand.len() > 1 {
        nxt = group_hand.get(1).unwrap().1;
    }
    if top == 3 {
        if nxt == 2 {
            return HandType::FULL;
        }
        return HandType::THREE;
    }
    if top == 2 {
        if nxt == 2 {
            return HandType::TWOPAIR;
        }
        return HandType::PAIR;
    }
    return HandType::HIGH;
}

fn find_type_joker(hand: &Vec<u8>)->HandType {
    let mut hand_copy = hand.clone();
    hand_copy.sort_unstable();
    if *hand_copy.get(0).unwrap() > 1 {
        return find_type(hand);
    }
    let mut num_joker = 1;
    while num_joker < 5 && hand_copy[num_joker] == 1 {
        num_joker += 1;
    }
    if num_joker == 5 {
        return HandType::FIVE;
    }
    if num_joker == 4 {
        return HandType::FIVE;
    }
    let slice = hand_copy[num_joker..5].to_vec();
    let subtype = find_type(&slice);
    if num_joker == 3 {
        if subtype == HandType::PAIR {
            return HandType::FIVE;
        }
        return HandType::FOUR;
    }
    if num_joker == 2 {
        if subtype == HandType::THREE {
            return HandType::FIVE;
        }
        if subtype == HandType::PAIR {
            return HandType::FOUR;
        }
        return HandType::THREE;
    }
    if num_joker == 1 {
        if subtype == HandType::FOUR {
            return HandType::FIVE;
        }
        if subtype == HandType::THREE {
            return HandType::FOUR;
        }
        if subtype == HandType::TWOPAIR {
            return HandType::FULL;
        }
        if subtype == HandType::PAIR {
            return HandType::THREE;
        }
    }
    return HandType::PAIR;
}

fn build_hand(hand: &str, bid: usize, joker:bool)->Hand {
    let mut getter: &dyn Fn(char) -> u8 = &get_cardtype;
    let mut typer: &dyn Fn(&Vec<u8>) -> HandType = &find_type;
    if joker {
        getter = &get_cardtype_joker;
        typer = &find_type_joker;
    }
    let cards: Vec<_>= hand.chars().map(getter).collect();
    let hand_type = typer(&cards);
    if joker {
        // println!("{}, {}, {:?}", hand, bid, hand_type);
    }
    return Hand {cards: cards, bid: bid, h_type:hand_type};
}

fn sort_hands(hands: &mut Vec<Hand>) {
    hands.sort_unstable_by(|x,y| {
        let (x_h, y_h) = (x.h_type as u8, y.h_type as u8);
        if x_h != y_h {
            return x_h.cmp(&y_h);
        }
        for (x_c, y_c) in x.cards.iter().zip(y.cards.iter()) {
            if *x_c != *y_c {
                return x_c.cmp(&y_c);
            }
        }
        return Ordering::Equal;
    });
}

pub fn day7(contents: &String) {
    let mut hands = Vec::new();
    let mut hands_joker = Vec::new();
    for line in contents.lines() {
        if line.len() <= 0 {
            break;
        }
        let (hand_str, bid_str) = line.rsplit_once(' ').unwrap();
        let bid = bid_str.parse().unwrap();
        let hand = build_hand(hand_str, bid, false);
        let hand_joker = build_hand(hand_str, bid, true);
        hands.push(hand);
        hands_joker.push(hand_joker);
    }
    sort_hands(&mut hands);
    sort_hands(&mut hands_joker);
    let mut sum = 0;
    let mut sum2 = 0;
    for j in 0..hands.len() {
        let hand = &hands[j];
        let hand2 = &hands_joker[j];
        // println!("{j}, {:?}, {}", hand2.h_type, hand2.bid);
        sum += (j+1)*hand.bid;
        sum2 += (j+1)*hand2.bid;
    }
    println!("Part 1: {sum}");
    println!("Part 2: {sum2}");
}