fn gen_fn(input: &str) -> Vec<u32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}
fn part1a(x: &Vec<u32>) -> u32 {
    x.iter().sum()
}
fn part1b(x: &Vec<u32>) -> u32 {
    x.len() as u32
}
fn part2a(_x: &Vec<u32>) -> String {
    "answer".to_string()
}
mod full {
    use super::*;
    use aoc_harness_macros::aoc_main;
    aoc_main!(2021 day 1, generator gen_fn, part1 [part1a, part1b] => 42, part2 [part2a]);
}
mod no_gen {
    use aoc_harness_macros::aoc_main;
    fn p1(input: &str) -> usize {
        input.len()
    }
    fn p2(_input: &str) -> &str {
        "answer"
    }
    aoc_main!(2021 day 2, part1 [p1] => 42, part2 [p2] => "answer");
}
fn main() {
    full::run_main();
    no_gen::run_main();
}

// We want to say something like:
// aoc_main!(2021 day 1, generator ident, [part1a,part1b,part1c] => 42, [part2a,part2b,part2c])
// or
// aoc_main!(2021 day 1, [part1a,part1b,part1c], [part2a,part2b,part2c])
// and have it:
// 1. output a main function which runs all parts
// 2. make tests that foreach partX fn, run it and check that the answer is the given one (if the answers are given)
