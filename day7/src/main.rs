use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let mut hands: Vec<(Vec<char>, i32)> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(' ');

        let chars: Vec<char> = parts.next().unwrap().chars().collect();
        let bid = parts.next().unwrap().parse::<i32>().unwrap();

        hands.push((chars, bid));
    }

    println!("part1: {}", part1(hands.clone()));
    println!("part2: {}", part2(hands.clone()));
}

fn part1(hands: Vec<(Vec<char>, i32)>) -> i32 {
    let ranks = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    let mut sorted = hands.clone();

    sorted.sort_by(|a, b| compare(a.0.clone(), b.0.clone(), ranks));
    sorted.reverse();

    return sorted
        .iter()
        .enumerate()
        .fold(0, |acc, v| acc + (v.0 as i32 + 1) * v.1 .1);
}

fn part2(hands: Vec<(Vec<char>, i32)>) -> i32 {
    let ranks = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    let mut best_cards: Vec<(Vec<char>, Vec<char>, i32)> = Vec::new();

    for hand in hands {
        let mut all: Vec<Vec<char>> = Vec::new();
        populate_all_hands(hand.0.clone(), &mut all);
        all.sort_by(|a, b| compare(a.clone(), b.clone(), ranks));
        best_cards.push((all[0].clone(), hand.0.clone(), hand.1));
    }

    best_cards
        .sort_by(|a, b| compare_v2(a.0.clone(), a.1.clone(), b.0.clone(), b.1.clone(), ranks));
    best_cards.reverse();

    return best_cards
        .iter()
        .enumerate()
        .fold(0, |acc, v| acc + (v.0 as i32 + 1) * v.1 .2);
}

fn compare(a: Vec<char>, b: Vec<char>, ranks: [char; 13]) -> std::cmp::Ordering {
    let ord = rank_hand(a.clone()).cmp(&rank_hand(b.clone()));

    if ord != std::cmp::Ordering::Equal {
        return ord;
    }

    for (first, second) in a.iter().zip(b) {
        let card_ord = rank_card(first, ranks).cmp(&rank_card(&second, ranks));
        if card_ord != std::cmp::Ordering::Equal {
            return card_ord;
        }
    }

    return std::cmp::Ordering::Equal;
}

fn compare_v2(
    a1: Vec<char>,
    a2: Vec<char>,
    b1: Vec<char>,
    b2: Vec<char>,
    ranks: [char; 13],
) -> std::cmp::Ordering {
    let ord = rank_hand(a1.clone()).cmp(&rank_hand(b1.clone()));

    if ord != std::cmp::Ordering::Equal {
        return ord;
    }

    for (first, second) in a2.iter().zip(b2) {
        let card_ord = rank_card(first, ranks).cmp(&rank_card(&second, ranks));
        if card_ord != std::cmp::Ordering::Equal {
            return card_ord;
        }
    }

    return std::cmp::Ordering::Equal;
}

fn rank_card(card: &char, ranks: [char; 13]) -> usize {
    return ranks.iter().position(|v| v == card).unwrap();
}

fn rank_hand(cards: Vec<char>) -> u8 {
    let mut freq: HashMap<char, usize> = HashMap::new();

    for card in cards {
        match freq.get(&card) {
            Some(v) => freq.insert(card, v + 1),
            None => freq.insert(card, 1),
        };
    }

    return match freq.len() {
        1 => 1,
        2 => {
            if freq.values().any(|&v| v == 4) {
                return 2;
            }
            return 3;
        }
        3 => {
            if freq.values().any(|&v| v == 3) {
                return 4;
            }
            return 5;
        }
        4 => 6,
        5 => 7,
        _ => panic!("noop"),
    };
}

fn populate_all_hands(cards: Vec<char>, all_hands: &mut Vec<Vec<char>>) {
    if !cards.contains(&'J') {
        all_hands.push(cards.clone());
        return;
    }

    all_hands.push(cards.clone());

    let index = cards.iter().position(|&c| c == 'J').unwrap();

    for c in ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'] {
        let mut new_cards = cards.clone();
        new_cards[index] = c;
        populate_all_hands(new_cards, all_hands);
    }
}
