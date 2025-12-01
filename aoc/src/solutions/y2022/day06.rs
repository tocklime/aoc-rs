
use utils::numset::NumSet;

aoc_harness::aoc_main!(2022 day 6, part1 [solve::<4>, solve_by_strides::<4>] => 1544, part2 [solve::<14>, solve_by_strides::<14>] => 2145, example both EG => (7,19));

const EG: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

fn solve<const WINDOW_SIZE: usize>(input: &str) -> usize {
    let bytes = input.as_bytes();
    (WINDOW_SIZE..bytes.len()).find(|&ix| {
        let mut set = NumSet::<u32>::new();
        bytes[ix-WINDOW_SIZE..ix].iter().all(|c| set.insert(c - b'a'))
    }).unwrap()
}

fn solve_by_strides<const WINDOW_SIZE: usize>(input: &str) -> usize {
    let bytes = input.as_bytes();
    let mut candidate = 0;
    while candidate < bytes.len() {
        let mut set = NumSet::<u32>::new();
        let found_duplicate = bytes[candidate..candidate+WINDOW_SIZE].iter().enumerate().rev().find(|&(_, c)| !set.insert(*c - b'a'));
        if let Some((ix,_)) = found_duplicate {
            candidate += ix + 1; //move the window to just after the found duplicate.
        } else {
            return candidate + WINDOW_SIZE; //easier to reason if candidate is the start of the window, but the solution wants the end of the window.
        }
    }
    0
}
