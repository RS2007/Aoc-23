use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

#[derive(Debug)]
struct Turn {
    hand: String,
    bid: u64,
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn parse_input(input: &str) -> Vec<Turn> {
    input
        .lines()
        .map(|line| {
            let (hand, bid_string) = line.split_once(" ").unwrap();
            let bid = bid_string.parse::<u64>().unwrap();
            Turn {
                hand: hand.to_string(),
                bid,
            }
        })
        .collect::<Vec<_>>()
}

fn find_hand_type(hand: &str) -> HandType {
    let set_from_hand = hand.chars().collect::<BTreeSet<char>>();
    match set_from_hand.len() {
        1 => HandType::FiveOfKind,
        2 => {
            let map = hand.chars().fold(BTreeMap::new(), |mut acc, card| {
                if acc.contains_key(&card) {
                    let value = acc.get_mut(&card).unwrap();
                    *value += 1;
                    return acc;
                }
                acc.insert(card, 1);
                acc
            });
            if map.values().filter(|val| **val == 4).count() == 0 {
                return HandType::FullHouse;
            }
            HandType::FourOfKind
        }
        3 => {
            let map = hand.chars().fold(BTreeMap::new(), |mut acc, card| {
                if acc.contains_key(&card) {
                    let value = acc.get_mut(&card).unwrap();
                    *value += 1;
                    return acc;
                }
                acc.insert(card, 1);
                acc
            });
            if map.values().filter(|val| **val == 3).count() == 0 {
                return HandType::TwoPair;
            }
            HandType::ThreeOfKind
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => {
            assert!(false, "Unreachable");
            HandType::OnePair
        }
    }
}

fn part1(input: &str) -> u64 {
    const POKER_ORDER: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    let mut parsed = parse_input(input);
    parsed.sort_by(|a, b| {
        let f_hand_type = find_hand_type(&a.hand);
        let s_hand_type = find_hand_type(&b.hand);
        match f_hand_type.cmp(&s_hand_type) {
            std::cmp::Ordering::Equal => {
                let (non_match_1, non_match2) = a
                    .hand
                    .chars()
                    .zip(b.hand.chars())
                    .find(|(val1, val2)| val1 != val2)
                    .unwrap();
                let pos1 = POKER_ORDER.iter().position(|&x| x == non_match_1).unwrap();
                let pos2 = POKER_ORDER.iter().position(|&x| x == non_match2).unwrap();
                pos2.cmp(&pos1)
            }
            _ => s_hand_type.cmp(&f_hand_type),
        }
    });

    parsed
        .into_iter()
        .enumerate()
        .fold(0, |acc, (indx, turn)| acc + (indx as u64 + 1) * turn.bid)
}

fn modified_hand_type_with_joker(hand: &str) -> HandType {
    let j_count = hand
        .chars()
        .fold(0, |acc, x| if x == 'J' { acc + 1 } else { acc });
    let hand_type = find_hand_type(hand);
    match hand_type {
        HandType::FiveOfKind => hand_type,
        HandType::FourOfKind => match j_count {
            4 | 1 => HandType::FiveOfKind,
            0 => HandType::FourOfKind,
            _ => {
                assert!(false, "Unreachable");
                HandType::FourOfKind
            }
        },
        HandType::FullHouse => match j_count {
            3 | 2 => HandType::FiveOfKind,
            _ => HandType::FullHouse,
        },
        HandType::ThreeOfKind => match j_count {
            3 | 1 => HandType::FourOfKind,
            0 => HandType::ThreeOfKind,
            _ => {
                assert!(false, "Unreachable");
                HandType::ThreeOfKind
            }
        },
        HandType::TwoPair => match j_count {
            2 => HandType::FourOfKind,
            1 => HandType::FullHouse,
            0 => HandType::TwoPair,
            _ => {
                assert!(false, "Unreachable");
                HandType::TwoPair
            }
        },
        HandType::OnePair => match j_count {
            2 | 1 => HandType::ThreeOfKind,
            0 => HandType::OnePair,
            _ => {
                assert!(false, "Unreachable");
                HandType::TwoPair
            }
        },
        HandType::HighCard => match j_count {
            1 => HandType::OnePair,
            0 => HandType::HighCard,
            _ => {
                assert!(false, "Unreachable");
                HandType::HighCard
            }
        },
    }
}

fn part2(input: &str) -> u64 {
    const POKER_ORDER: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];
    let mut parsed = parse_input(input);
    parsed.sort_by(|a, b| {
        let f_hand_type = modified_hand_type_with_joker(&a.hand);
        let s_hand_type = modified_hand_type_with_joker(&b.hand);
        match f_hand_type.cmp(&s_hand_type) {
            std::cmp::Ordering::Equal => {
                let (non_match_1, non_match2) = a
                    .hand
                    .chars()
                    .zip(b.hand.chars())
                    .find(|(val1, val2)| val1 != val2)
                    .unwrap();
                let pos1 = POKER_ORDER.iter().position(|&x| x == non_match_1).unwrap();
                let pos2 = POKER_ORDER.iter().position(|&x| x == non_match2).unwrap();
                pos2.cmp(&pos1)
            }
            _ => s_hand_type.cmp(&f_hand_type),
        }
    });

    parsed
        .into_iter()
        .enumerate()
        .fold(0, |acc, (indx, turn)| acc + (indx as u64 + 1) * turn.bid)
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Cannot read file");
    println!("part1: {:?}", part1(&input));
    println!("part2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_dev_part1() {
        let input = fs::read_to_string("./input.dev.txt").expect("Expected file");
        assert_eq!(part1(&input), 6440);
    }

    #[test]
    fn test_dev_part2() {
        let input = fs::read_to_string("./input.dev.txt").expect("Expected file");
        assert_eq!(part2(&input), 5905);
    }
}
