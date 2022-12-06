use aoc_harness::*;
use utils::numset::NumSet;

aoc_main!(2022 day 6, part1 [solve::<4>] => 1544, part2 [solve::<14>] => 2145, example both EG => (7,19));

const EG: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

fn solve<const WINDOW_SIZE: usize>(input: &str) -> usize {
    let bytes = input.as_bytes();
    (WINDOW_SIZE..bytes.len()).find(|&ix| {
        let mut set = NumSet::<u32>::new();
        bytes[ix-WINDOW_SIZE..ix].iter().all(|c| set.insert(c - b'a'))
    }).unwrap()
}
