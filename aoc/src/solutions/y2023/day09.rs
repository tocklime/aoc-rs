use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    Parser,
};
use nom_supreme::ParserExt;
use utils::nom::IResult;

aoc_harness::aoc_main!(2023 day 9, both [both] => (1_877_825_184, 1108), example both EG => (114,2));

fn gen(input: &str) -> Vec<Vec<i64>> {
    let p: IResult<Vec<Vec<i64>>> =
        separated_list1(newline, separated_list1(space1, complete::i64))
            .terminated(newline.opt())
            .all_consuming()
            .complete()
            .parse(input);
    p.expect("parse").1
}
fn solve_line(mut ns: Vec<i64>) -> (i64, i64) {
    let mut suffix = *ns.last().unwrap();
    let mut first = [0, ns[0]];
    for first_ix in [0, 1].into_iter().cycle() {
        let mut all_zero = true;
        for ix in 0..ns.len() - 1 {
            ns[ix] = ns[ix + 1] - ns[ix];
            if ns[ix] != 0 {
                all_zero = false;
            }
        }
        ns.pop();
        suffix += ns[ns.len() - 1];
        first[first_ix] += ns[0];
        if all_zero {
            break;
        }
    }
    let prefix = first[1] - first[0];
    (suffix, prefix)
}
fn both(input: &str) -> (i64, i64) {
    let x = gen(input);
    x.into_iter()
        .map(solve_line)
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d))
}

const EG: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
