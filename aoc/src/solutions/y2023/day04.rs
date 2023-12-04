use std::collections::HashSet;

use aoc_harness::aoc_main;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

aoc_main!(2023 day 4, part1 [p1], part2 [p2], example both EG => (13, 30));

fn p1(input: &str) -> u32 {
    let cards = all_consuming(many1(terminated(parse_card, newline)))(input)
        .unwrap()
        .1;
    cards.iter().map(|x| x.score()).sum()
}
fn p2(input: &str) -> u32 {
    let cards = all_consuming(many1(terminated(parse_card, newline)))(input)
        .unwrap()
        .1;
    let win_map : Vec<usize> = cards.iter().map(|x| x.win_count()).collect();
    let mut card_counts = vec![1;cards.len()];
    let mut index = 0;
    while (index < cards.len()) {
        let copies = card_counts[index];
        let win_count = win_map[index];
        for i in (index+1..=index+win_count) {
            card_counts[i]+=copies;
        }
        index += 1;
    }
    card_counts.iter().sum()
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    have: Vec<u32>,
}
impl Card {
    fn win_count(&self) -> usize {
        let win: HashSet<u32> = self.winning.iter().cloned().collect();
        self.have.iter().filter(|x| win.contains(&x)).count()
    }
    fn score(&self) -> u32 {
        let count = self.win_count();
        if count > 0 {
            1 << (count -1)
        } else {
            0
        }
    }
}
fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, id) = preceded(tuple((tag("Card"), many1(tag(" ")))), complete::u32)(input)?;
    let (input, winning) = preceded(tuple((tag(":"),many1(tag(" ")))), separated_list1(many1(tag(" ")), complete::u32))(input)?;
    let (input, have) = preceded(tuple((tag(" |"),many1(tag(" ")))), separated_list1(many1(tag(" ")), complete::u32))(input)?;
    Ok((input, Card { id, winning, have }))
}

const EG: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
