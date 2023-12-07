use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

aoc_harness::aoc_main!(2023 day 7, part1 [p1], example part1 EG => 6440);


fn bid(hand: &str) -> u32 {
    let (_, bid) = hand.split_once(' ').unwrap();
    let bid = u32::from_str_radix(bid, 10).unwrap();
    bid
}
#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Clone,Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn detect_type(a: &str) -> HandType {
    let map : counter::Counter<char> = a.chars().collect();
    let scores : Vec<usize> = map.values().sorted().rev().copied().collect();
    if scores.get(0) == Some(&5) {
        HandType::FiveOfAKind
    } else if scores.get(0) == Some(&4){
        HandType::FourOfAKind
    } else if scores.get(0) == Some(&3) {
        if scores.get(1) == Some(&2) {
            HandType::FullHouse
        }else {
            HandType::ThreeOfAKind
        }
    } else if scores.get(0) == Some(&2) {
        if scores.get(1) == Some(&2) {
            HandType::TwoPair
        } else {
            HandType::OnePair
        }
    } else {
        HandType::HighCard
    }
}

fn card_rank(a: char) -> usize {
    match a {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unknown card {a}")
    }

}

fn compare_hands(a: &str, b: &str) -> Ordering {
    let (a, _bid) = a.split_once(' ').unwrap();
    let (b, _bid) = b.split_once(' ').unwrap();
    let at = detect_type(a);
    let bt = detect_type(b);
    let a_ranks = a.chars().map(card_rank).collect_vec();
    let b_ranks = b.chars().map(card_rank).collect_vec();
    at.cmp(&bt).then((a_ranks).cmp(&b_ranks))
}

fn p1(input: &str) -> u32 {
    let hands = input.lines().sorted_by(|&a,&b| compare_hands(a,b)).collect_vec();
    hands.into_iter().zip(1..).map(|(hand, rank)| rank * bid(hand)).sum()
}

const EG: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";