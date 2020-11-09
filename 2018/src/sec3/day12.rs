
use std::collections::{HashSet, BTreeSet};

fn step(plants: &BTreeSet<i64>, rules : &HashSet<u8>) -> BTreeSet<i64> {
    let min = plants.iter().next().unwrap();
    let max = plants.iter().next_back().unwrap();
    (min - 2..=max + 2).filter_map(|ix| {
        let r : u8 = (0..5).filter_map(|i| if plants.contains(&(ix - 2 + i)) {Some (1<<i)}else{None}).sum();
        if rules.contains(&r) {
            Some(ix)
        } else {
            None
        }
    }).collect()
}

struct Prob {
    initial_plants : BTreeSet<i64>,
    rules : HashSet<u8>
}
#[aoc_generator(day12)]
fn gen (input : &str) -> Prob {
    let init_state = &input.lines().nth(0).unwrap()[15..];
    let initial_plants : BTreeSet<i64> =
        init_state.chars()
            .enumerate()
            .filter_map(|(ix,c)| if c == '#' {Some(ix as i64)} else {None}).collect();
    let rules : HashSet<u8> =
    input.trim().lines().skip(2).filter_map(|l| {
        let x = l.as_bytes();
        let inputs : u8 = (0..5).map(|i| if x[i] ==  b'#' { 1 << i} else {0}).sum();
        let output = x[9] == b'#';
        if output {
            Some(inputs)
        } else { None }
    }).collect();
    Prob {
        initial_plants,
        rules
    }
}

#[aoc(day12, part1)]
fn p1(input: &Prob) -> i64 {
    let mut plants = input.initial_plants.clone();
    (0..20).for_each(|_| {
        plants = step(&plants,&input.rules)
    });
    plants.iter().sum()
}

#[aoc(day12, part2)]
fn p2(input: &Prob) -> i64 {
    let mut plants = input.initial_plants.clone();
    let mut last_diff = 0;
    let mut time = 0;
    loop {
        let next = step(&plants, &input.rules);
        let diff = next.iter().sum::<i64>() - plants.iter().sum::<i64>();
        if diff == last_diff {
            break;
        } else {
            last_diff = diff;
        }
        plants = next;
        time += 1;
    }
    let target_time = 50_000_000_000_i64;
    let curr_total = plants.iter().sum::<i64>();
    let remaining_time = target_time - time;
    curr_total + remaining_time * last_diff
}