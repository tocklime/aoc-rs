use itertools::Itertools;
use nom::{
    Parser, bytes::complete::tag, character::complete::{self, alphanumeric1, newline}, combinator::{all_consuming, opt}, multi::separated_list1, sequence::{separated_pair, terminated}
};
use utils::nom::IResult;

aoc_harness::aoc_main!(2023 day 7, part1 [solve::<false>] => 251_545_216, part2 [solve::<true>] => 250_384_185, example both EG => (6440,5905));

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct Hand {
    hand_type: HandType,
    cards: Vec<usize>,
    bid: u32,
}

impl Hand {
    fn new(cards: Vec<usize>, bid: u32) -> Self {
        Self {
            hand_type: Self::detect_type(&cards),
            cards,
            bid,
        }
    }
    fn parse<const J_IS_JOKER: bool>(input: &str) -> IResult<'_, Self> {
        let (input, (cards, bid)) = separated_pair(alphanumeric1, tag(" "), complete::u32).parse(input)?;
        Ok((
            input,
            Self::new(cards.chars().map(card_rank::<J_IS_JOKER>).collect(), bid),
        ))
    }
    fn detect_type(cards: &[usize]) -> HandType {
        let mut counts = [0; card_rank::<false>('A') + 1];
        for c in cards {
            counts[*c] += 1;
        }
        let joker_count = counts[0];
        let mut scores: Vec<usize> = counts[1..].iter().sorted().rev().take(2).copied().collect();
        scores[0] += joker_count;
        match (scores.first(), scores.get(1)) {
            (Some(5), _) => HandType::FiveOfAKind,
            (Some(4), _) => HandType::FourOfAKind,
            (Some(3), Some(2)) => HandType::FullHouse,
            (Some(3), _) => HandType::ThreeOfAKind,
            (Some(2), Some(2)) => HandType::TwoPair,
            (Some(2), _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

const fn card_rank<const J_IS_JOKER: bool>(card: char) -> usize {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => {
            if J_IS_JOKER {
                0
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unknown card"),
    }
}

fn solve<const J_IS_JOKER: bool>(input: &str) -> u32 {
    let mut hands = all_consuming(
        terminated(separated_list1(newline, Hand::parse::<J_IS_JOKER>),opt(newline)),
    ).parse(input)
    .unwrap().1;
    hands.sort();
    hands
        .into_iter()
        .zip(1..)
        .map(|(hand, rank)| rank * hand.bid)
        .sum()
}

const EG: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
