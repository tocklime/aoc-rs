use aoc_harness::*;
use itertools::{iterate, unfold};

aoc_main!(2019 day 1, generator input_generator, part1 [part1] => 3154112, part2 [unfolding,iteration] => 4728317,
    example part1 "12" => 2,
    example part1 "14" => 2,
    example part1 "1969" => 654,
    example part1 "100756" => 33583,
    example part2 "14" => 2,
    example part2 "1969" => 966,
    example part2 "100756" => 50346,
);
pub fn rocket_fn(x: i32) -> i32 {
    (x / 3) - 2
}

pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

pub fn part1(input: &[i32]) -> i32 {
    input.iter().cloned().map(rocket_fn).sum()
}

pub fn unfolding(input: &[i32]) -> i32 {
    input
        .iter()
        .flat_map(|x| {
            unfold(*x, |last_mass| match rocket_fn(*last_mass) {
                a if a <= 0 => None,
                a => {
                    *last_mass = a;
                    Some(a)
                }
            })
        })
        .sum()
}
pub fn iteration(input: &[i32]) -> i32 {
    input
        .iter()
        .cloned()
        .flat_map(|x| iterate(x, |&y| rocket_fn(y)).skip(1).take_while(|&x| x > 0))
        .sum()
}
