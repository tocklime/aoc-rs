use aoc_harness::*;
use utils::numset::NumSet;

aoc_main!(2022 day 6, part1 [solve::<4>] => 1544, part2 [solve::<14>] => 2145, example both EG => (7,19));

const EG: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

fn solve<const WINDOW_SIZE: usize>(input: &str) -> usize {
    let bytes = input.as_bytes();
    for ix in 0..(bytes.len() - WINDOW_SIZE) {
        let mut set = NumSet::<u32>::new();
        for c in &bytes[ix..ix + WINDOW_SIZE] {
            if !set.insert(*c - b'a') {
                break;
            }
        }
        if (set.len() as usize) == WINDOW_SIZE {
            return ix + WINDOW_SIZE;
        }
    }
    0
}
