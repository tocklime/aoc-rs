#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.parse::<i32>().expect("Bad int"))
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().fold(0, |a, b| a + b)
}

use std::collections::HashSet;

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    //Find first number reached twice.
    //set....
    let mut set = HashSet::new();
    let mut freq = 0;
    set.insert(freq);
    for i in input.iter().cycle() {
        freq += i;
        //println!("i: {}, f: {}", i, freq);
        if !set.insert(freq) {
            return freq;
        }
    }
    panic!("Finished infinite loop");
}
