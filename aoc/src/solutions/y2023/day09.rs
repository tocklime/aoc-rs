use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
};
use nom_supreme::{final_parser::final_parser, ParserExt};
use utils::nom::NomError;
use rayon::prelude::*;

aoc_harness::aoc_main!(2023 day 9, both [single_thread, multi_thread] => (1_877_825_184, 1108), example both EG => (114,2));

fn gen(input: &str) -> Result<Vec<Vec<i64>>, NomError> {
    let p = separated_list1(
        newline::<_, NomError>,
        separated_list1(space1, complete::i64),
    )
    .terminated(newline.opt());
    final_parser(p)(input)
}
fn solve_line(mut ns: Vec<i64>) -> (i64, i64) {
    let mut suffix = *ns.last().unwrap();
    let mut prefix = ns[0];
    for first_sign in [-1, 1].into_iter().cycle() {
        let mut all_zero = true;
        for ix in 0..ns.len() - 1 {
            ns[ix] = ns[ix + 1] - ns[ix];
            if ns[ix] != 0 {
                all_zero = false;
            }
        }
        ns.pop();
        suffix += ns[ns.len() - 1];
        prefix += first_sign * ns[0];
        if all_zero {
            break;
        }
    }
    (suffix, prefix)
}

fn single_thread(input: &str) -> (i64, i64) {
    gen(input)
        .expect("parse")
        .into_iter()
        .map(solve_line)
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d))
}

fn multi_thread(input: &str) -> (i64, i64) {
    gen(input)
        .expect("parse")
        .into_par_iter()
        .map(solve_line)
        .reduce(|| (0, 0), |(a, b), (c, d)| (a + c, b + d))
}

const EG: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
