aoc_harness::aoc_main!(2018 day 14, generator gen, part1 [p1], part2 [p2]);
use std::string::ToString;

use itertools::Itertools;
use utils::nums::digits;

fn gen(input: &str) -> usize {
    input.trim().parse().unwrap()
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn p1(input: &usize) -> String {
    let mut d = vec![3, 7];
    let mut pos = [0, 1];

    while d.len() < *input + 10 {
        let sum = d[pos[0]] + d[pos[1]];
        let mut digs = digits(sum).collect_vec();
        d.append(&mut digs);
        for p in &mut pos {
            *p = (*p + 1 + d[*p]) % d.len();
        }
    }
    d.iter()
        .skip(*input)
        .take(10)
        .map(ToString::to_string)
        .join("")
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn p2(input: &usize) -> usize {
    let mut d = vec![3, 7];
    let mut pos = vec![0, 1];
    let target = digits(*input).collect_vec();
    loop {
        let sum = d[pos[0]] + d[pos[1]];
        for x in digits(sum) {
            d.push(x);
            if d.ends_with(&target) {
                return d.len() - target.len();
            }
        }

        for p in &mut pos {
            *p = (*p + 1 + d[*p]) % d.len();
        }
    }
}
