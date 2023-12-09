use itertools::Itertools;
use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    Parser,
};
use nom_supreme::ParserExt;
use utils::nom::IResult;

aoc_harness::aoc_main!(2023 day 9, both [both] => (1_877_825_184, 1108), example both EG => (114,2));

fn parse_line(input: &str) -> IResult<Vec<i64>> {
    separated_list1(space1, complete::i64)
        .all_consuming()
        .complete()
        .parse(input)
}
fn both(input: &str) -> (i64, i64) {
    input
        .lines()
        .map(|l| {
            let (_, ns) = parse_line(l).expect("parse");
            let mut grid = vec![ns];
            while grid.last().unwrap().iter().any(|&x| x != 0) {
                let mut i = grid.last().unwrap().iter();
                let first = *i.next().unwrap();
                let diffs = i
                    .scan(first, |state, n| {
                        let diff = *n - *state;
                        *state = *n;
                        Some(diff)
                    })
                    .collect_vec();
                grid.push(diffs);
            }
            let (start,end) = grid.iter().rev().fold((0,0), |(start, end), x| (x.first().unwrap() - start, x.last().unwrap() + end));
            (end,start)
        })
        .fold((0,0), |(a,b),(c,d)| (a+c,b+d))
}

const EG: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
