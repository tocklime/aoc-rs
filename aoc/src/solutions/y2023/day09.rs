use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
};
use nom_supreme::ParserExt;
use rayon::prelude::*;
use utils::nom::parse_all;

aoc_harness::aoc_main!(2023 day 9, generator gen, both [single_thread, multi_thread] => (1_877_825_184, 1108), example both EG => (114,2));

fn gen(input: &str) -> Vec<Vec<i64>> {
    parse_all(
        input,
        separated_list1(newline, separated_list1(space1, complete::i64)).terminated(newline.opt()),
    )
}
fn solve_line(mut ns: Vec<i64>) -> (i64, i64) {
    let mut suffix = *ns.last().unwrap();
    let mut prefix = ns[0];
    let mut mults = [-1, 1].into_iter().cycle();
    while ns.iter().any(|x| x != &0) {
        for ix in 0..ns.len() - 1 {
            ns[ix] = ns[ix + 1] - ns[ix];
        }
        ns.pop();
        suffix += ns[ns.len() - 1];
        prefix += mults.next().unwrap() * ns[0];
    }
    (suffix, prefix)
}

fn single_thread(input: &[Vec<i64>]) -> (i64, i64) {
    input
        .iter()
        .cloned()
        .map(solve_line)
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

fn multi_thread(input: &[Vec<i64>]) -> (i64, i64) {
    input
        .to_vec()
        .into_par_iter()
        .map(solve_line)
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

const EG: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
