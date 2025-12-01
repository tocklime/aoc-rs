
use nom::{
    bytes::complete::tag, character::complete::{self, space1}, multi::separated_list1
};
use utils::{
    nom::IResult,
    numset::NumSet,
};

aoc_harness::aoc_main!(2023 day 4, generator gen, part1 [p1] => 18619, part2 [p2] => 8_063_216, example both EG => (13, 30));

fn gen(input: &str) -> Vec<Card> {
    use nom::Parser;

    nom::multi::many1(Card::parse)
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
    fn parse(input: &str) -> IResult<'_, Self> {
        use nom::Parser;
        let (input, _id) = (tag("Card "), complete::u32, tag(":"), space1).map(|(_,id,_,_)| id).parse(input)?;
        let (input, winning) =
            separated_list1(space1, complete::u8).parse(input)?;
        let (input, _) = tag(" | ").parse(input)?;
        let (input, have) =
            separated_list1(space1, complete::u8).parse(input)?;
        Ok((input, Self { _id, winning: winning.into_iter().collect(), have: have.into_iter().collect() }))
    }
}

const EG: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
