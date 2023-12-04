use aoc_harness::aoc_main;
use nom::{
    character::complete::{self, newline, space1},
    combinator::{eof, success},
};
use nom_supreme::{multi::collect_separated_terminated, tag::complete::tag, ParserExt};
use utils::{
    nom::{ws, IResult},
    numset::NumSet,
};

aoc_main!(2023 day 4, generator gen, part1 [p1] => 18619, part2 [p2] => 8_063_216, example both EG => (13, 30));

fn gen(input: &str) -> Vec<Card> {
    use nom::Parser;

    collect_separated_terminated(Card::parse, success(()), eof.opt_preceded_by(newline))
        .parse(input)
        .expect("Parse")
        .1
}
fn p1(cards: &[Card]) -> u32 {
    cards.iter().map(Card::score).sum()
}
fn p2(cards: &[Card]) -> u32 {
    let win_map: Vec<usize> = cards.iter().map(Card::win_count).collect();
    let mut card_counts = vec![1; cards.len()];
    (0..cards.len())
        .map(|index| {
            let copies = card_counts[index];
            let win_count = win_map[index];
            for x in &mut card_counts[index + 1..=index + win_count] {
                *x += copies;
            }
            copies
        })
        .sum()
}

#[derive(Debug)]
struct Card {
    _id: u32,
    winning: NumSet<u128>,
    have: NumSet<u128>,
}
impl Card {
    fn win_count(&self) -> usize {
        (self.winning & self.have).len().try_into().unwrap()
    }
    fn score(&self) -> u32 {
        self.win_count()
            .checked_sub(1)
            .map(|x| 1 << x)
            .unwrap_or_default()
    }
    fn parse(input: &str) -> IResult<Self> {
        use nom::Parser;
        let (input, _id) = complete::u32
            .terminated(tag(":"))
            .terminated(space1)
            .preceded_by(tag("Card").terminated(space1))
            .context("Find ID")
            .parse(input)?;
        let (input, winning) =
            collect_separated_terminated(complete::u8, space1, ws(tag("|").terminated(space1)))
                .parse(input)?;
        let (input, have) =
            nom_supreme::multi::collect_separated_terminated(complete::u8, space1, newline)
                .parse(input)?;
        Ok((input, Self { _id, winning, have }))
    }
}

const EG: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
