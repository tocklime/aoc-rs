use std::collections::{HashMap, HashSet};

use aoc_harness::*;

aoc_main!(2023 day 3, part1 [p1] => 527364, part2 [p2] => 79026871, example both EG => (4361, 467835));

#[derive(Copy, Clone, Debug)]
enum State {
    Blank,
    InNum(usize),
}
fn p1(input: &str) -> u32 {
    let mut sym_locs = HashSet::new();
    for (row, l) in input.lines().enumerate() {
        for (col, c) in l.char_indices() {
            if c != '.' && !c.is_alphanumeric() {
                sym_locs.insert((row, col));
            }
        }
    }
    let mut part_nums = vec![];
    for (row, l) in input.lines().enumerate() {
        let mut state = State::Blank;
        let mut curr_num = 0;
        for (col, c) in l.char_indices() {
            match (c.is_ascii_digit(), state) {
                (true, State::Blank) => {
                    //just started reading a num
                    state = State::InNum(col);
                    curr_num = c.to_digit(10).unwrap()
                }
                (true, State::InNum(_)) => {
                    curr_num = curr_num * 10 + c.to_digit(10).unwrap();
                }
                (false, State::InNum(x)) => {
                    let mut found_sym = false;
                    for l in row.saturating_sub(1)..=row + 1 {
                        for c in x.saturating_sub(1)..=col {
                            if sym_locs.contains(&(l, c)) {
                                found_sym = true;
                            }
                        }
                    }
                    if found_sym {
                        part_nums.push(curr_num);
                    }
                    curr_num = 0;
                    state = State::Blank
                }
                _ => {}
            }
        }
        if let State::InNum(x) = state {
            let col = l.len();
            let mut found_sym = false;
            for l in row.saturating_sub(1)..=row + 1 {
                for c in x.saturating_sub(1)..=col {
                    if sym_locs.contains(&(l, c)) {
                        found_sym = true;
                    }
                }
            }
            if found_sym {
                part_nums.push(curr_num);
            }
        }
    }
    part_nums.into_iter().sum()
}

const EG: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

fn p2(input: &str) -> u32 {
    let mut sym_locs = HashMap::new();
    for (row, l) in input.lines().enumerate() {
        for (col, c) in l.char_indices() {
            if c == '*' {
                sym_locs.insert((row, col), vec![]);
            }
        }
    }
    for (row, l) in input.lines().enumerate() {
        let mut state = State::Blank;
        let mut curr_num = 0;
        for (col, c) in l.char_indices() {
            match (c.is_ascii_digit(), state) {
                (true, State::Blank) => {
                    //just started reading a num
                    state = State::InNum(col);
                    curr_num = c.to_digit(10).unwrap()
                }
                (true, State::InNum(_)) => {
                    curr_num = curr_num * 10 + c.to_digit(10).unwrap();
                }
                (false, State::InNum(x)) => {
                    let mut found_sym = false;
                    for l in row.saturating_sub(1)..=row + 1 {
                        for c in x.saturating_sub(1)..=col {
                            if let Some(x) = sym_locs.get_mut(&(l, c)) {
                                x.push(curr_num);
                            }
                        }
                    }
                    curr_num = 0;
                    state = State::Blank
                }
                _ => {}
            }
        }
        if let State::InNum(x) = state {
            let col = l.len();
            for l in row.saturating_sub(1)..=row + 1 {
                for c in x.saturating_sub(1)..=col {
                    if let Some(x) = sym_locs.get_mut(&(l, c)) {
                        x.push(curr_num);
                    }
                }
            }
        }
    }
    sym_locs
        .iter()
        .filter_map(|(loc, x)| {
            assert!(x.len() <= 2);
            if x.len() == 2 {
                Some(x.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum()
}
