#![allow(clippy::maybe_infinite_iter)]

use md5;
use itertools::Itertools;

#[aoc(day5, part1)]
pub fn p1(input: &str) -> String {
    (0..)
        .map(|i| {
            let str = format!("{}{}", input, i);
            md5::compute(&str)
        })
        .filter(|md5|
            md5[0] == 0 && md5[1] == 0 && (md5[2] & 0xF0) == 0
        ).map(|x| format!("{:?}", x).chars().nth(5).unwrap())
        .take(8)
        .collect()
}

#[aoc(day5, part2)]
pub fn p2(input: &str) -> String {
    let matches = (0..)
        .map(|i| {
            let str = format!("{}{}", input, i);
            md5::compute(&str)
        })
        .filter(|md5|
            md5[0] == 0 && md5[1] == 0 && (md5[2] & 0xF0) == 0
        );
    let mut pw = vec!['_';8];
    for x in matches {
        let pos = (x[2] & 0x0F) as usize;
        if pos < pw.len() && pw[pos] == '_' {
            pw[pos] = std::char::from_digit(((x[3] & 0xF0) >> 4) as u32, 16).unwrap();
            if pw.iter().all(|&x| x != '_') {
                break;
            }
        }
    }
    pw.into_iter().collect()
}
