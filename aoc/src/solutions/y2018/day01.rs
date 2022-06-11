
aoc_harness::aoc_main!(2018 day 1, generator input_generator, part1 [part1], part2 [part2]);
fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.parse::<i32>().expect("Bad int"))
        .collect()
}


fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

use std::collections::HashSet;


fn part2(input: &[i32]) -> i32 {
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
