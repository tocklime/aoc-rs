use aoc_harness::aoc_main;

aoc_main!(2020 day 15, part1 [p1], part2 [p2]);
use std::convert::TryInto;
use utils::inputs::parse_input_from_str_sep_by;

fn solve(input: &str, turns: u32) -> u32 {
    let input = parse_input_from_str_sep_by::<u32>(input, ",");
    //memory is number to turn of last speaking. Initialise it with input.
    let mut memory = vec![0; turns as usize];
    (0..input.len() - 1).for_each(|ix| {
        memory[input[ix] as usize] = (1 + ix).try_into().unwrap();
    });
    (input.len().try_into().unwrap()..turns).fold(input[input.len() - 1], |last_spoke, t| {
        //get when last_spoke was spoken before that, and insert last_spoke=t into memory.
        match std::mem::replace(&mut memory[last_spoke as usize], t) {
            0 => 0,
            x => t - x,
        }
    })
}

fn p1(input: &str) -> u32 {
    solve(input, 2020)
}

fn p2(input: &str) -> u32 {
    solve(input, 30_000_000)
}
