use nom::{
    Parser,
    bytes::complete::tag,
    character::{complete, one_of},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::separated_pair,
};
use utils::nom::NomError;

aoc_harness::aoc_main!(2025 day 12, part1 [p1] => 526);

#[derive(Debug)]
struct Prob {
    pieces: Vec<u32>,
    spaces: Vec<((u32, u32), Vec<u32>)>,
}
impl Prob {
    fn parser<'a>() -> impl Parser<&'a str, Output = Self, Error = NomError<'a>> {
        all_consuming(
            (
                separated_list1(
                    tag("\n\n"),
                    (
                        complete::u32::<&'a str, NomError>,
                        tag(":\n"),
                        separated_list1(tag("\n"), many1(one_of("#."))),
                    ),
                ),
                tag("\n\n"),
                separated_list1(
                    tag("\n"),
                    (
                        separated_pair(complete::u32, tag("x"), complete::u32),
                        tag(": "),
                        separated_list1(tag(" "), complete::u32),
                    ),
                ),
            )
                .map(|(pieces, _, spaces)| Self {
                    pieces: pieces
                        .into_iter()
                        .map(|(_id, _tag, syms)| {
                            syms.iter().flatten().filter(|&c| c == &'#').count() as u32
                        })
                        .collect(),
                    spaces: spaces
                        .into_iter()
                        .map(|((x, y), _tag, ixs)| ((x, y), ixs))
                        .collect(),
                }),
        )
    }
}

fn p1(input: &str) -> usize {
    let p = Prob::parser().parse(input.trim()).unwrap().1;
    p.spaces
        .iter()
        .filter(|((x, y), ixs)| {
            let total_space = x * y;
            let minimum_space = ixs
                .iter()
                .enumerate()
                .map(|(ix, &count)| p.pieces[ix] * count)
                .sum();
            let max_space = ixs.iter().sum::<u32>() * 9;
            if total_space >= max_space {
                true
            } else if total_space < minimum_space {
                false
            } else {
                panic!("Oh no")
            }
        })
        .count()
}
